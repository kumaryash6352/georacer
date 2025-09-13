use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type LobbyId = usize;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyState {
    pub id: Uuid,
    pub players: Vec<Player>,
    pub settings: LobbySettings,
    pub phase: LobbyPhase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LobbyPhase {
    WaitingForStart,
    Countdown,
    Searching {
        target: GameObject,
        scores: HashMap<Player, f32>,
        players_already_found: Vec<Player>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbySettings {
    pub points_to_win: f32,
    pub scorers_per_target: usize,
}


#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Submission {
    pub player: Player,
    pub image_b64: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameMessage {
    GameState(LobbyState),
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameObject {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<mongodb::bson::oid::ObjectId>,
    pub name: String,
    pub image_url: String,
    pub gps_coordinates: (f64, f64),
}
