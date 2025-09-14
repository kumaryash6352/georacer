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
        self.state.lock().await.players.push(player.clone());
        let (mut sender, mut receiver) = ws.split();

        let self_clone = self.clone();
        tokio::spawn(async move {
            while let Some(Ok(msg)) = receiver.next().await {
                trace!("Received WS message: {:?}", msg);
                if let Message::Text(text) = msg {
                    if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                        trace!("Parsed message: {:?}", client_msg);
                        match client_msg {
                            ClientMessage::StartGame => {
                                self_clone.start_game().await;
                            }
                            ClientMessage::SubmitGuess => {
                                let submission = Submission { player: player.clone() };
                                self_clone.submit_guess(submission).await;
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
            let game_objects = self.db.collection::<GameObject>("gameobjects");
            let pipeline = vec![doc! { "$sample": { "size": 1 } }];
            let mut cursor = game_objects.aggregate(pipeline).await.unwrap();

            if let Some(result) = cursor.next().await {
                if let Ok(target) = bson::from_document::<GameObject>(result.unwrap()) {
                    trace!("Found target object: {:?}", target.name);
                    state.phase = LobbyPhase::Searching {
                        target,
                        scores: HashMap::new(),
                    };

                    let lobbies = self.db.collection::<LobbyState>("lobbies");
                    lobbies
                        .replace_one(doc! { "id": state.id.to_string() }, state.clone())
                        .await
                        .unwrap();
                    
                    self.broadcast_state().await;
                } else {
                    trace!("Failed to deserialize target object from bson.");
                }
            } else {
                trace!("No game objects found in database.");
            }
        }
    }

    pub async fn submit_guess(&self, submission: Submission) {
        let mut state = self.state.lock().await;
        if let LobbyPhase::Searching { target, mut scores } = state.clone().phase {
            if !scores.contains_key(&submission.player) {
                let score = state.players.len() - scores.len() - 1;
                scores.insert(submission.player.clone(), score as f32);
                state.phase = LobbyPhase::Searching {
                    target,
                    scores,
                };
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
