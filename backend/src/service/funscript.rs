use camino::{Utf8Path, Utf8PathBuf};
use clip_mash_types::{Beats, Clip, StrokeType};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{info, warn};

use super::Video;
use crate::data::stash_api::StashApi;
use crate::service::VideoInfo;
use crate::util::lerp;
use crate::Result;

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
                info!(
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
                    info!("beats: {:?}", beats);
                    Some(beats)
                }
            }
        }
    }
}

pub fn create_beat_script(songs: Vec<Beats>, stroke_type: StrokeType) -> FunScript {
    let mut actions = vec![];

    let mut state = 0;
    let mut beat_state = BeatState::new(songs, stroke_type);
    while let Some(beats) = beat_state.next_offset() {
        for beat in beats {
            let position = (beat * 1000.0).round() as u32;
            info!("beat at {position}ms with pos {state}");

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
                    warn!(
                        "failed to get .funscript for scene ID {}: {}",
                        clip.video_id, e
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

        info!("generated funscript with {} actions", script.actions.len());

        Ok(script)
    }
}

#[cfg(test)]
mod test {
    use clip_mash_types::Beats;
    use tracing_test::traced_test;

    use super::StrokeType;
    use crate::service::funscript::create_beat_script;

    #[traced_test]
    #[test]
    fn test_create_beat_script_basic() {
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

        let script = create_beat_script(beats, StrokeType::EveryNth { n: 1 });

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
    fn test_create_beat_script_accelerate() {
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

        let _script = create_beat_script(
            beats,
            StrokeType::Accelerate {
                start_strokes_per_beat: 1.0,
                end_strokes_per_beat: 0.25,
            },
        );
    }
}
