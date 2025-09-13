use crate::handlers::{create_lobby, join_lobby, register_object, start_game, submit_picture, ws_handler};
use crate::state::AppState;
use anyhow::Context;
use axum::{routing::{get, post}, Router};
use dashmap::DashMap;
use dotenvy::var;
use mongodb::Client;
use reqwest::Method;
use tower_http::cors::{Any, CorsLayer};
use std::{error::Error, sync::Arc};
use tokio::net::TcpListener;

pub mod lobby;
pub mod gemini;
pub mod models;
pub mod state;
pub mod handlers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().with_env_filter("trace").init();

    tracing::info!("starting georacer-server");

    let mdb = Client::with_uri_str(var("MONGO_DB_CONNECT").expect("need MONGO_DB_CONNECT!")).await.context("connecting to mongodb")?;

    let state = Arc::new(AppState {
        mdb,
        lobbies: DashMap::new(),
    });

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let router = Router::new()
        .route("/", get(async || "wrong url!"))
        .route("/lobby", post(create_lobby))
        .route("/lobby/{id}/join", post(join_lobby))
        .route("/lobby/{id}/start", post(start_game))
        .route("/ws/{id}", get(ws_handler))
        .route("/register", post(register_object))
        .route("/lobby/{id}/submit", post(submit_picture))
        .with_state(state)
        .layer(cors);

    let listener = TcpListener::bind("0.0.0.0:3000").await.context("binding to network")?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, router).await?;

    Ok(())
}
