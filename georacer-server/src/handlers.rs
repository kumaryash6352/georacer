#[derive(Deserialize)]
pub struct NewObject {
    name: String,
    image: String,
}

use crate::models::{Player, ServerMessage};
use crate::state::AppState;
use axum::{
    Json,
    extract::{State, ws::{WebSocketUpgrade, Message}},
    response::IntoResponse,
};
use dotenvy::var;
use mongodb::bson::doc;
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};
use futures_util::{StreamExt, SinkExt};

#[derive(Deserialize)]
pub struct GuessPayload { pub image_b64: String }

#[derive(serde::Serialize)]
pub struct GuessResponse { pub correct: bool }

pub async fn add_image_to_gameobject(
    State(state): State<Arc<AppState>>,
    Json(obj): Json<NewObject>,
) -> Json<String> {
    let db = state
        .mdb
        .database(&var("MONGO_DB_NAME").expect("need MONGO_DB_NAME!"));
    let game_objects = db.collection::<super::models::GameObject>("gameobjects");

    let new_image = super::models::GameObject {
        id: None,
        image_b64: obj.image,
        name: obj.name,
    };
    game_objects.insert_one(&new_image).await.unwrap();

    Json("Image registered".to_string())
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

pub async fn submit_guess(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<GuessPayload>,
) -> Json<GuessResponse> {
    if let Some(current) = state.feed.current().await {
        // Compare using Gemini
        match crate::gemini::is_same_image(&payload.image_b64, &current.image_b64).await {
            Ok(correct) => Json(GuessResponse { correct }),
            Err(e) => {
                tracing::error!("gemini compare error: {:?}", e);
                Json(GuessResponse { correct: false })
            }
        }
    } else {
        Json(GuessResponse { correct: false })
    }
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    axum::extract::Query(params): axum::extract::Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let player_name = params.get("player_name").cloned().unwrap_or_default();
    let _player = Player { name: player_name };

    let feed = Arc::clone(&state.feed);
    ws.on_upgrade(move |socket| async move {
        let (mut sender, mut receiver) = socket.split();

        // Ignore any client messages from this socket to keep the server simple
        tokio::spawn(async move {
            while let Some(Ok(_msg)) = receiver.next().await {
                // no-op
            }
        });

        // Broadcast subscription: forward ServerMessage to this socket
        let mut rx = feed.subscribe();
        tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                if let Ok(json) = serde_json::to_string(&msg) {
                    if sender.send(Message::Text(json.into())).await.is_err() {
                        break;
                    }
                }
            }
        });
    })
}
