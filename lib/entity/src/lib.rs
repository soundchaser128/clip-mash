use sea_orm::DbErr;

pub mod ffprobe;
pub mod markers;
pub mod music;
pub mod performers;
pub mod progress;
pub mod settings;
pub mod videos;

pub type DbResult<T> = Result<T, DbErr>;
