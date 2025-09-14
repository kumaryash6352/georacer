
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameObjectImage {
    pub image_data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    StartGame,
    SubmitGuess { image_b64: String },
}

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
    pub total_scores: HashMap<Player, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LobbyPhase {
    WaitingForStart,
    Countdown,
    Searching {
        target: GameObject,
        scores: HashMap<Player, f32>,
        zoom_level: f32,
    },
    RoundOver,
    GameOver,
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
#[serde(tag = "type")]
pub enum GameMessage {
    GameState(LobbyState),
    Error(String),
    Countdown { duration: u8 },
    NewRound { target: GameObject },
    UpdateImage { zoom_level: f32 },
    ProximityUpdate { status: ProximityStatus },
    GuessResult { correct: bool },
    RoundOver { scores: HashMap<Player, f32> },
    GameOver { leaderboard: Vec<(Player, f32)> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProximityStatus {
    Hotter,
    Colder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameObject {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<mongodb::bson::oid::ObjectId>,
    pub name: String,
    pub image_b64: String,
    // pub latitude: f64,
    // pub longitude: f64,
}
