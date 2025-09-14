impl LobbyPhase {
    fn zoom_level(&self) -> Option<f32> {
        match self {
            LobbyPhase::Searching { zoom_level, .. } => Some(*zoom_level),
            _ => None,
        }
    }
}

use mongodb::bson;
use tracing::trace;

use crate::models::{ClientMessage, GameObject, GameMessage, LobbyPhase, LobbyState, Player, ProximityStatus, Submission};
use axum::extract::ws::{Message, WebSocket};
use futures_util::sink::SinkExt;
use mongodb::bson::doc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};

#[derive(Clone)]
pub struct Lobby {
    state: Arc<Mutex<LobbyState>>,
    tx: broadcast::Sender<GameMessage>,
    db: mongodb::Database,
}

use futures_util::stream::StreamExt;

impl Lobby {
    pub fn new(state: LobbyState, db: mongodb::Database) -> Self {
        let (tx, _) = broadcast::channel(100);
        Self {
            state: Arc::new(Mutex::new(state)),
            tx,
            db,
        }
    }

    pub async fn add_player(&self, player: Player, ws: WebSocket) {
        self.state.lock().await.players.push(player.clone());
        let (mut sender, mut receiver) = ws.split();
        let cself = self.clone();

        tokio::spawn(async move {
            while let Some(Ok(msg)) = receiver.next().await {
                trace!("Received WS message: {:?}", &msg.clone().into_text().unwrap()[..40]);
                if let Message::Text(text) = msg {
                    if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                        trace!("Parsed message: {:?}", &format!("{client_msg:?}")[..40]);
                        match client_msg {
                            ClientMessage::StartGame => {
                                cself.start_game().await;
                            }
                            ClientMessage::SubmitGuess { image_b64 } => {
                                let submission = Submission {
                                    player: player.clone(),
                                    image_b64,
                                };
                                cself.submit_guess(submission).await;
                            }
                        }
                    }
                }
            }
        });

