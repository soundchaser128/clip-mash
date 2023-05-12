use crate::{data::database::Database, service::generator::CompilationGenerator};

pub mod common;
pub mod local;
pub mod stash;

pub struct AppState {
    pub generator: CompilationGenerator,
    pub database: Database,
}
