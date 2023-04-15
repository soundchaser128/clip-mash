use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{clip::Clip, stash_api::Api, Result};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FSPoint {
    pub pos: i32,
    /// Position in the video in milliseconds
    pub at: u32,
}

/// properties about a pressure simulator
/// that can be used to input points in a .funscript
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
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

/// extra metadata, specifically for OpenFunscripter (OFS)
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct OFSMetadata {
    pub bookmarks: Vec<i32>,
    pub chapters: Vec<String>,
    pub creator: String,
    pub description: String,
    pub duration: i32,
    pub license: String,
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
#[serde(deny_unknown_fields, rename_all = "camelCase", default)]
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
    pub metadata: OFSMetadata,
}

impl Default for FunScript {
    fn default() -> Self {
        Self {
            version: "".to_string(),
            inverted: false,
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
            metadata: OFSMetadata {
                bookmarks: Vec::new(),
                chapters: Vec::new(),
                creator: "".to_string(),
                description: "".to_string(),
                duration: -1,
                license: "".to_string(),
                notes: "".to_string(),
                performers: Vec::new(),
                script_url: "".to_string(),
                tags: Vec::new(),
                title: "".to_string(),
                ofs_type: "".to_string(),
                video_url: "".to_string(),
            },
        }
    }
}

pub struct ScriptBuilder<'a> {
    api: &'a Api,
}

impl<'a> ScriptBuilder<'a> {
    pub fn new(api: &'a Api) -> Self {
        Self { api }
    }

    pub async fn combine_scripts(&self, clips: Vec<Clip>) -> Result<FunScript> {
        let mut resulting_actions = vec![];
        let mut offset = 0;

        for clip in clips {
            let (start, end) = clip.range_millis();
            let duration = end - start;
            let script = self.api.get_funscript(&clip.scene_id).await?;

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

        let mut script = FunScript::default();

        script.actions = resulting_actions;
        script.metadata.creator = "stash-compilation-maker".into();
        Ok(script)
    }
}