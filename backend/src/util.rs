use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::process::Output;

use camino::Utf8Path;
use lazy_static::lazy_static;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{thread_rng, SeedableRng};
use reqwest::Url;
use tracing::{debug, Level};

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

pub fn commandline_error<T>(command_name: &str, output: Output) -> crate::Result<T> {
    use color_eyre::eyre::eyre;

    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    let stderr = std::str::from_utf8(&output.stderr).unwrap();
    Err(eyre!(
        "command {} failed with exit code {}, stdout:\n'{}'\nstderr:\n'{}'",
        command_name,
        output.status.code().unwrap_or(1),
        stdout,
        stderr
    ))
}

pub fn debug_output(output: Output) {
    if tracing::enabled!(Level::DEBUG) {
        let stdout = std::str::from_utf8(&output.stdout).unwrap();
        let stderr = std::str::from_utf8(&output.stderr).unwrap();

        debug!("stdout = '{}'", stdout);
        debug!("stderr = '{}'", stderr);
    }
}

pub fn generate_id() -> String {
    const ADJECTIVES: &str = include_str!("../data/adjectives.txt");
    const ANIMALS: &str = include_str!("../data/animals.txt");

    lazy_static! {
        static ref ADJECTIVE_LIST: Vec<&'static str> =
            ADJECTIVES.split('\n').map(|n| n.trim()).collect();
        static ref ANIMALS_LIST: Vec<&'static str> =
            ANIMALS.split('\n').map(|n| n.trim()).collect();
    }
    let mut rng = thread_rng();
    let adjective1 = ADJECTIVE_LIST.choose(&mut rng).unwrap();
    let adjective2 = ADJECTIVE_LIST.choose(&mut rng).unwrap();
    let animal = ANIMALS_LIST.choose(&mut rng).unwrap();

    format!("{adjective1}-{adjective2}-{animal}")
}

/// Formats a number of seconds as a mm:ss string
pub fn format_duration(seconds: f64) -> String {
    let minutes = (seconds / 60.0).floor();
    let seconds = (seconds % 60.0).floor();
    format!("{:02}:{:02}", minutes, seconds)
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

#[cfg(test)]
mod test {
    use regex::Regex;

    use super::{add_api_key, expect_file_name, format_duration, generate_id};

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

    #[test]
    fn test_generate_id() {
        let id = generate_id();
        let regex = Regex::new("[a-z]+-[a-z]+-[a-z]+").unwrap();
        assert!(regex.is_match(&id));
    }

    #[test]
    fn test_format_duration() {
        let formatted = format_duration(123.0);
        assert_eq!("02:03", formatted);

        let formatted = format_duration(123.456);
        assert_eq!("02:03", formatted);
    }
}
