use std::collections::HashMap;

use camino::{Utf8Path, Utf8PathBuf};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{debug, info, warn};

use crate::data::database::videos::{DbVideo, VideoSource};
use crate::data::stash_api::StashApi;
use crate::server::types::{Beats, Clip, StrokeType};
use crate::util::lerp;
use crate::Result;

// Funscript structs taken from https://github.com/JPTomorrow/funscript-rs/blob/main/src/funscript.rs

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FSPoint {
    pub pos: i32,
    /// Position in the video in milliseconds
    pub at: u32,
}

/// properties about a pressure simulator
/// that can be used to input points in a .funscript
#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Clone)]
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

struct BeatState {
    songs: Vec<Beats>,
    stroke_type: StrokeType,
    song_index: usize,
    offset: f32,
    total_duration: f32,
}

impl BeatState {
    pub fn new(mut songs: Vec<Beats>, stroke_type: StrokeType) -> Self {
        for song in &mut songs {
            song.offsets.reverse();
        }
        let total_duration = songs.iter().map(|s| s.length).sum();
        info!("total duration: {}", total_duration);
        Self {
            songs,
            stroke_type,
            song_index: 0,
            offset: 0.0,
            total_duration,
        }
    }

    pub fn next_offset(&mut self) -> Option<Vec<f32>> {
        if self.song_index == self.songs.len() - 1 && self.songs[self.song_index].offsets.is_empty()
        {
            return None;
        }

        if self.songs[self.song_index].offsets.is_empty() {
            let song_duration = self.songs[self.song_index].length;
            self.song_index += 1;
            self.offset += song_duration;
        }

        match self.stroke_type {
            StrokeType::EveryNth { n } => {
                let song = &mut self.songs[self.song_index];
                let beat = song.offsets.pop()?;
                if song.offsets.len() % n == 0 {
                    Some(vec![beat + self.offset])
                } else {
                    self.next_offset()
                }
            }
            StrokeType::Accelerate {
                start_strokes_per_beat,
                end_strokes_per_beat,
            } => {
                // lerp between start and end strokes per beat based on the percentage of the whole video
                let song = &mut self.songs[self.song_index];
                let position = self.offset + song.offsets.last().unwrap();
                let percentage = position / self.total_duration;
                let strokes_per_beat =
                    lerp(start_strokes_per_beat, end_strokes_per_beat, percentage);
                debug!(
                    "at {}% of the song, strokes per beat: {}",
                    percentage * 100.0,
                    strokes_per_beat
                );
                if strokes_per_beat >= 1.0 {
                    let beat = song.offsets.pop()?;
                    let rounded = strokes_per_beat.round() as usize;

                    if song.offsets.len() % rounded == 0 {
                        Some(vec![beat + self.offset])
                    } else {
                        self.next_offset()
                    }
                } else {
                    let num_beats = (1.0 / strokes_per_beat).round() as usize;
                    let beat = song.offsets.pop()?;
                    let beat_after =
                        song.offsets.last().copied().or_else(|| {
                            self.songs.get(self.song_index + 1).map(|s| s.offsets[0])
                        })?;
                    let beats = (0..num_beats)
                        .map(|i| {
                            let percentage = i as f32 / num_beats as f32;
                            lerp(beat, beat_after, percentage) + self.offset
                        })
                        .collect();
                    debug!("beats: {:?}", beats);
                    Some(beats)
                }
            }
        }
    }
}

pub fn create_beat_funscript(songs: Vec<Beats>, stroke_type: StrokeType) -> FunScript {
    let mut actions = vec![];

    let mut state = 0;
    let mut beat_state = BeatState::new(songs, stroke_type);
    while let Some(beats) = beat_state.next_offset() {
        for beat in beats {
            let position = (beat * 1000.0).round() as u32;
            debug!("beat at {position}ms with pos {state}");

            let action = FSPoint {
                pos: state,
                at: position,
            };
            actions.push(action);
            state = if state == 0 { 100 } else { 0 };
        }
    }

    let version = env!("CARGO_PKG_VERSION");
    FunScript {
        actions,
        metadata: Some(OFSMetadata {
            creator: format!("clip-mash v{}", version),
            ..Default::default()
        }),
        ..Default::default()
    }
}

struct FunScriptSegment<'a> {
    script: &'a FunScript,
    clip_start: u32,
    clip_end: u32,
    offset: u32,
}

fn combine_scripts(segments: Vec<FunScriptSegment>) -> FunScript {
    let mut resulting_actions = vec![];

    for segment in segments {
        let start = segment.clip_start;
        let end = segment.clip_end;
        let offset = segment.offset;

        let actions = segment
            .script
            .actions
            .iter()
            .filter(|s| s.at >= start && s.at <= end)
            .map(|a| FSPoint {
                at: (a.at - start) + offset,
                pos: a.pos,
            });
        resulting_actions.extend(actions);
    }

    let version = env!("CARGO_PKG_VERSION");
    FunScript {
        actions: resulting_actions,
        metadata: Some(OFSMetadata {
            creator: format!("clip-mash v{}", version),
            ..Default::default()
        }),
        ..Default::default()
    }
}

