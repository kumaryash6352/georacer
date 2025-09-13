use crate::lobby::Lobby;
use dashmap::DashMap;
use mongodb::Client;
use uuid::Uuid;
use std::sync::Arc;

pub struct AppState {
    pub mdb: Client,
    pub lobbies: DashMap<Uuid, Arc<Lobby>>,
}
