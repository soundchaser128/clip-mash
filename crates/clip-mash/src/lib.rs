pub mod data;
pub mod helpers;
pub mod service;

use color_eyre::eyre::Report;
pub use helpers::util;

pub type Result<T> = std::result::Result<T, Report>;
