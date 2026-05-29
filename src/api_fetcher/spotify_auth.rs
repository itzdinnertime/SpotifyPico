use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::random;

fn generate_code_verifier() -> String {
    let gen_code: [u8; 64] = random();
    let encoded = URL_SAFE_NO_PAD.encode(&gen_code);
    return encoded;
}
