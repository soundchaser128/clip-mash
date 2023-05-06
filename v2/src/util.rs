use futures::{Future, StreamExt};
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

pub async fn parallelize<T, I, F>(futures: I, parallelism: usize) -> Vec<T>
where
    F: Future<Output = T>,
    I: IntoIterator<Item = F>,
{
    use futures::stream;

    let mut stream = stream::iter(futures).buffer_unordered(parallelism);

    let mut results = vec![];
    while let Some(result) = stream.next().await {
        results.push(result)
    }

    results
}
