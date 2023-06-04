mod beats;
mod download;

pub use self::beats::{detect_beats, parse_beats};
pub use self::download::{MusicDownloadService, SongInfo};
