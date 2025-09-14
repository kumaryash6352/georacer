use futures_util::StreamExt;
use mongodb::bson::{self, doc};
use mongodb::Database;
use tokio::sync::{broadcast, RwLock};

use crate::models::{GameObject, ServerMessage};

#[derive(Clone)]
pub struct Feed {
    tx: broadcast::Sender<ServerMessage>,
    db: Database,
    current: std::sync::Arc<RwLock<Option<GameObject>>>,
}

impl Feed {
    pub fn new(db: Database) -> Self {
        let (tx, _) = broadcast::channel(100);
        Self { tx, db, current: std::sync::Arc::new(RwLock::new(None)) }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ServerMessage> {
        self.tx.subscribe()
    }

    pub async fn current(&self) -> Option<GameObject> {
        self.current.read().await.clone()
    }

    pub fn spawn_loop(self: &std::sync::Arc<Self>, period_secs: u64) {
        let slf = std::sync::Arc::clone(self);
        tokio::spawn(async move {
            loop {
                // Pick a random target from MongoDB
                let coll = slf.db.collection::<GameObject>("gameobjects");
                let pipeline = vec![doc! { "$sample": { "size": 1 } }];

                match coll.aggregate(pipeline).await {
                    Ok(mut cursor) => {
                        if let Some(result) = cursor.next().await {
                            if let Ok(doc) = result {
                                match bson::from_document::<GameObject>(doc) {
                                    Ok(target) => {
                                        {
                                            let mut w = slf.current.write().await;
                                            *w = Some(target.clone());
                                        }
                                        tracing::info!(
                                            target_name = %target.name,
                                            receivers = slf.tx.receiver_count(),
                                            "Selected new target for broadcast"
                                        );
                                        let _ = slf.tx.send(ServerMessage::Guess { target });
                                    }
                                    Err(e) => {
                                        tracing::error!("Failed to deserialize GameObject: {:?}", e);
                                    }
                                }
                            }
                        } else {
                            tracing::warn!("No gameobjects found to sample from");
                        }
                    }
                    Err(e) => tracing::error!("Mongo aggregate error: {:?}", e),
                }

                tokio::time::sleep(std::time::Duration::from_secs(period_secs)).await;
            }
        });
    }
}
