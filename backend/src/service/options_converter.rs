use std::collections::{HashMap, HashSet};

use crate::data::database::music::DbSong;
use crate::data::database::videos::DbVideo;
use crate::data::database::Database;
use crate::server::types::{Clip, CreateClipsBody, CreateVideoBody, SelectedMarker};
use crate::service::clip::CreateClipsOptions;
use crate::service::generator::CompilationOptions;
use crate::service::Marker;
use crate::Result;

pub struct OptionsConverterService {
    db: Database,
}

impl OptionsConverterService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn convert_clips(&self, clips: Vec<Clip>) -> Result<Vec<(DbVideo, Clip)>> {
        let all_video_ids: HashSet<_> = clips.iter().map(|c| c.video_id.as_str()).collect();
        let all_video_ids: Vec<_> = all_video_ids.into_iter().collect();
        let videos: HashMap<_, _> = self
            .db
            .videos
            .get_videos_by_ids(&all_video_ids)
            .await?
            .into_iter()
            .map(|v| (v.id.clone(), v))
            .collect();

        let mut results = vec![];
        for clip in &clips {
            let video = videos.get(&clip.video_id).unwrap().clone();
            results.push((video, clip.clone()));
        }
        Ok(results)
    }

    fn convert_selected_markers(&self, markers: Vec<SelectedMarker>) -> Vec<Marker> {
        let mut results = vec![];

        for selected_marker in markers {
            let (start_time, end_time) = selected_marker.selected_range;
            results.push(Marker {
                start_time,
                end_time,
                id: selected_marker.id,
                video_id: selected_marker.video_id,
                index_within_video: selected_marker.index_within_video,
                title: selected_marker.title,
                loops: selected_marker.loops,
                source: selected_marker.source,
            })
        }

        results
    }

    pub async fn convert_compilation_options(
        &self,
        body: CreateVideoBody,
    ) -> Result<CompilationOptions> {
        let songs = self.resolve_songs(&body.song_ids).await?;
        let video_ids = body
            .selected_markers
            .iter()
            .map(|m| m.video_id.as_str())
            .collect::<HashSet<_>>();
        let video_ids = video_ids.into_iter().collect::<Vec<_>>();
        let videos = self.db.videos.get_videos_by_ids(&video_ids).await?;

        Ok(CompilationOptions {
            video_id: body.video_id,
            clips: body.clips,
            markers: self.convert_selected_markers(body.selected_markers),
            output_resolution: body.output_resolution,
            output_fps: body.output_fps,
            file_name: body.file_name,
            songs,
            music_volume: body.music_volume.unwrap_or(0.0),
            video_codec: body.video_codec,
            encoding_effort: body.encoding_effort,
            video_quality: body.video_quality,
            videos,
            padding: body.padding.unwrap_or_default(),
            force_re_encode: body.force_re_encode,
            include_original_file_name: body.include_original_file_name,
        })
    }

    async fn resolve_songs(&self, song_ids: &[i64]) -> Result<Vec<DbSong>> {
        self.db.music.get_songs(song_ids).await
    }

    pub async fn convert_clip_options(&self, body: CreateClipsBody) -> Result<CreateClipsOptions> {
        Ok(CreateClipsOptions {
            markers: self.convert_selected_markers(body.markers),
            seed: body.seed,
            clip_options: body.clips,
        })
    }
}
