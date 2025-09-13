use anyhow::Context;
use axum::{extract::ws::WebSocket, routing::get, Router};
use dashmap::DashMap;
use mongodb::{bson::oid::ObjectId, Client};
use tokio::{net::TcpListener, sync::mpsc::{Sender, Receiver}};

use std::{collections::HashMap, env::var, error::Error};

type LobbyId = usize;

pub struct State {
    mdb: Client,
    lobbies: DashMap<LobbyId, Sender<WebSocket>>
}

pub struct LobbyState {
    players: Vec<Player>,
    new_players: Receiver<WebSocket>,
    settings: LobbySettings,
    phase: LobbyPhase
}

pub enum LobbyPhase {
    WaitingForStart,
    Countdown,
    Searching {
        target: SearchTarget,
        scores: HashMap<Player, f32>,
        players_already_found: Vec<Player>
    },
}

pub struct LobbySettings {
    points_to_win: f32,
    scorers_per_target: usize
}

pub struct SearchTarget {
    img_b64: String,
    obj_name: String,
    oid: ObjectId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Player {
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let mdb = Client::with_uri_str(var("MONGO_DB_CONNECT").expect("need MONGO_DB_CONNECT!")).await.context("connecting to mongodb")?;

    let router = Router::new().route("/", get(async || ""));

    axum::serve(TcpListener::bind("0.0.0.0:3000").await.context("binding to network")?, router).await?;

    Ok(())
}
