use crate::NowPlaying;
use axum::{Json, extract::State};
use std::sync::{Arc, Mutex};

type SharedState = Arc<Mutex<Option<NowPlaying>>>;

pub async fn handler_function(State(state): State<SharedState>) -> Json<Option<NowPlaying>> {
    let track = state.lock().unwrap().clone();
    Json(track)
}
