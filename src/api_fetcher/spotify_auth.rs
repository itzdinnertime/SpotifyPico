use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::random;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::{Arc, Mutex};

use axum::extract::{Query, State};

/// Generates a random code verifier.
pub fn generate_code_verifier() -> String {
    let gen_code: [u8; 64] = random();
    let encoded = URL_SAFE_NO_PAD.encode(&gen_code);
    return encoded;
}

/// Generates a code challenge from a code verifier.
pub fn generate_code_challenge(verifier: &str) -> String {
    let hashed = Sha256::digest(verifier.as_bytes());
    let encoded = URL_SAFE_NO_PAD.encode(&hashed);
    return encoded;
}

/// Builds the Spotify authorization URL.
pub fn build_auth_url(client_id: &str, code_challenge: &str) -> String {
    let url = format!(
        "https://accounts.spotify.com/authorize?client_id={}&code_challenge={}&response_type=code&redirect_uri={}&code_challenge_method=S256&scope=user-read-currently-playing",
        client_id, code_challenge, "http://localhost:8888/callback"
    );
    return url;
}

/// Opens the url
pub fn open_auth_url(url: &str) {
    if let Err(e) = open::that(url) {
        eprintln!("Failed to open browser: {}", e);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallbackParams {
    code: String,
}

pub async fn handler_function(
    State(tx): State<Arc<Mutex<Option<tokio::sync::oneshot::Sender<String>>>>>,
    Query(params): Query<CallbackParams>,
) -> &'static str {
    if let Some(sender) = tx.lock().unwrap().take() {
        sender.send(params.code).unwrap();
    }

    "Login successful, you can close this tab"
}
pub async fn start_callback_server() -> Result<String, Box<dyn std::error::Error>> {
    let (tx, rx) = tokio::sync::oneshot::channel::<String>();
    let tx = Arc::new(Mutex::new(Some(tx)));
    let app = axum::Router::new()
        .route("/callback", axum::routing::get(handler_function))
        .with_state(tx);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8888").await?;
    axum::serve(listener, app).await?;

    let code = rx.await?;
    return Ok(code);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenResponse {
    access_token: String,
    refresh_token: String,
}

pub async fn exchange_code_for_token(
    client: &reqwest::Client,
    code: &str,
    client_id: &str,
    code_verifier: &str,
) -> Result<TokenResponse, Box<dyn std::error::Error>> {
    let response = client
        .post("https://accounts.spotify.com/api/token")
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", "http://localhost:8888/callback"),
            ("client_id", client_id),
            ("code_verifier", code_verifier),
        ])
        .send()
        .await?;

    let token_response: TokenResponse = response.json().await?;
    Ok(token_response)
}

pub async fn authenticate(
    client: &reqwest::Client,
    client_id: &str,
) -> Result<TokenResponse, Box<dyn std::error::Error>> {
    let verifier = generate_code_verifier();
    let challenge = generate_code_challenge(&verifier);
    let url = build_auth_url(client_id, &challenge);
    open_auth_url(&url);
    let code = start_callback_server().await?;
    exchange_code_for_token(client, &code, client_id, &verifier).await
}
