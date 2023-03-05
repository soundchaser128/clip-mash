use api::{
    find_markers_query::{
        self, CriterionModifier, FindFilterType, HierarchicalMultiCriterionInput,
        SceneMarkerFilterType,
    },
    find_performers_query, find_tags_query, Api,
};
use camino::Utf8Path;
use dialoguer::{Input, Password, Select};
use dotenvy::dotenv;
use ffmpeg::{ClipOrder, Ffmpeg};
use std::{cmp::Reverse, env};

use crate::api::find_markers_query::MultiCriterionInput;

mod api;
mod ffmpeg;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

async fn select_tags(client: &Api) -> Result<Vec<String>> {
    use dialoguer::MultiSelect;

    let tags = client.find_tags(find_tags_query::Variables {}).await?;
    let mut tags: Vec<_> = tags
        .into_iter()
        .filter(|t| t.scene_marker_count.unwrap_or_default() > 0)
        .collect();
    tags.sort_by_key(|t| Reverse(t.scene_marker_count));
    let options: Vec<_> = tags
        .iter()
        .map(|t| {
            format!(
                "{} ({} markers)",
                t.name,
                t.scene_marker_count.unwrap_or_default()
            )
        })
        .collect();

    let chosen_indices = MultiSelect::new()
        .with_prompt("Choose marker tags to include")
        .items(&options)
        .interact()?;

    Ok(chosen_indices
        .into_iter()
        .map(|idx| tags[idx].id.clone())
        .collect())
}

async fn select_performers(client: &Api) -> Result<Vec<String>> {
    use dialoguer::MultiSelect;

    let mut performers = client
        .find_performers(find_performers_query::Variables {})
        .await?;
    performers.sort_by_key(|p| Reverse(p.scene_count));

    let options: Vec<_> = performers
        .iter()
        .map(|p| format!("{} ({} scenes)", p.name, p.scene_count.unwrap_or_default()))
        .collect();
    let chosen_indices = MultiSelect::new()
        .with_prompt("Select performers to include")
        .items(&options)
        .interact()?;

    Ok(chosen_indices
        .into_iter()
        .map(|idx| performers[idx].id.clone())
        .collect())
}

enum FilterType {
    Tags,
    Performers,
}

fn select_filter() -> Result<FilterType> {
    let answer = Select::new()
        .with_prompt("Select type of filter")
        .items(&["Tags", "Performers"])
        .interact()?;

    match answer {
        0 => Ok(FilterType::Tags),
        1 => Ok(FilterType::Performers),
        _ => unreachable!(),
    }
}

fn select_clip_order() -> Result<ClipOrder> {
    let answer = Select::new()
        .with_prompt("Select the order of the clips")
        .items(&["Random", "Scene order"])
        .interact()?;

    match answer {
        0 => Ok(ClipOrder::Random),
        1 => Ok(ClipOrder::SceneOrder),
        _ => unreachable!(),
    }
}

fn setup_dotenv() -> Result<()> {
    if !Utf8Path::new(".env").is_dir() {
        let mut url = Input::<String>::new()
            .with_prompt("Enter the URL of your Stash instance (e.g. http://localhost:9999)")
            .interact_text()?;

        if url.ends_with('/') {
            url.pop();
        }

        let api_key = Password::new()
            .with_prompt(format!(
                "Enter your Stash API key from {}/settings?tab=security",
                url
            ))
            .interact()?;

        let file_contents = &[
            format!("STASHAPP_URL={url}"),
            format!("STASHAPP_API_KEY={api_key}"),
        ]
        .join("\n");

        std::fs::write(".env", file_contents)?;
        println!("Wrote configuration data to .env file.");
    }

    dotenv().expect("failed to dotenv");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_dotenv()?;

    let api_url = env::var("STASHAPP_URL").expect("missing STASHAPP_URL");
    let api_key = env::var("STASHAPP_API_KEY").expect("missing STASHAPP_API_KEY");

    println!("{}", console::style("stash-compilation-maker").bold());
    println!("
    Create a video compilation from scene markers on your Stash instance.
    Answer a few questions about what videos should be included, and then wait until the clips are downloaded and assembled.
    The resulting clips will be in the `videos` subfolder of the current working directory.
    Select options with arrow keys, use TAB to select options when multiple are allowed and enter to confirm.");
    println!();

    let client = Api::new(&api_url, &api_key);
    let filter_type = select_filter()?;
    let ids = match filter_type {
        FilterType::Performers => select_performers(&client).await?,
        FilterType::Tags => select_tags(&client).await?,
    };
    let clip_order = select_clip_order()?;
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

    match filter_type {
        FilterType::Performers => {
            filter.performers = Some(MultiCriterionInput {
                modifier: CriterionModifier::INCLUDES,
                value: Some(ids),
            });
        }
        FilterType::Tags => {
            filter.tags = Some(HierarchicalMultiCriterionInput {
                depth: None,
                modifier: CriterionModifier::INCLUDES,
                value: Some(ids),
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
    let ffmpeg = Ffmpeg::new();
    let clips = ffmpeg.gather_clips(markers, 15).await?;
    let result_file = ffmpeg.compile_clips(clips, clip_order).await?;
    println!("wrote result to {}", result_file);

    Ok(())
}
