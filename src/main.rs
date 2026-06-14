use spotify_pico::NowPlaying;
use spotify_pico::api_fetcher::{
    server::start_server, spotify_auth::authenticate, spotify_auth::refresh_access_token,
    spotify_fetch::get_current_playing,
};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let client_id = std::env::var("SPOTIFY_CLIENT_ID").unwrap();
    let client = reqwest::Client::new();
    let mut token_response = authenticate(&client, &client_id).await.unwrap();
    let mut token_issued = std::time::Instant::now();
    let state = Arc::new(Mutex::new(None::<NowPlaying>));
    let server_state = state.clone();

    tokio::spawn(async move {
        start_server(server_state).await.unwrap();
    });

    loop {
        // Check if token is expired and refresh if needed
        if token_issued.elapsed().as_secs() >= token_response.expires_in {
            token_response =
                refresh_access_token(&client, &token_response.refresh_token, &client_id)
                    .await
                    .unwrap();
            token_issued = std::time::Instant::now();
        }

        // Fetch current track
        match get_current_playing(&client, &token_response.access_token).await {
            Ok(Some(track)) => {
                *state.lock().unwrap() = Some(track.clone());
                println!("Playing: {:?}", track);
            }
            Ok(None) => println!("Nothing playing"),
            Err(e) => println!("Error: {}", e),
        }

        // Wait 5 seconds
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}
