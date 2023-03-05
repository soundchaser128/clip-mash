use api::{
    find_markers_query::{
        self, CriterionModifier, FindFilterType, HierarchicalMultiCriterionInput,
        SceneMarkerFilterType,
    },
    Api,
};
use ffmpeg::Ffmpeg;
use std::env;

use crate::{
    api::find_markers_query::MultiCriterionInput,
    cli::{Cli, Filter},
    ffmpeg::OutputFormat,
};

mod api;
mod cli;
mod ffmpeg;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    cli::setup_dotenv()?;

    let api_url = env::var("STASHAPP_URL").expect("missing STASHAPP_URL");
    let api_key = env::var("STASHAPP_API_KEY").expect("missing STASHAPP_API_KEY");

    let client = Api::new(&api_url, &api_key);
    let cli = Cli::new(&client);
    cli.print_info();
    let options = cli.ask_questions().await?;

    let mut filter = SceneMarkerFilterType {
        created_at: None,
        scene_created_at: None,
        scene_updated_at: None,
        updated_at: None,
        performers: None,
        scene_date: None,
        scene_tags: None,
        tag_id: None,
        tags: None,
    };

    match &options.filter {
        Filter::PerformerFilter(ids) => {
            filter.performers = Some(MultiCriterionInput {
                modifier: CriterionModifier::INCLUDES,
                value: Some(ids.clone()),
            });
        }
        Filter::TagFilter(ids) => {
            filter.tags = Some(HierarchicalMultiCriterionInput {
                depth: None,
                modifier: CriterionModifier::INCLUDES,
                value: Some(ids.clone()),
            });
        }
    }

    let markers = client
        .find_markers(find_markers_query::Variables {
            filter: Some(FindFilterType {
                per_page: Some(-1),
                page: None,
                q: None,
                sort: None,
                direction: None,
            }),
            scene_marker_filter: Some(filter),
        })
        .await?;
    let output = OutputFormat::from(options);
    let ffmpeg = Ffmpeg::new();
    let clips = ffmpeg.gather_clips(markers, &output).await?;
    let result_file = ffmpeg.compile_clips(clips, output).await?;
    println!("wrote result to {}", result_file);

    Ok(())
}
