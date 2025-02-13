use std::process::Output;

use camino::Utf8Path;
use tracing::{debug, Level};
use url::Url;

pub fn add_api_key(url: &str, api_key: Option<&str>) -> String {
    let mut url = Url::parse(url).expect("invalid url");
    if let Some(api_key) = api_key {
        url.query_pairs_mut().append_pair("apikey", api_key);
    }
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

    let mut message = format!(
        "Command '{}' failed with exit code {}.",
        command_name,
        output.status.code().unwrap_or(1)
    );

    if let Some(stdout) = std::str::from_utf8(&output.stdout).ok() {
        if !stdout.is_empty() {
            message.push_str("\nProcess standard output:\n");
            message.push_str(stdout);
        }
    }

    if let Some(stderr) = std::str::from_utf8(&output.stderr).ok() {
        if !stderr.is_empty() {
            message.push_str("\nProcess error output:\n");
            message.push_str(stderr);
        }
    }

    Err(eyre!(message))
}

pub fn debug_output(output: Output) {
    if tracing::enabled!(Level::DEBUG) {
        let stdout = std::str::from_utf8(&output.stdout).unwrap();
        let stderr = std::str::from_utf8(&output.stderr).unwrap();

        if !stdout.is_empty() {
            debug!("stdout = '{}'", stdout);
        }
        if !stderr.is_empty() {
            debug!("stderr = '{}'", stderr);
        }
    }
}

/// Formats a number of seconds as a mm:ss string
pub fn format_duration(seconds: f64) -> String {
    let minutes = (seconds / 60.0).floor();
    let seconds = (seconds % 60.0).floor();
    format!("{:02}:{:02}", minutes, seconds)
}

pub trait StrExt {
    fn limit_length(&self, max_length: usize) -> String;

    fn collapse_whitespace(&self) -> String;
}

impl StrExt for str {
    fn limit_length(&self, max_length: usize) -> String {
        if self.len() > max_length {
            format!("{}…", &self[..max_length])
        } else {
            self.to_string()
        }
    }

    fn collapse_whitespace(&self) -> String {
        self.split_whitespace()
            .map(|s| s.replace("\n", " "))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod test {
    use regex::Regex;

    use super::{add_api_key, expect_file_name, format_duration};
    use crate::helpers::random::generate_id;

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
        let result = add_api_key("http://localhost:3001", Some("super-secret-123"));
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
