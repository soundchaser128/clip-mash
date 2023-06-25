use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::process::Output;
use std::time::{Duration, Instant};

use camino::Utf8Path;
use clip_mash_types::Progress;
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

pub struct ProgressTracker {
    work_done: u64,
    started_at: Instant,
    work_total: u64,
}

impl ProgressTracker {
    pub fn new(work_todo: u64) -> Self {
        ProgressTracker {
            work_done: 0,
            started_at: Instant::now(),
            work_total: work_todo,
        }
    }

    pub fn inc_work_done(&mut self) {
        self.work_done = self.work_done + 1;
    }

    /// Increment work done by a given amonut.
    pub fn inc_work_done_by(&mut self, units: u64) {
        self.work_done = self.work_done + units;
    }

    pub fn eta(&self) -> Duration {
        let work_not_done = self
            .work_total
            .checked_sub(self.work_done)
            .unwrap_or(self.work_total);
        let not_done_to_done_ratio = work_not_done as f64 / self.work_done as f64;
        let seconds_since_start = Instant::now() - self.started_at;
        let eta_seconds = not_done_to_done_ratio * seconds_since_start.as_secs() as f64;

        Duration::from_secs(eta_seconds as u64)
    }

    pub fn progress(&self) -> Progress {
        Progress {
            items_finished: self.work_done,
            items_total: self.work_total,
            eta_seconds: self.eta().as_secs_f64(),
            done: self.work_done == self.work_total,
        }
    }
}

#[cfg(test)]
mod test {
    use regex::Regex;

    use super::{add_api_key, expect_file_name, generate_id};

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
}