pub struct ScriptBuilder {
    api: StashApi,
}

impl ScriptBuilder {
    pub fn new(stash_api: StashApi) -> Self {
        Self { api: stash_api }
    }

    async fn fetch_funscripts(&self, videos: &[&DbVideo]) -> Result<HashMap<String, FunScript>> {
        let mut map = HashMap::new();

        for video in videos {
            if map.contains_key(&video.id) {
                continue;
            }

            let script = match video.source {
                VideoSource::Stash => {
                    let stash_id = video
                        .stash_scene_id
                        .expect("stash scenes must have a stash scene ID set");
                    self.api.get_funscript(stash_id).await
                }
                VideoSource::Download | VideoSource::Folder => {
                    let path =
                        Utf8PathBuf::from(video.file_path.clone()).with_extension("funscript");
                    info!("trying to load funscript from {}", path);
                    FunScript::load(path).await
                }
            };

            match script {
                Ok(script) => {
                    map.insert(video.id.clone(), script);
                }
                Err(e) => {
                    warn!("failed to get .funscript for scene ID {}: {}", video.id, e);
                }
            }
        }

        Ok(map)
    }

    pub async fn create_combined_funscript(
        &self,
        clips: Vec<(DbVideo, Clip)>,
    ) -> Result<FunScript> {
        let mut offset = 0;
        let mut segments = vec![];
        let videos: Vec<_> = clips
            .iter()
            .map(|(video, _)| video)
            .unique_by(|v| v.id.as_str())
            .collect();
        let funscripts = self.fetch_funscripts(&videos).await?;

        for (video, clip) in clips {
            let (start, end) = clip.range_millis();
            let duration = clip.duration_millis();
            if let Some(script) = funscripts.get(&video.id) {
                segments.push(FunScriptSegment {
                    script,
                    clip_start: start,
                    clip_end: end,
                    offset,
                });
            }

            offset += duration;
        }

        Ok(combine_scripts(segments))
    }
}

#[cfg(test)]
mod test {
    use tracing_test::traced_test;

    use super::StrokeType;
    use crate::server::types::Beats;
    use crate::service::funscript::{
        combine_scripts, create_beat_funscript, FunScript, FunScriptSegment,
    };
    use crate::Result;

    #[traced_test]
    #[tokio::test]
    async fn test_create_combined_script() -> Result<()> {
        let script_1 = FunScript::load("data/funscripts/dokkaebi.funscript").await?;
        let script_2 = FunScript::load("data/funscripts/rachel-amber.funscript").await?;

        let combined = combine_scripts(vec![
            FunScriptSegment {
                script: &script_1,
                clip_start: 0,
                clip_end: 1000,
                offset: 0,
            },
            FunScriptSegment {
                script: &script_2,
                clip_start: 0,
                clip_end: 1000,
                offset: 1000,
            },
            FunScriptSegment {
                script: &script_1,
                clip_start: 1000,
                clip_end: 2000,
                offset: 2000,
            },
        ]);

        dbg!(combined);

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_create_beat_funscript_basic() {
        let beats = vec![
            Beats {
                length: 1.0,
                offsets: vec![0.0, 0.5, 1.0],
            },
            Beats {
                length: 2.0,
                offsets: vec![0.5, 1.0, 1.5, 2.0],
            },
        ];

        let script = create_beat_funscript(beats, StrokeType::EveryNth { n: 1 });

        assert_eq!(script.actions.len(), 7);
        assert_eq!(script.actions[0].pos, 0);
        assert_eq!(script.actions[0].at, 0);

        assert_eq!(script.actions[1].pos, 100);
        assert_eq!(script.actions[1].at, 500);

        assert_eq!(script.actions[2].pos, 0);
        assert_eq!(script.actions[2].at, 1000);

        assert_eq!(script.actions[3].pos, 100);
        assert_eq!(script.actions[3].at, 1500);

        assert_eq!(script.actions[4].pos, 0);
        assert_eq!(script.actions[4].at, 2000);

        assert_eq!(script.actions[5].pos, 100);
        assert_eq!(script.actions[5].at, 2500);

        assert_eq!(script.actions[6].pos, 0);
        assert_eq!(script.actions[6].at, 3000);
    }

    #[traced_test]
    #[test]
    fn test_create_beat_funscript_accelerate() {
        let len1 = 8.0_f32;
        let len2 = 12.0_f32;
        let beats = vec![
            Beats {
                length: len1,
                offsets: (0..(len1 as usize)).map(|i| i as f32).collect(),
            },
            Beats {
                length: len2,
                offsets: (0..(len2 as usize)).map(|i| i as f32).collect(),
            },
        ];

        let _script = create_beat_funscript(
            beats,
            StrokeType::Accelerate {
                start_strokes_per_beat: 1.0,
                end_strokes_per_beat: 0.25,
            },
        );
    }
}
