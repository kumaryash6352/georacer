pub async fn join_lobby(
    State(state): State<Arc<AppState>>,
    Path(lobby_id): Path<Uuid>,
    Json(player): Json<Player>,
) -> Json<String> {
    let db = state
        .mdb
        .database(&var("MONGO_DB_NAME").expect("need MONGO_DB_NAME!"));
    let lobbies = db.collection::<LobbyState>("lobbies");

    if let Some(mut lobby) = lobbies
        .find_one(doc! { "id": lobby_id.to_string() })
        .await
        .unwrap()
    {
        lobby.players.push(player);
        lobbies
            .replace_one(doc! { "id": lobby_id.to_string() }, lobby)
            .await
            .unwrap();
        Json("Joined lobby".to_string())
    } else {
        Json("Lobby not found".to_string())
    }
}

use crate::lobby::Lobby;
use crate::models::{LobbyPhase, LobbySettings, LobbyState, Player};
use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, State, ws::WebSocketUpgrade},
    response::IntoResponse,
};
use dotenvy::var;
use mongodb::bson::doc;
use serde_json::{json, Value};
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;

pub async fn create_lobby(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<(LobbySettings, Player)>,
) -> Json<Value> {
    let (settings, player) = payload;
    let lobby_id = Uuid::new_v4();
    let lobby_state = LobbyState {
        id: lobby_id,
        players: vec![player.clone()],
        settings,
        phase: LobbyPhase::WaitingForStart,
    };

    let lobby = Lobby::new(lobby_state.clone(), state.mdb.database(&var("MONGO_DB_NAME").expect("need MONGO_DB_NAME!")));
    state.lobbies.insert(lobby_id, Arc::new(lobby));

    let db = state
        .mdb
        .database(&var("MONGO_DB_NAME").expect("need MONGO_DB_NAME!"));
    db.collection::<LobbyState>("lobbies")
        .insert_one(lobby_state)
        .await
        .unwrap();

    Json(json!({ "id": lobby_id }))
}




pub async fn register_object(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<super::models::GameObject>,
) -> Json<String> {
    let db = state
        .mdb
        .database(&var("MONGO_DB_NAME").expect("need MONGO_DB_NAME!"));
    let game_objects = db.collection::<super::models::GameObject>("gameobjects");

    game_objects.insert_one(payload).await.unwrap();

    Json("Object registered".to_string())
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    Path(lobby_id): Path<Uuid>,
    axum::extract::Query(params): axum::extract::Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let player_name = params.get("player_name").cloned().unwrap_or_default();
    let player = Player {
        name: player_name,
    };

    if let Some(lobby) = state.lobbies.get(&lobby_id) {
        let lobby = lobby.clone();
        ws.on_upgrade(move |socket| {
            tokio::spawn(async move {
                lobby.add_player(player, socket).await;
            });
            async {}
        })
    } else {
        ws.on_upgrade(|_| async {})
    }
}
