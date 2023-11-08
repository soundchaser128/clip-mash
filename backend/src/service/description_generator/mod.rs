use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::generator::CompilationOptions;
use crate::server::types::VideoCodec;
use crate::util::StrExt;
use crate::Result;

mod markdown;

pub trait DescriptionGenerator {
    fn generate(&self, options: TemplateContext) -> Result<String>;
}

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum DescriptionType {
    Markdown,
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
        Self {
            title: options.file_name.clone(),
            width: options.output_resolution.0,
            height: options.output_resolution.1,
            fps: options.output_fps,
            codec: options.video_codec,
            clips: options
                .clips
                .clone()
                .into_iter()
                .map(|clip| {
                    let video_title = options
                        .videos
                        .iter()
                        .find(|v| v.id == clip.video_id)
                        .and_then(|v| v.video_title.clone())
                        .map(|t| t.limit_length(45))
                        .unwrap_or_else(|| "unknown".to_string());
                    ClipInfo {
                        start: clip.range.0,
                        end: clip.range.1,
                        marker_title: clip.marker_title,
                        video_title,
                    }
                })
                .collect(),
        }
    }
}

pub fn render_description(options: &CompilationOptions, ty: DescriptionType) -> Result<String> {
    let context = TemplateContext::from(options);
    match ty {
        DescriptionType::Markdown => markdown::MarkdownDescriptionGenerator.generate(context),
    }
}
