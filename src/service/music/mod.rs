mod beats;
mod download;

pub use self::beats::{detect_beats, parse_beats, Beats};
pub use self::download::{MusicDownloadService, SongInfo};
