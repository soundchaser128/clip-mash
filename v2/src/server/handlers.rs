use crate::{data::database::Database, service::generator::CompilationGenerator};

pub struct AppState {
    pub generator: CompilationGenerator,
    pub database: Database,
}

pub mod stash {

}

pub mod local {

}
