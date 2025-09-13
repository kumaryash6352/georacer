use crate::models::{GameMessage, LobbyState, Player};
use axum::extract::ws::{Message, WebSocket};
use futures_util::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};

pub struct Lobby {
    state: Arc<Mutex<LobbyState>>,
    tx: broadcast::Sender<GameMessage>,
}

impl Lobby {
    pub fn new(state: LobbyState) -> Self {
        let (tx, _) = broadcast::channel(100);
        Self {
            state: Arc::new(Mutex::new(state)),
            tx,
        }
    }

    pub async fn add_player(&self, player: Player, ws: WebSocket) {
        self.state.lock().await.players.push(player);
        let mut rx = self.tx.subscribe();
        let (mut sender, mut receiver) = ws.split();

        // Handle incoming messages
        tokio::spawn(async move {
            while let Some(Ok(msg)) = receiver.next().await {
                if let Message::Close(_) = msg {
                    break;
                }
            }
        });

        // Broadcast game state updates
        tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
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
}
