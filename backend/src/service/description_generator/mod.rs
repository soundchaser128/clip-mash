use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::generator::CompilationOptions;
use crate::server::types::VideoCodec;
use crate::util::StrExt;
use crate::Result;

mod json;
mod markdown;
mod yaml;

pub trait DescriptionGenerator {
    fn generate(&self, options: TemplateContext) -> Result<String>;
}

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum DescriptionType {
    Markdown,
    Json,
    Yaml,
}

impl DescriptionType {
    pub fn content_type(&self) -> &'static str {
        match self {
            Self::Markdown => "text/markdown",
            Self::Json => "application/json",
            Self::Yaml => "application/yaml",
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ClipInfo {
    pub start: f64,
    pub end: f64,
    pub marker_title: String,
    pub video_title: String,
}

#[derive(Serialize, Debug)]
pub struct TemplateContext {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub clips: Vec<ClipInfo>,
    pub codec: VideoCodec,
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
                    .and_then(|v| v.video_title.clone())
                    .map(|t| t.limit_length(45))
                    .unwrap_or_else(|| "unknown".to_string());
                ClipInfo {
                    start,
                    end,
                    marker_title: clip.marker_title,
                    video_title,
                }
            })
            .collect();

        Self {
            title: options.file_name.clone(),
            width: options.output_resolution.0,
            height: options.output_resolution.1,
            fps: options.output_fps,
            codec: options.video_codec,
            clips,
        }
    }
}

pub fn render_description(options: &CompilationOptions, ty: DescriptionType) -> Result<String> {
    let context = TemplateContext::from(options);
    match ty {
        DescriptionType::Markdown => markdown::MarkdownDescriptionGenerator.generate(context),
        DescriptionType::Json => json::JsonDescriptionGenerator.generate(context),
        DescriptionType::Yaml => yaml::YamlDescriptionGenerator.generate(context),
    }
}
