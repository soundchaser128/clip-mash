use lazy_static::lazy_static;
use serde::Serialize;
use tera::{Context, Tera};

use super::generator::CompilationOptions;

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

pub struct MarkdownDescriptionGenerator;

impl DescriptionGenerator for MarkdownDescriptionGenerator {
    fn generate(&self, options: TemplateContext) -> String {
        let mut context = Context::new();
        context.insert("video", &options);
        TEMPLATES
            .render("description.md", &context)
            .expect("failed to render markdown")
    }
}

#[cfg(test)]
mod tests {
    use super::{DescriptionGenerator, MarkdownDescriptionGenerator, TemplateContext};

    #[test]
    fn test_markdown_description() {
        let options = TemplateContext {
            title: "test".to_string(),
            width: 1920,
            height: 1080,
            fps: 30,
        };

        let description = MarkdownDescriptionGenerator.generate(options);
        println!("description: {}", description);
    }
}
