
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameObjectImage {
    pub image_data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameObject {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<mongodb::bson::oid::ObjectId>,
    pub name: String,
    pub image_b64: String,
}

// Simplified protocol: clients connect and receive periodic Guess messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    Guess { target: GameObject },
    Error(String),
}

// For compatibility with any existing client code that might send messages,
// keep a minimal ClientMessage (can be expanded later if needed)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    Ping,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
}
