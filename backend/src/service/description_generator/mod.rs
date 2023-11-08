use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tera::Tera;
use utoipa::ToSchema;

use super::generator::CompilationOptions;
use crate::server::types::Clip;

mod markdown;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = match Tera::new("data/templates/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera
    };
}

pub trait DescriptionGenerator {
    fn generate(&self, options: TemplateContext) -> String;
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
    // pub video_title: String,
}

impl From<Clip> for ClipInfo {
    fn from(value: Clip) -> Self {
        ClipInfo {
            start: value.range.0,
            end: value.range.1,
            marker_title: value.marker_title,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct TemplateContext {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub clips: Vec<ClipInfo>,
}

impl From<&CompilationOptions> for TemplateContext {
    fn from(options: &CompilationOptions) -> Self {
        Self {
            title: options.file_name.clone(),
            width: options.output_resolution.0,
            height: options.output_resolution.1,
            fps: options.output_fps,
            clips: options
                .clips
                .clone()
                .into_iter()
                .map(|c| c.into())
                .collect(),
        }
    }
}

pub fn render_description(options: &CompilationOptions, ty: DescriptionType) -> String {
    let context = TemplateContext::from(options);
    match ty {
        DescriptionType::Markdown => markdown::MarkdownDescriptionGenerator.generate(context),
    }
}
