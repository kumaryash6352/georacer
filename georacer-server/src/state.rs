use crate::feed::Feed;
use mongodb::Client;
use std::sync::Arc;

pub struct AppState {
    pub mdb: Client,
    pub feed: Arc<Feed>,
}
