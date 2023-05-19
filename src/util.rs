use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    process::Output,
};

use camino::Utf8Path;

use rand::{rngs::StdRng, SeedableRng};
use reqwest::Url;

const DEFAULT_SEED: u64 = 123456789;

pub fn create_seeded_rng(seed: Option<&str>) -> StdRng {
    let seed = match seed {
        Some(string) => {
            let mut hasher = DefaultHasher::new();
            string.hash(&mut hasher);
            hasher.finish()
        }
        None => DEFAULT_SEED,
    };
    StdRng::seed_from_u64(seed)
}

pub fn add_api_key(url: &str, api_key: &str) -> String {
    let mut url = Url::parse(url).expect("invalid url");
    url.query_pairs_mut().append_pair("apikey", api_key);
    url.to_string()
}

pub fn expect_file_name(path: &str) -> String {
    Utf8Path::new(path)
        .file_name()
        .expect("path must have a file name here")
        .to_string()
}

pub fn commandline_error<T>(output: Output) -> crate::Result<T> {
    use color_eyre::eyre::eyre;

    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    let stderr = std::str::from_utf8(&output.stderr).unwrap();
    Err(eyre!(
        "ffmpeg failed with exit code {}, stdout:\n{}\nstderr:\n{}",
        output.status.code().unwrap_or(1),
        stdout,
        stderr
    ))
}


#[cfg(test)]
mod test {
    use super::{add_api_key, expect_file_name};

    #[test]
    #[cfg(not(windows))]
    fn test_expect_file_name() {
        let path = "/Users/test/123.txt";
        let file_name = expect_file_name(path);
        assert_eq!("123.txt", file_name);
    }

    #[test]
    #[cfg(windows)]
    fn test_expect_file_name() {
        let path = "C:\\Users\\123.txt";
        let file_name = expect_file_name(path);
        assert_eq!("123.txt", file_name);
    }

    #[test]
    fn test_add_api_key() {
        let result = add_api_key("http://localhost:3001", "super-secret-123");
        assert_eq!(result, "http://localhost:3001/?apikey=super-secret-123");
    }
}
