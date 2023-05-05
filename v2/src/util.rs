use rand::{rngs::StdRng, SeedableRng};
use reqwest::Url;

pub fn create_seeded_rng() -> StdRng {
    StdRng::seed_from_u64(123456789)
}

pub fn add_api_key(url: &str, api_key: &str) -> String {
    let mut url = Url::parse(url).expect("invalid url");
    url.query_pairs_mut().append_pair("apikey", api_key);
    url.to_string()
}