        // Broadcast game state updates
        let mut rx = self.tx.subscribe();
        tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                trace!("{msg:?}");
                let json = serde_json::to_string(&msg).unwrap();
                if sender.send(Message::Text(json.into())).await.is_err() {
                    break;
                }
            }
        });

        self.broadcast_state().await;
    }

    async fn broadcast_state(&self) {
        let state = self.state.lock().await.clone();
        self.tx.send(GameMessage::GameState(state)).unwrap();
    }

    pub async fn start_game(&self) {
        let mut state = self.state.lock().await;
        if let LobbyPhase::WaitingForStart = state.phase {
            state.phase = LobbyPhase::Countdown;
            
            let lobby_id = state.id.to_string();
            drop(state);

            self.tx.send(GameMessage::Countdown { duration: 3 }).unwrap();
            self.broadcast_state().await;

            let self_clone = self.clone();
            tokio::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;

                let mut state = self_clone.state.lock().await;
                if let LobbyPhase::Countdown = state.phase {
                    let game_objects = self_clone.db.collection::<GameObject>("gameobjects");
                    let pipeline = vec![doc! { "$sample": { "size": 1 } }];
                    let mut cursor = game_objects.aggregate(pipeline).await.unwrap();

                    if let Some(result) = cursor.next().await {
                        if let Ok(target) = bson::from_document::<GameObject>(result.unwrap()) {
                            trace!("Found target object: {:?}", target.name);
                            state.phase = LobbyPhase::Searching {
                                target: target.clone(),
                                scores: HashMap::new(),
                                zoom_level: 1.0,
                            };
                            
                            self_clone.tx.send(GameMessage::NewRound { target: target.clone() }).unwrap();

                            let zoom_task_self_clone = self_clone.clone();
                            tokio::spawn(async move {
                                let mut nzoom_level = 1.0;
                                loop {
                                    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                                    nzoom_level -= 0.1;

                                    if nzoom_level < 0.1 {
                                        break;
                                    }

                                    let mut state = zoom_task_self_clone.state.lock().await;
                                    if let LobbyPhase::Searching { ref mut zoom_level, .. } = state.phase {
                                        *zoom_level = nzoom_level;
                                        zoom_task_self_clone.tx.send(GameMessage::UpdateImage { zoom_level: *zoom_level }).unwrap();

                                        let lobbies = zoom_task_self_clone.db.collection::<LobbyState>("lobbies");
                                        lobbies
                                            .replace_one(doc! { "id": &state.id.to_string() }, state.clone())
                                            .await
                                            .unwrap();

                                    } else {
                                        break;
                                    }
                                }
                            });
                        } else {
                            trace!("Failed to deserialize target object from bson.");
                            state.phase = LobbyPhase::WaitingForStart;
                        }
                    } else {
                        trace!("No game objects found in database.");
                        state.phase = LobbyPhase::WaitingForStart;
                    }
                    
                    let lobbies = self_clone.db.collection::<LobbyState>("lobbies");
                    lobbies
                        .replace_one(doc! { "id": &lobby_id }, state.clone())
                        .await
                        .unwrap();
                    
                    drop(state);

                    self_clone.broadcast_state().await;
                }
            });
        }
    }

    pub async fn submit_guess(&self, submission: Submission) {
        let mut state = self.state.lock().await;
        if let LobbyPhase::Searching { target, mut scores, .. } = state.clone().phase {
            if !scores.contains_key(&submission.player) {
                let correct = crate::gemini::is_same_image(&target.image_b64, &submission.image_b64)
                    .await
                    .unwrap_or(false);

                self.tx
                    .send(GameMessage::GuessResult { correct })
                    .unwrap();

                if correct {
                    let score = state.players.len() - scores.len() - 1;
                    scores.insert(submission.player.clone(), score as f32);

                    let total_score = state.total_scores.entry(submission.player.clone()).or_insert(0.0);
                    *total_score += score as f32;

                    if *total_score >= state.settings.points_to_win {
                        state.phase = LobbyPhase::GameOver;
                        let mut leaderboard: Vec<(Player, f32)> = state.total_scores.clone().into_iter().collect();
                        leaderboard.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

                        self.tx.send(GameMessage::GameOver { leaderboard }).unwrap();
                    } else if scores.len() >= state.settings.scorers_per_target {
                        state.phase = LobbyPhase::RoundOver;
                        self.tx.send(GameMessage::RoundOver { scores: scores.clone() }).unwrap();

                        let self_clone = self.clone();
                        tokio::spawn(async move {
                            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                            self_clone.start_new_round().await;
                        });

                    } else {
                        state.phase = LobbyPhase::Searching {
                            target,
                            scores,
                            zoom_level: state.phase.zoom_level().unwrap_or(1.0),
                        };
                    }
                    
                    let lobbies = self.db.collection::<LobbyState>("lobbies");
                    lobbies
                        .replace_one(doc! { "id": state.id.to_string() }, state.clone())
                        .await
                        .unwrap();
                    self.broadcast_state().await;
                }
            }
        }
    }

    async fn start_new_round(&self) {
        let mut state = self.state.lock().await;
        let game_objects = self.db.collection::<GameObject>("gameobjects");
        let pipeline = vec![doc! { "$sample": { "size": 1 } }];
        let mut cursor = game_objects.aggregate(pipeline).await.unwrap();

        if let Some(result) = cursor.next().await {
            if let Ok(target) = bson::from_document::<GameObject>(result.unwrap()) {
                trace!("Found target object: {:?}", target.name);
                state.phase = LobbyPhase::Searching {
                    target: target.clone(),
                    scores: HashMap::new(),
                    zoom_level: 1.0,
                };

                self.tx.send(GameMessage::NewRound { target }).unwrap();
            } else {
                trace!("Failed to deserialize target object from bson.");
                state.phase = LobbyPhase::WaitingForStart;
            }
        } else {
            trace!("No game objects found in database.");
            state.phase = LobbyPhase::WaitingForStart;
        }

        let lobbies = self.db.collection::<LobbyState>("lobbies");
        lobbies
            .replace_one(doc! { "id": state.id.to_string() }, state.clone())
            .await
            .unwrap();
        
        self.broadcast_state().await;
    }
}
