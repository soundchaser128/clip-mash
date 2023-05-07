use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{Clip, Video};
use crate::{data::stash_api::StashApi, service::VideoInfo, Result};

// Funscript structs taken from https://github.com/JPTomorrow/funscript-rs/blob/main/src/funscript.rs

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FSPoint {
    pub pos: i32,
    /// Position in the video in milliseconds
    pub at: u32,
}

/// properties about a pressure simulator
/// that can be used to input points in a .funscript
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulatorPresets {
    pub name: String,
    pub full_range: bool,
    pub direction: i32,
    pub rotation: f32,
    pub length: f32,
    pub width: f32,
    pub offset: String,
    pub color: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct OFSMetadata {
    pub creator: String,
    pub description: String,
    pub duration: i32,
    pub notes: String,
    pub performers: Vec<String>,
    #[serde(rename = "script_url")]
    pub script_url: String,
    pub tags: Vec<String>,
    pub title: String,
    #[serde(rename = "type")]
    pub ofs_type: String,
    #[serde(rename = "video_url")]
    pub video_url: String,
}

/// a serializable and deserializable .funscript file
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct FunScript {
    pub version: String,
    pub inverted: bool,
    pub range: i32,
    pub bookmark: i32,
    pub last_position: i64,
    pub graph_duration: i32,
    pub speed_ratio: f32,
    pub injection_speed: i32,
    pub injection_bias: f32,
    pub scripting_mode: i32,
    pub simulator_presets: Vec<SimulatorPresets>,
    pub active_simulator: i32,
    pub reduction_tolerance: f32,
    pub reduction_stretch: f32,
    pub clips: Vec<Value>,
    pub actions: Vec<FSPoint>,
    pub raw_actions: Vec<FSPoint>,
    pub metadata: Option<OFSMetadata>,
}

impl FunScript {
    pub async fn load(path: impl AsRef<Utf8Path>) -> Result<Self> {
        let text = tokio::fs::read_to_string(path.as_ref()).await?;
        Ok(serde_json::from_str(&text)?)
    }
}

impl Default for FunScript {
    fn default() -> Self {
        FunScript {
            range: -1,
            bookmark: -1,
            last_position: -1,
            graph_duration: -1,
            speed_ratio: -1.0,
            injection_speed: -1,
            injection_bias: -1.0,
            scripting_mode: -1,
            simulator_presets: Vec::new(),
            active_simulator: -1,
            reduction_tolerance: -1.0,
            reduction_stretch: -1.0,
            clips: Vec::new(),
            actions: Vec::new(),
            raw_actions: Vec::new(),
            metadata: Default::default(),
            inverted: false,
            version: "".to_string(),
        }
    }
}

pub struct ScriptBuilder<'a> {
    api: &'a StashApi,
}

impl<'a> ScriptBuilder<'a> {
    pub fn new(api: &'a StashApi) -> Self {
        Self { api }
    }

    pub async fn combine_scripts(&self, clips: Vec<(Video, Clip)>) -> Result<FunScript> {
        let mut resulting_actions = vec![];
        let mut offset = 0;

        for (video, clip) in clips {
            let (start, end) = clip.range_millis();
            let duration = end - start;

            let script = match video.info {
                VideoInfo::Stash { .. } => self.api.get_funscript(&clip.video_id.to_string()).await,
                VideoInfo::LocalFile { video } => {
                    let path = Utf8PathBuf::from(video.file_path).with_extension("funscript");
                    FunScript::load(path).await
                }
            };
            match script {
                Ok(script) => {
                    let actions: Vec<_> = script
                        .actions
                        .into_iter()
                        .filter(|a| a.at >= start && a.at <= end)
                        .map(|action| FSPoint {
                            pos: action.pos,
                            at: (action.at - start) + offset,
                        })
                        .collect();

                    resulting_actions.extend(actions);
                    offset += duration;
                }
                Err(e) => {
                    tracing::warn!(
                        "failed to get .funscript for scene ID {}: {}",
                        clip.video_id,
                        e
                    )
                }
            }
        }

        let version = env!("CARGO_PKG_VERSION");

        let script = FunScript {
            actions: resulting_actions,
            metadata: Some(OFSMetadata {
                creator: format!("clip-mash v{}", version),
                ..Default::default()
            }),
            ..Default::default()
        };

        tracing::info!("generated funscript with {} actions", script.actions.len());

        Ok(script)
    }
}
