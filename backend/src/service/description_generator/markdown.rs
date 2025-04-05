use minijinja::{context, Environment};
use tracing::info;

use super::{DescriptionGenerator, TemplateContext};
use crate::Result;

pub struct MarkdownDescriptionGenerator {
    environment: Environment<'static>,
}

impl MarkdownDescriptionGenerator {
    pub fn new() -> Result<Self> {
        let mut environment = Environment::new();
        let template = include_str!("../../../data/templates/description.md");
        environment.add_template("description", template)?;
        environment.set_trim_blocks(true);
        environment.set_lstrip_blocks(true);

        Ok(MarkdownDescriptionGenerator { environment })
    }
}

impl DescriptionGenerator for MarkdownDescriptionGenerator {
    fn generate(&self, options: TemplateContext) -> Result<String> {
        info!("Generating Markdown description");
        self.environment
            .get_template("description")?
            .render(context! {ctx => options})
            .map_err(From::from)
    }
}

#[cfg(test)]
mod tests {
    use super::{DescriptionGenerator, MarkdownDescriptionGenerator, TemplateContext};
    use crate::{server::types::VideoCodec, service::description_generator::ClipInfo};

    #[test]
    fn test_markdown_description() {
        let options = TemplateContext {
            title: "test".to_string(),
            width: 1920,
            height: 1080,
            fps: 30,
            clips: vec![ClipInfo {
                start: "00:00:00".to_string(),
                end: "00:00:10".to_string(),
                marker_title: "test_marker".to_string(),
                video_title: "test_video".to_string(),
            }],
            codec: VideoCodec::H264,
            videos: vec![],
        };

        let generator = MarkdownDescriptionGenerator::new().expect("failed to create generator");
        let description = generator.generate(options).expect("failed to generate");
    }
}
