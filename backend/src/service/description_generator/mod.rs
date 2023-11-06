use super::generator::CompilationOptions;

pub trait DescriptionGenerator {
    fn generate(&self, options: &CompilationOptions) -> String;
}

pub struct MarkdownDescriptionGenerator;

impl DescriptionGenerator for MarkdownDescriptionGenerator {
    fn generate(&self, options: &CompilationOptions) -> String {
        let mut description = String::new();

        description.push_str(&format!("# Compilation '{}'\n\n", options.file_name));
        description.push_str("## Video details\n\n");
        // description.push_str(
        //     "- Resolution: **{} x {}**\n",
        //     options.output_resolution.0,
        //     options.output_resolution.1,
        // );
        description.push_str(&format!("- **{}** FPS\n", options.output_fps));

        for (index, clip) in options.clips.iter().enumerate() {
            description.push_str(&format!(
                "## Clip {} / {}\n\n",
                index + 1,
                options.clips.len()
            ));
            description.push_str(&format!("- Marker title: {}\n", clip.marker_title));
            // description.push_str(&format!("- Video title: {}\n", ));
        }

        description
    }
}
