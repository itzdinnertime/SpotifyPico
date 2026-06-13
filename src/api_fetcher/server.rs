use crate::NowPlaying;
use axum::{Json, extract::State};
use std::sync::{Arc, Mutex};

type SharedState = Arc<Mutex<Option<NowPlaying>>>;

// Handles incoming GET requests to the "/now-playing" endpoint.
pub async fn handler_function(State(state): State<SharedState>) -> Json<Option<NowPlaying>> {
    let track = state.lock().unwrap().clone();
    Json(track)
}

// Starts the server and listens for incoming requests.
pub async fn start_server(state: SharedState) -> Result<(), Box<dyn std::error::Error>> {
    let app = axum::Router::new()
        .route("/now-playing", axum::routing::get(handler_function))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
