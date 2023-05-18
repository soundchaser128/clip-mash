use std::path::Path;

use axum::routing::{get_service, MethodRouter};
use include_dir::{include_dir, Dir};
use tower_http::services::{ServeDir, ServeFile};

static BUILD_DIR: Dir<'_> = include_dir!("frontend/dist");

pub fn service() -> MethodRouter {
    let base_dir = Path::new("assets");
    if !base_dir.is_dir() {
        std::fs::create_dir_all(&base_dir).expect("cannot create assets folder, cannot continue")
    }

    BUILD_DIR.extract(base_dir).unwrap();
    let index_html = base_dir.join("index.html");
    get_service(
        ServeDir::new(base_dir)
            .append_index_html_on_directories(true)
            .not_found_service(ServeFile::new(index_html)),
    )
}
