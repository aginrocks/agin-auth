use rand::{Rng, distr::Alphanumeric, rngs::ThreadRng};

pub fn generate_client_id() -> String {
    let rng = ThreadRng::default();

    rng.sample_iter(&Alphanumeric)
        .take(24)
        .map(char::from)
        .collect()
}
