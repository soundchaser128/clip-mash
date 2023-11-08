use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tera::Tera;
use utoipa::ToSchema;

use super::generator::CompilationOptions;

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
pub struct TemplateContext {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
}

impl From<&CompilationOptions> for TemplateContext {
    fn from(options: &CompilationOptions) -> Self {
        Self {
            title: options.file_name.clone(),
            width: options.output_resolution.0,
            height: options.output_resolution.1,
            fps: options.output_fps,
        }
    }
}

pub fn render_description(options: &CompilationOptions, ty: DescriptionType) -> String {
    let context = TemplateContext::from(options);
    match ty {
        DescriptionType::Markdown => markdown::MarkdownDescriptionGenerator.generate(context),
    }
}
