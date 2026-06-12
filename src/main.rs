use spotify_pico::api_fetcher::{spotify_auth::authenticate, spotify_fetch::get_current_playing};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let client_id = std::env::var("SPOTIFY_CLIENT_ID").unwrap();
    let client = reqwest::Client::new();
    let token_response = authenticate(&client, &client_id).await.unwrap();
    loop {
        // 1. check if token is expired and refresh if needed
        if token_issued.elapsed().as_secs() >= token_response.expires_in {
            token_response =
                refresh_access_token(&client, &token_response.refresh_token, &client_id)
                    .await
                    .unwrap();
        }

        // 2. fetch current track
        match get_current_playing(&client, &token_response.access_token).await {
            Ok(Some(track)) => println!("Playing: {:?}", track),
            Ok(None) => println!("Nothing playing"),
            Err(e) => println!("Error: {}", e),
        }

        // 3. wait 5 seconds
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
    match get_current_playing(&client, &token_response.access_token).await {
        Ok(Some(track)) => println!("Playing: {:?}", track),
        Ok(None) => println!("Nothing playing"),
        Err(e) => println!("Error: {}", e),
    }
}
