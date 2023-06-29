use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::process::Output;
use std::time::Duration;
#[cfg(not(test))]
use std::time::Instant;

use camino::Utf8Path;
use clip_mash_types::Progress;
use float_cmp::approx_eq;
use lazy_static::lazy_static;
#[cfg(test)]
use mock_instant::Instant;
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
    work_total: f64,
    work_done: f64,
    started_at: Instant,
    message: String,
}

impl Default for ProgressTracker {
    fn default() -> Self {
        ProgressTracker {
            work_done: 0.0,
            started_at: Instant::now(),
            work_total: 0.0,
            message: String::new(),
        }
    }
}

impl ProgressTracker {
    #[cfg(test)]
    pub fn new(work_todo: f64) -> Self {
        ProgressTracker {
            work_done: 0.0,
            started_at: Instant::now(),
            work_total: work_todo,
            message: String::new(),
        }
    }

    pub fn reset(&mut self, work_todo: f64) {
        self.work_done = 0.0;
        self.started_at = Instant::now();
        self.work_total = work_todo;
        self.message = String::new();
    }

    /// Increment work done by a given amonut.
    pub fn inc_work_done_by(&mut self, units: f64, message: &str) {
        self.work_done += units;
        self.message = message.into();
    }

    pub fn eta(&self) -> Duration {
        if self.work_done == 0.0 || self.work_total == 0.0 || self.work_total <= self.work_done {
            return Duration::ZERO;
        }
        let work_not_done = self.work_total - self.work_done;
        let not_done_to_done_ratio = work_not_done / self.work_done;
        let seconds_since_start = Instant::now() - self.started_at;
        let eta_seconds = not_done_to_done_ratio * seconds_since_start.as_secs_f64();

        assert!(
            eta_seconds.is_finite(),
            "eta_seconds is NaN or infinite: {}",
            eta_seconds
        );

        Duration::from_secs_f64(eta_seconds)
    }

    pub fn progress(&self) -> Progress {
        Progress {
            items_finished: self.work_done,
            items_total: self.work_total,
            eta_seconds: self.eta().as_secs_f64(),
            done: self.work_total != 0.0
                && approx_eq!(f64, self.work_done, self.work_total, epsilon = 0.01),
            message: self.message.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use float_cmp::assert_approx_eq;
    use mock_instant::MockClock;
    use regex::Regex;

    use super::{add_api_key, expect_file_name, generate_id, ProgressTracker};

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
    fn test_progress_tracker_eta() {
        let mut tracker = ProgressTracker::new(100.0);
        tracker.inc_work_done_by(10.0, "");
        MockClock::advance(Duration::from_secs(1));
        assert_eq!(10.0, tracker.progress().items_finished);
        assert_eq!(10.0, tracker.eta().as_secs_f64());

        MockClock::advance(Duration::from_secs(2));
        tracker.inc_work_done_by(10.0, "");

        let eta = tracker.eta().as_secs_f64();
        assert!(eta >= 15.0);

        MockClock::advance(Duration::from_secs(5));
        tracker.inc_work_done_by(80.0, "");

        let eta = tracker.eta().as_secs_f64();
        assert_approx_eq!(f64, eta, 8.0, ulps = 2);
        assert!(tracker.progress().done);
    }

    #[test]
    fn test_progress_tracker() {}
}
