use base64::{Engine, engine::general_purpose};
use rand::RngCore;

pub fn generate_client_id() -> String {
    let mut rng = rand::rngs::ThreadRng::default();

    let mut bytes = [0u8; 24];
    rng.fill_bytes(&mut bytes);

    general_purpose::STANDARD.encode(bytes)
}
