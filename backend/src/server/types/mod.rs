pub use clip::*;
pub use marker::*;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
pub use video::*;

mod clip;
mod marker;
mod video;

#[derive(Serialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
#[aliases(
    ListVideoDtoPage = Page<ListVideoDto>,
    StashVideoDtoPage = Page<StashVideoDto>,
    MarkerDtoPage = Page<MarkerDto>,
)]
pub struct Page<T> {
    pub content: Vec<T>,
    pub total_items: usize,
    pub page_number: usize,
    pub page_size: usize,
    pub total_pages: usize,
}

impl<T> Page<T> {
    pub fn empty() -> Self {
        Page {
            content: vec![],
            total_items: 0,
            page_number: 0,
            page_size: 0,
            total_pages: 0,
        }
    }
}

impl<T: Serialize + ToSchema<'static>> Page<T> {
    pub fn new(content: Vec<T>, size: usize, page: PageParameters) -> Self {
        let page_number = page.page.unwrap_or(PageParameters::DEFAULT_PAGE as usize);
        let page_size = page.size.unwrap_or(PageParameters::DEFAULT_SIZE as usize);
        let total_pages = (size as f64 / page_size as f64).ceil() as usize;

        Page {
            content,
            total_items: size,
            page_number,
            page_size,
            total_pages,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct Beats {
    pub offsets: Vec<f32>,
    pub length: f32,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SongDto {
    pub song_id: i64,
    pub duration: f64,
    pub file_name: String,
    pub url: String,
    pub beats: Vec<f32>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NewId {
    pub id: String,
}

#[derive(Deserialize, Debug, Clone, Copy, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Deserialize, Debug, Clone, IntoParams)]
pub struct PageParameters {
    pub page: Option<usize>,
    pub size: Option<usize>,
    pub sort: Option<String>,
    #[allow(unused)]
    pub dir: Option<SortDirection>,
}

impl PageParameters {
    pub const DEFAULT_PAGE: i64 = 0;
    pub const DEFAULT_SIZE: i64 = 20;

    #[allow(unused)]
    pub fn new(page: usize, size: usize) -> Self {
        Self {
            page: Some(page),
            size: Some(size),
            sort: None,
            dir: None,
        }
    }

    pub fn limit(&self) -> i64 {
        self.size.map(|s| s as i64).unwrap_or(Self::DEFAULT_SIZE)
    }

    pub fn offset(&self) -> i64 {
        self.page
            .map(|p| p as i64 * self.limit())
            .unwrap_or(Self::DEFAULT_PAGE)
    }

    pub fn size(&self) -> i64 {
        self.size.map(|s| s as i64).unwrap_or(Self::DEFAULT_SIZE)
    }

    pub fn page(&self) -> i64 {
        self.page.map(|p| p as i64).unwrap_or(Self::DEFAULT_PAGE)
    }
}

#[derive(Debug, Default, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Progress {
    pub video_id: String,
    pub items_finished: f64,
    pub items_total: f64,
    pub done: bool,
    pub eta_seconds: Option<f64>,
    pub message: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum StrokeType {
    /// Creates a stroke every `n` beats
    EveryNth { n: usize },
    /// Steadily accelerates the strokes from `start_strokes_per_beat` to `end_strokes_per_beat`
    Accelerate {
        start_strokes_per_beat: f32,
        end_strokes_per_beat: f32,
    },
}

impl StrokeType {
    #[allow(unused)]
    pub fn initial_acceleration(&self) -> Option<f32> {
        match self {
            Self::Accelerate {
                start_strokes_per_beat,
                ..
            } => Some(*start_strokes_per_beat),
            _ => None,
        }
    }
}

#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateBeatFunscriptBody {
    pub song_ids: Vec<i64>,
    pub stroke_type: StrokeType,
}
