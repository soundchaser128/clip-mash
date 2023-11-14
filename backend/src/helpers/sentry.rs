use std::env;

use sentry::ClientInitGuard;
use tracing::info;

const DISABLE_SENTRY: &str = "CLIP_MASH_DISABLE_SENTRY";

pub fn setup() -> Option<ClientInitGuard> {
    let is_debug_build = cfg!(debug_assertions);

    if is_debug_build {
        info!("Debug build, not initializing Sentry");
        return None;
    }

    if env::var(DISABLE_SENTRY).is_ok() {
        info!(
            "Environment variable {} is set, not initializing Sentry",
            DISABLE_SENTRY
        );
        None
    } else {
        if let Some(uri) = option_env!("CLIP_MASH_SENTRY_URI") {
            let guard = sentry::init((
                uri,
                sentry::ClientOptions {
                    release: sentry::release_name!(),
                    ..Default::default()
                },
            ));

            info!("Sentry initialized. If you want to disable it, set the environment variable {} to any value.", DISABLE_SENTRY);
            Some(guard)
        } else {
            info!("Sentry URI not found, not initializing Sentry.");
            None
        }
    }
}
