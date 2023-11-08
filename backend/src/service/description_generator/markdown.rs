use tera::Context;

use super::{DescriptionGenerator, TemplateContext, TEMPLATES};

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
            clips: vec![],
        };

        let description = MarkdownDescriptionGenerator.generate(options);
        println!("description: {}", description);
    }
}
