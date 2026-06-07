use spotify_pico::api_fetcher::{spotify_auth::authenticate, spotify_fetch::get_current_playing};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let client_id = std::env::var("SPOTIFY_CLIENT_ID").unwrap();
    let client = reqwest::Client::new();
    let token_response = authenticate(&client, &client_id).await.unwrap();
    let token_issued = std::time::Instant::now();
    if token_issued.elapsed().as_secs() >= token_response.expires_in {
        refresh_access_token();
    }
    match get_current_playing(&client, &token_response.access_token).await {
        Ok(Some(track)) => println!("Playing: {:?}", track),
        Ok(None) => println!("Nothing playing"),
        Err(e) => println!("Error: {}", e),
    }
}
