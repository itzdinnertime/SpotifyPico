use spotify_pico::api_fetcher::spotify_fetch::get_current_playing;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    match get_current_playing(&client, "").await {
        Ok(Some(track)) => println!("Playing: {:?}", track),
        Ok(None) => println!("Nothing playing"),
        Err(e) => println!("Error: {}", e),
    }
}
