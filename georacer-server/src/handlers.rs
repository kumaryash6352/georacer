use crate::gemini::is_same_image;
use crate::lobby::Lobby;
use crate::models::{LobbyPhase, LobbySettings, LobbyState, Player};
use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, State, ws::WebSocketUpgrade},
    response::IntoResponse,
};
use dotenvy::var;
use futures_util::StreamExt;
use mongodb::bson::{self, doc};
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;

pub async fn create_lobby(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LobbySettings>,
) -> Json<String> {
    let lobby_id = Uuid::new_v4();
    let lobby_state = LobbyState {
        id: lobby_id,
        players: Vec::new(),
        settings: payload,
        phase: LobbyPhase::WaitingForStart,
    };

    let lobby = Lobby::new(lobby_state.clone());
    state.lobbies.insert(lobby_id, Arc::new(lobby));

    let db = state
        .mdb
        .database(&var("MONGO_DB_NAME").expect("need MONGO_DB_NAME!"));
    db.collection::<LobbyState>("lobbies")
        .insert_one(lobby_state)
        .await
        .unwrap();

    Json(lobby_id.to_string())
}

pub async fn join_lobby(
    State(state): State<Arc<AppState>>,
    Path(lobby_id): Path<u32>,
    Json(player): Json<Player>,
) -> Json<String> {
    let db = state
        .mdb
        .database(&var("MONGO_DB_NAME").expect("need MONGO_DB_NAME!"));
    let lobbies = db.collection::<LobbyState>("lobbies");

    if let Some(mut lobby) = lobbies.find_one(doc! { "id": lobby_id }).await.unwrap() {
        lobby.players.push(player);
        lobbies
            .replace_one(doc! { "id": lobby_id }, lobby)
            .await
            .unwrap();
        Json("Joined lobby".to_string())
    } else {
        Json("Lobby not found".to_string())
    }
}

pub async fn start_game(
    State(state): State<Arc<AppState>>,
    Path(lobby_id): Path<u32>,
) -> Json<String> {
    let db = state
        .mdb
        .database(&var("MONGO_DB_NAME").expect("need MONGO_DB_NAME!"));
    let lobbies = db.collection::<LobbyState>("lobbies");

    if let Some(mut lobby) = lobbies.find_one(doc! { "id": lobby_id }).await.unwrap() {
        let game_objects = db.collection::<super::models::GameObject>("gameobjects");
        let pipeline = vec![doc! { "$sample": { "size": 1 } }];
        let mut cursor = game_objects.aggregate(pipeline).await.unwrap();

        if let Some(result) = cursor.next().await {
            match bson::from_document(result.unwrap()) {
                Ok(target) => {
                    lobby.phase = LobbyPhase::Searching {
                        target,
                        scores: HashMap::new(),
                        players_already_found: Vec::new(),
                    };
                    lobbies
                        .replace_one(doc! { "id": lobby_id }, lobby)
                        .await
                        .unwrap();
                    Json("Game started".to_string())
                }
                Err(_) => Json("Failed to deserialize target".to_string()),
            }
        } else {
            Json("No game objects found".to_string())
        }
    } else {
        Json("Lobby not found".to_string())
    }
}

pub async fn submit_picture(
    State(state): State<Arc<AppState>>,
    Path(lobby_id): Path<u32>,
    Json(submission): Json<super::models::Submission>,
) -> Json<String> {
    let db = state
        .mdb
        .database(&var("MONGO_DB_NAME").expect("need MONGO_DB_NAME!"));
    let lobbies = db.collection::<LobbyState>("lobbies");

    if let Some(mut lobby) = lobbies.find_one(doc! { "id": lobby_id }).await.unwrap() {
        if let LobbyPhase::Searching {
            target,
            mut scores,
            mut players_already_found,
        } = lobby.phase
        {
            if players_already_found.contains(&submission.player) {
                return Json("You have already found this object".to_string());
            }

            match is_same_image(&submission.image_b64, &target.image_url).await {
                Ok(true) => {
                    let score = lobby.players.len() - players_already_found.len() - 1;
                    scores.insert(submission.player.clone(), score as f32);
                    players_already_found.push(submission.player.clone());

                    lobby.phase = LobbyPhase::Searching {
                        target,
                        scores,
                        players_already_found,
                    };

                    lobbies
                        .replace_one(doc! { "id": lobby_id }, lobby)
                        .await
                        .unwrap();

                    Json("Correct!".to_string())
                }
                Ok(false) => Json("Incorrect, try again.".to_string()),
                Err(_) => Json("Failed to compare images".to_string()),
            }
        } else {
            Json("Game not in searching phase".to_string())
        }
    } else {
        Json("Lobby not found".to_string())
    }
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
    let player = Player { name: player_name };

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
