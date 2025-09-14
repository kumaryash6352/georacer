
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

use crate::models::{ClientMessage, GameObject, GameMessage, LobbyPhase, LobbyState, Player, Submission};
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
        // Deduplicate by player name to avoid duplicates from double WS init (e.g., React StrictMode)
        {
            let mut state = self.state.lock().await;
            state.players.retain(|p| p.name != player.name);
            state.players.push(player.clone());
        }

        let (mut sender, mut receiver) = ws.split();
        let cself = self.clone();
        let player_for_cleanup = player.clone();

        tokio::spawn(async move {
            while let Some(Ok(msg)) = receiver.next().await {
                //trace!("Received WS message: {:?}", &msg.clone().into_text().unwrap());
                if let Message::Text(text) = msg {
                    if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                        //trace!("Parsed message: {:?}", &format!("{client_msg:?}")[..40]);
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
            // Receiver ended => WS closed. Remove the player and broadcast.
            {
                let mut state = cself.state.lock().await;
                state.players.retain(|p| p.name != player_for_cleanup.name);
            }
            cself.broadcast_state().await;
        });

        // Broadcast game state updates
        let mut rx = self.tx.subscribe();
        tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                dbg!(&msg);
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
        // Best-effort broadcast; do not panic if there are no subscribers
        let _ = self.tx.send(GameMessage::GameState(state));
    }

    /// Spawn the continuous round loop with fixed duration per round
    pub fn spawn_round_loop(self: &Arc<Self>, round_secs: u64) {
        let slf = Arc::clone(self);
        tokio::spawn(async move {
            loop {
                // Pick a random target
                let game_objects = slf.db.collection::<GameObject>("gameobjects");
                let pipeline = vec![doc! { "$sample": { "size": 1 } }];
                let mut cursor = game_objects.aggregate(pipeline).await.unwrap();

                if let Some(result) = cursor.next().await {
                    if let Ok(doc) = result {
                        match bson::from_document::<GameObject>(doc) {
                            Ok(target) => {
                                // Start new round
                                {
                                    let mut state = slf.state.lock().await;
                                    state.phase = LobbyPhase::Searching {
                                        target: target.clone(),
                                        scores: HashMap::new(),
                                        zoom_level: 1.0,
                                    };
                                }
                                let _ = slf.tx.send(GameMessage::NewRound { target: target.clone() });
                                slf.broadcast_state().await;

                                // Tick every second
                                let mut seconds_left = round_secs;
                                while seconds_left > 0 {
                                    {
                                        let state = slf.state.lock().await;
                                        let (submitted, active) = match &state.phase {
                                            LobbyPhase::Searching { scores, .. } => (scores.len(), state.players.len()),
                                            _ => (0, state.players.len()),
                                        };
                                        let _ = slf.tx.send(GameMessage::Tick { seconds_left: seconds_left as u8, submitted, active });
                                    }
                                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                                    seconds_left -= 1;
                                }

                                // End of round: emit RoundOver with the scores
                                let scores_snapshot = {
                                    let state = slf.state.lock().await;
                                    match &state.phase {
                                        LobbyPhase::Searching { scores, .. } => scores.clone(),
                                        _ => HashMap::new(),
                                    }
                                };
                                let _ = slf.tx.send(GameMessage::RoundOver { scores: scores_snapshot });

                                // Continue immediately to next round
                                continue;
                            }
                            Err(e) => {
                                tracing::error!("Failed to deserialize target object: {:?}", e);
                            }
                        }
                    }
                }

                // If no object found, wait briefly before retrying
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        });
    }

    pub async fn start_game(&self) {
        let mut state = self.state.lock().await;
        if let LobbyPhase::WaitingForStart = state.phase {
            state.phase = LobbyPhase::Countdown;
            
            let lobby_id = state.id.to_string();
            drop(state);

            // Emit countdown (best-effort)
            let _ = self.tx.send(GameMessage::Countdown { duration: 3 });
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
                        match result {
                            Ok(doc) => {
                                match bson::from_document::<GameObject>(doc) {
                                    Ok(target) => {
                                        trace!("Found target object: {:?}", target.name);
                                        state.phase = LobbyPhase::Searching {
                                            target: target.clone(),
                                            scores: HashMap::new(),
                                            zoom_level: 1.0,
                                        };

                                        tracing::info!("Emitting NewRound (initial) for lobby {}", state.id);
                                        let _ = self_clone.tx.send(GameMessage::NewRound { target: target.clone() });

                                        // Start periodic zoom update task for this round
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
                                                    let _ = zoom_task_self_clone.tx.send(GameMessage::UpdateImage { zoom_level: *zoom_level });

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
                                    }
                                    Err(e) => {
                                        tracing::error!("Failed to deserialize target object from bson: {:?}", e);
                                        state.phase = LobbyPhase::WaitingForStart;
                                    }
                                }
                            }
                            Err(e) => {
                                tracing::error!("Mongo cursor error during initial target fetch: {:?}", e);
                                state.phase = LobbyPhase::WaitingForStart;
                            }
                        }
                    } else {
                        trace!("No game objects found in database.");
                        state.phase = LobbyPhase::WaitingForStart;
                    }

                    // Persist updated lobby state and broadcast
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
            if !scores.contains_key(&submission.player.name) {
                let correct = crate::gemini::is_same_image(&target.image_b64, &submission.image_b64)
                    .await
                    .unwrap();

                let _ = self.tx
                    .send(GameMessage::GuessResult { correct });

                if correct {
                    // Points equal to active players who have not yet submitted
                    let score = state.players.len() - scores.len() - 1;
                    scores.insert(submission.player.name.clone(), score as f32);

                    let total_score = state.total_scores.entry(submission.player.name.clone()).or_insert(0.0);
                    *total_score += score as f32;

                    // Stay in Searching phase until round timer ends
                    state.phase = LobbyPhase::Searching {
                        target,
                        scores,
                        zoom_level: state.phase.zoom_level().unwrap_or(1.0),
                    };

                    // Broadcast updated state for leaderboard
                    self.broadcast_state().await;
                }
            }
        }
    }

    async fn start_new_round(&self) {
        tracing::info!("Starting new round - selecting new target");
        let mut state = self.state.lock().await;
        let game_objects = self.db.collection::<GameObject>("gameobjects");
        let pipeline = vec![doc! { "$sample": { "size": 1 } }];
        let mut cursor = game_objects.aggregate(pipeline).await.unwrap();

        if let Some(result) = cursor.next().await {
            match result {
                Ok(doc) => {
                    match bson::from_document::<GameObject>(doc) {
                        Ok(target) => {
                            trace!("Found target object: {:?}", target.name);
                            state.phase = LobbyPhase::Searching {
                                target: target.clone(),
                                scores: HashMap::new(),
                                zoom_level: 1.0,
                            };

                            tracing::info!("Emitting NewRound (next) for lobby {}", state.id);
                            let _ = self.tx.send(GameMessage::NewRound { target });
                        }
                        Err(e) => {
                            tracing::error!("Failed to deserialize target object from bson: {:?}", e);
                            state.phase = LobbyPhase::WaitingForStart;
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Mongo cursor error during next target fetch: {:?}", e);
                    state.phase = LobbyPhase::WaitingForStart;
                }
            }
        } else {
            // Fallback: no objects in DB. Use a transparent 1x1 PNG so the loop progresses.
            let fallback = GameObject {
                id: None,
                name: "Sample".to_string(),
                image_b64: "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO2nY0kAAAAASUVORK5CYII=".to_string(),
            };
            trace!("No game objects found; using fallback");
            state.phase = LobbyPhase::Searching {
                target: fallback.clone(),
                scores: HashMap::new(),
                zoom_level: 1.0,
            };
            let _ = self.tx.send(GameMessage::NewRound { target: fallback });
        }

        let lobbies = self.db.collection::<LobbyState>("lobbies");
        lobbies
            .replace_one(doc! { "id": state.id.to_string() }, state.clone())
            .await
            .unwrap();

        self.broadcast_state().await;
    }
}
