use tracing::info;

use super::{DescriptionGenerator, TemplateContext};
use crate::Result;
use crate::data::database::videos::VideoSource;

pub struct MarkdownDescriptionGenerator;

impl DescriptionGenerator for MarkdownDescriptionGenerator {
    fn generate(&self, ctx: TemplateContext) -> Result<String> {
        info!("Generating Markdown description");
        let mut string = format!(
            "# Compilation '{}'
Created with [ClipMash](https://github.com/soundchaser128/clip-mash).

## Video information

- Resolution: **{} x {}**
- Frames per second: **{}**
- Video codec: **{}**\n\n",
            ctx.title, ctx.width, ctx.height, ctx.fps, ctx.codec
        );

        string.push_str("## Clips\n\n");
        string.push_str(
            "| Video | Description | Start | End |\n| ----- | ----------- | ----- | --- |\n",
        );
        for clip in ctx.clips {
            string.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                clip.video_title, clip.marker_title, clip.start, clip.end
            ));
        }

        string.push_str("\n## Videos\n\n");
        string.push_str("| Video | Title | Interactive |\n| ----- | ----- | ----------- |\n");
        for video in ctx.videos {
            string.push_str(&format!(
                "| {} | {} | {} |\n",
                match video.source {
                    VideoSource::Folder => "Local folder",
                    VideoSource::Download => "Downloaded",
                    VideoSource::Stash => "Stash",
                },
                video.title,
                video.interactive
            ));
        }

        Ok(string)
    }
}

#[cfg(test)]
mod tests {
    use super::{DescriptionGenerator, MarkdownDescriptionGenerator, TemplateContext};
    use crate::service::description_generator::ClipInfo;
    use crate::types::VideoCodec;

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

        let generator = MarkdownDescriptionGenerator;
        let description = generator.generate(options).expect("failed to generate");
        assert!(description.contains("# Compilation 'test'"));
    }
}
