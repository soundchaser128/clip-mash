#![allow(unused)]

use std::cmp::Reverse;

use dialoguer::{Input, MultiSelect, Select};

use crate::ffmpeg::formatted_scene;
use crate::stash_api::find_markers_query::{
    self, CriterionModifier, FindFilterType,
    FindMarkersQueryFindSceneMarkersSceneMarkers as Marker, HierarchicalMultiCriterionInput,
    MultiCriterionInput, SceneMarkerFilterType,
};
use crate::stash_api::{find_performers_query, find_tags_query};
use crate::Result;
use crate::{ffmpeg::ClipOrder, stash_api::Api};

#[derive(Debug)]
pub enum Filter {
    TagFilter(Vec<String>),
    PerformerFilter(Vec<String>),
}

enum FilterType {
    Tags,
    Performers,
}

#[derive(Debug)]
pub struct CompilationInfo {
    pub filter: Filter,
    pub clip_order: ClipOrder,
    pub video_name: String,
    pub clip_duration: u32,
    pub output_resolution: (u32, u32),
    pub output_fps: f64,
    pub markers: Vec<Marker>,
}

pub struct Cli<'a> {
    client: &'a Api,
}

impl<'a> Cli<'a> {
    pub fn new(api: &'a Api) -> Self {
        Cli { client: api }
    }

    async fn select_tags(&self) -> Result<Vec<String>> {
        let tags = self.client.find_tags(find_tags_query::Variables {}).await?;
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

    async fn select_performers(&self) -> Result<Vec<String>> {
        let mut performers = self
            .client
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

    fn select_filter(&self) -> Result<FilterType> {
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

    fn select_clip_order(&self) -> Result<ClipOrder> {
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

    fn select_video_name(&self) -> Result<String> {
        let mut answer = Input::<String>::new()
            .with_prompt("What should the video be called? This will be the file name of the resulting compilation.")
            .default("compilation".into())
            .interact_text()?;

        // strip potential extensions to not confuse ffmpeg, we always want mp4 output (for now)
        if let Some(idx) = answer.find('.') {
            answer = answer[0..idx].into();
        }

        Ok(answer)
    }

    fn select_clip_duration(&self) -> Result<u32> {
        let answer = Input::<u32>::new()
            .with_prompt("Enter the maximum duration (in seconds) for each clip")
            .default(15)
            .validate_with(|input: &u32| -> std::result::Result<(), &str> {
                if *input >= 1 {
                    Ok(())
                } else {
                    Err("Clips must be at least one second long")
                }
            })
            .interact()?;

        Ok(answer)
    }

    fn select_output_resolution(&self) -> Result<(u32, u32)> {
        let answer = Select::new()
            .with_prompt("Select the output resolution")
            .items(&["720p", "1080p", "4K"])
            .interact()?;

        match answer {
            0 => Ok((1280, 720)),
            1 => Ok((1920, 1080)),
            2 => Ok((3840, 2160)),
            _ => unreachable!(),
        }
    }

    fn select_output_fps(&self) -> Result<f64> {
        let answer = Input::<f64>::new()
            .with_prompt("Enter the frame rate for the output video")
            .default(30.0)
            .interact()?;

        Ok(answer)
    }

    async fn select_markers(&self, filter: &Filter) -> Result<Vec<Marker>> {
        let mut scene_filter = SceneMarkerFilterType {
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

        match filter {
            Filter::PerformerFilter(ids) => {
                scene_filter.performers = Some(MultiCriterionInput {
                    modifier: CriterionModifier::INCLUDES,
                    value: Some(ids.clone()),
                });
            }
            Filter::TagFilter(ids) => {
                scene_filter.tags = Some(HierarchicalMultiCriterionInput {
                    depth: None,
                    modifier: CriterionModifier::INCLUDES,
                    value: Some(ids.clone()),
                });
            }
        }

        let markers = self
            .client
            .find_markers(find_markers_query::Variables {
                filter: Some(FindFilterType {
                    per_page: Some(-1),
                    page: None,
                    q: None,
                    sort: None,
                    direction: None,
                }),
                scene_marker_filter: Some(scene_filter),
            })
            .await?;
        let items: Vec<_> = markers
            .iter()
            .map(|m| {
                format!(
                    "Scene {}: {} at {} seconds",
                    formatted_scene(&m),
                    m.primary_tag.name,
                    m.seconds
                )
            })
            .collect();
        let defaults: Vec<_> = std::iter::repeat(true).take(markers.len()).collect();
        let chosen_indices = MultiSelect::new()
            .with_prompt("Select markers to include")
            .items(&items)
            .defaults(&defaults)
            .interact()?;

        Ok(chosen_indices
            .into_iter()
            .map(|idx| markers[idx].clone())
            .collect())
    }

    pub async fn ask_questions(&self) -> Result<CompilationInfo> {
        let filter_type = self.select_filter()?;
        let filter = match filter_type {
            FilterType::Performers => Filter::PerformerFilter(self.select_performers().await?),
            FilterType::Tags => Filter::TagFilter(self.select_tags().await?),
        };
        let clip_order = self.select_clip_order()?;
        let video_name = self.select_video_name()?;
        let clip_duration = self.select_clip_duration()?;
        let output_resolution = self.select_output_resolution()?;
        let output_fps = self.select_output_fps()?;
        let markers = self.select_markers(&filter).await?;

        Ok(CompilationInfo {
            filter,
            clip_order,
            video_name,
            clip_duration,
            output_fps,
            output_resolution,
            markers,
        })
    }

    pub fn print_info(&self) {
        println!("{}", console::style("stash-compilation-maker").bold());
        println!("
        Create a video compilation from scene markers on your Stash instance.
        Answer a few questions about what videos should be included, and then wait until the clips are downloaded and assembled.
        The resulting clips will be in the `videos` subfolder of the current working directory.
        Select options with arrow keys, use TAB to select options when multiple are allowed and enter to confirm.");
        println!();
    }
}
