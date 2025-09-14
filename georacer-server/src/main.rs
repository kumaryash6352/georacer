use crate::handlers::{register_object, ws_handler, add_image_to_gameobject, submit_guess};
use crate::state::AppState;
use anyhow::Context;
use axum::{routing::{get, post}, Router, response::IntoResponse};
use dotenvy::var;
use mongodb::Client;
use tower_http::cors::{Any, CorsLayer};
use std::{error::Error, sync::Arc};
use tokio::net::TcpListener;

pub mod gemini;
pub mod models;
pub mod state;
pub mod handlers;
pub mod feed;

async fn fallback() -> impl IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, "Invalid route")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().with_env_filter("georacer_server=trace").init();

    tracing::info!("starting georacer-server");

    let mdb = Client::with_uri_str(var("MONGO_DB_CONNECT").expect("need MONGO_DB_CONNECT!")).await.context("connecting to mongodb")?;
    let db = mdb.database(&var("MONGO_DB_NAME").expect("need MONGO_DB_NAME!"));

    // Global feed that pushes a new guess every 20s
    let feed = std::sync::Arc::new(crate::feed::Feed::new(db));
    feed.spawn_loop(20);

    let state = Arc::new(AppState {
        mdb,
        feed,
    });

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let router = Router::new()
        .route("/", get(async || "georacer-server running"))
        .route("/ws", get(ws_handler))
        .route("/register", post(register_object))
        .route("/gameobject/image", post(add_image_to_gameobject))
        .route("/guess", post(submit_guess))
        .fallback(fallback)
        .with_state(state)
        .layer(cors);

    let listener = TcpListener::bind("127.0.0.1:3000").await.context("binding to network")?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, router).await?;

    Ok(())
}
