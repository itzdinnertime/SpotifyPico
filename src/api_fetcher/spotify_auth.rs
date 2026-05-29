use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::random;
use sha2::{Digest, Sha256};

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
