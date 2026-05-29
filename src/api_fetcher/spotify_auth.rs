use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::random;
use sha2::{Digest, Sha256};

pub fn generate_code_verifier() -> String {
    let gen_code: [u8; 64] = random();
    let encoded = URL_SAFE_NO_PAD.encode(&gen_code);
    return encoded;
}

pub fn generate_code_challenge(verifier: &str) -> String {
    let hashed = Sha256::digest(verifier.as_bytes());
    let encoded = URL_SAFE_NO_PAD.encode(&hashed);
    return encoded;
}
