use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::generator::CompilationOptions;
use crate::Result;
use crate::data::database::videos::VideoSource;
use crate::types::VideoCodec;
use crate::util::StrExt;

mod json;
mod markdown;

pub trait DescriptionGenerator {
    fn generate(&self, options: TemplateContext) -> Result<String>;
}

fn format_timestamp(value: f64) -> String {
    let hours = value.floor() as u32 / 3600;
    let minutes = value.floor() as u32 / 60;
    let seconds = value.floor() as u32 % 60;
    let milliseconds = (value.fract() * 1000.0).floor() as u32;

    format!(
        "{:02}:{:02}:{:02}.{:03}",
        hours, minutes, seconds, milliseconds
    )
}

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum DescriptionType {
    Markdown,
    Json,
}

impl DescriptionType {
    pub fn content_type(&self) -> &'static str {
        match self {
            Self::Markdown => "text/markdown",
            Self::Json => "application/json",
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ClipInfo {
    pub start: String,
    pub end: String,
    pub marker_title: String,
    pub video_title: String,
}

#[derive(Serialize, Debug)]
pub struct VideoInfo {
    pub source: VideoSource,
    pub title: String,
    pub interactive: &'static str,
}

#[derive(Serialize, Debug)]
pub struct TemplateContext {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub clips: Vec<ClipInfo>,
    pub codec: VideoCodec,
    pub videos: Vec<VideoInfo>,
}

impl From<&CompilationOptions> for TemplateContext {
    fn from(options: &CompilationOptions) -> Self {
        let mut position = 0.0;
        let clips = options
            .clips
            .clone()
            .into_iter()
            .map(|clip| {
                let start = position;
                let end = position + clip.range.1 - clip.range.0;
                position = end;
                let video_title = options
                    .videos
                    .iter()
                    .find(|v| v.id == clip.video_id)
                    .map(|v| v.video_title.as_ref().unwrap_or(&v.id).to_string())
                    .map(|t| t.limit_length(45))
                    .unwrap_or_else(|| "unknown".to_string());
                ClipInfo {
                    start: format_timestamp(start),
                    end: format_timestamp(end),
                    marker_title: clip.marker_title,
                    video_title,
                }
            })
            .collect();

        Self {
            title: Some(&options.file_name)
                .filter(|s| s.is_empty())
                .unwrap_or(&options.video_id)
                .to_string(),
            width: options.output_resolution.0,
            height: options.output_resolution.1,
            fps: options.output_fps,
            codec: options.video_codec,
            clips,
            videos: options
                .videos
                .iter()
                .map(|v| VideoInfo {
                    source: v.source.clone(),
                    title: v.video_title.as_ref().unwrap_or(&v.id).to_string(),
                    interactive: if v.interactive { "Yes" } else { "No" },
                })
                .collect(),
        }
    }
}

pub fn render_description(options: &CompilationOptions, ty: DescriptionType) -> Result<String> {
    let context = TemplateContext::from(options);
    match ty {
        DescriptionType::Markdown => markdown::MarkdownDescriptionGenerator.generate(context),
        DescriptionType::Json => json::JsonDescriptionGenerator.generate(context),
    }
}
