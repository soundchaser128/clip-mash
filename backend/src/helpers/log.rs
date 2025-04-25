use std::env;

use camino::Utf8PathBuf;
use time::macros::format_description;
use tracing::info;
use tracing_appender::non_blocking::WorkerGuard;

use crate::Result;

const LOGS_DIR: &str = "./logs";

pub fn setup_logger() -> WorkerGuard {
    use tracing_subscriber::EnvFilter;
    use tracing_subscriber::prelude::*;

    if env::var("RUST_LOG").is_err() {
        // TODO: Audit that the environment access only happens in single-threaded code.
        unsafe { env::set_var("RUST_LOG", "info") };
    }
    let file_appender = tracing_appender::rolling::daily(LOGS_DIR, "clip-mash.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_writer(non_blocking.and(std::io::stdout))
        .with_ansi(true)
        .with_env_filter(EnvFilter::from_default_env())
        .compact()
        .init();

    guard
}

pub fn cleanup_logs() -> Result<()> {
    use std::fs;

    let today = time::OffsetDateTime::now_utc().date();

    for entry in fs::read_dir(LOGS_DIR)? {
        let entry = entry?;
        let path = Utf8PathBuf::from_path_buf(entry.path()).expect("must be utf-8 path");
        let format = format_description!("[year]-[month]-[day]");
        if let Some(name) = path.file_name() {
            let date = name.replace("clip-mash.log.", "");
            let date = time::Date::parse(&date[0..10], &format)?;
            let elapsed = today - date;
            if elapsed.whole_days() > 7 {
                info!("removing old log file: {path}");
                fs::remove_file(path)?;
            }
        }
    }

    Ok(())
}
