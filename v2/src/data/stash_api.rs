use reqwest::Client;

pub struct StashApi {
    api_url: String,
    api_key: String,
    client: Client,
}

impl StashApi {
    pub fn new(api_url: &str, api_key: &str) -> Self {
        StashApi {
            api_url: api_url.into(),
            api_key: api_key.into(),
            client: Client::new(),
        }
    }

    
}
