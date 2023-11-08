use tera::{Context, Tera};

use super::{DescriptionGenerator, TemplateContext};
use crate::Result;

pub struct MarkdownDescriptionGenerator;

impl DescriptionGenerator for MarkdownDescriptionGenerator {
    fn generate(&self, options: TemplateContext) -> Result<String> {
        let mut context = Context::new();
        context.insert("video", &options);
        Tera::one_off(
            include_str!("../../../data/templates/description.md"),
            &context,
            false,
        )
        .map_err(From::from)
    }
}

#[cfg(test)]
mod tests {
    use super::{DescriptionGenerator, MarkdownDescriptionGenerator, TemplateContext};
    use crate::server::types::VideoCodec;

    #[test]
    fn test_markdown_description() {
        let options = TemplateContext {
            title: "test".to_string(),
            width: 1920,
            height: 1080,
            fps: 30,
            clips: vec![],
            codec: VideoCodec::H264,
        };

        let description = MarkdownDescriptionGenerator
            .generate(options)
            .expect("failed to generate");
        println!("description: {}", description);
    }
}
