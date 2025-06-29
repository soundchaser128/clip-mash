use std::time::{Instant, SystemTime};

use camino::{Utf8Path, Utf8PathBuf};
use color_eyre::eyre::{bail, eyre};
use futures::future;
use itertools::Itertools;
use serde::Deserialize;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;
use tokio::task::spawn_blocking;
use tracing::{Level, debug, info, warn};
use url::Url;
use utoipa::ToSchema;
use walkdir::WalkDir;

use super::commands::ffmpeg::FfmpegLocation;
use super::directories::Directories;
use crate::Result;
use crate::data::database::Database;
use crate::data::database::markers::DbMarker;
use crate::data::database::performers::CreatePerformer;
use crate::data::database::videos::{CreateVideo, DbVideo, VideoSource, VideoUpdate};
use crate::data::stash_api::{MarkerLike, StashApi, StashMarker};
use crate::helpers::parallelize;
use crate::helpers::random::generate_id;
// use crate::server::handlers::AppState;
use crate::service::commands::{YtDlp, YtDlpOptions, ffprobe};
use crate::service::directories::FolderType;
use crate::service::preview_image::PreviewGenerator;
use crate::types::{CreateMarker, ListVideoDto, UpdateMarker};

const VIDEO_EXTENSIONS: &[&str] = &["mp4", "m4v", "webm"];

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum AddVideosRequest {
    Local {
        path: String,
        recurse: bool,
        tags: Option<Vec<String>>,
    },
    Download {
        urls: Vec<String>,
        tags: Option<Vec<String>>,
    },
    #[serde(rename_all = "camelCase")]
    Stash { scene_ids: Vec<i64> },
}

fn tags_to_string(tags: &[String]) -> String {
    serde_json::to_string(tags).expect("tags must be serializable")
}

pub struct VideoService {
    pub database: Database,
    pub directories: Directories,
    pub ffmpeg_location: FfmpegLocation,
    pub stash_api: StashApi,
    pub preview_generator: PreviewGenerator,
}

impl VideoService {
    async fn gather_files(&self, path: Utf8PathBuf, recurse: bool) -> Result<Vec<Utf8PathBuf>> {
        spawn_blocking(move || {
            let files = WalkDir::new(path)
                .max_depth(if recurse { usize::MAX } else { 1 })
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
                .map(|e| Utf8PathBuf::from_path_buf(e.into_path()).expect("not a utf8 path"))
                .filter(|e| {
                    if let Some(extension) = e.extension() {
                        VIDEO_EXTENSIONS.contains(&extension)
                    } else {
                        false
                    }
                })
                .collect();

            Ok(files)
        })
        .await?
    }

    async fn add_local_video(&self, path: Utf8PathBuf, tags: &[String]) -> Result<Option<DbVideo>> {
        let video_exists = self
            .database
            .videos
            .video_exists_by_path(path.as_str())
            .await?;
        info!("video at path '{path}' exists: {video_exists}");
        if !video_exists {
            let interactive = path.with_extension("funscript").is_file();
            let ffprobe = ffprobe(path.as_str(), &self.ffmpeg_location).await;
            if let Err(e) = ffprobe {
                warn!("skipping video {path} because ffprobe failed with error {e}");
                return Ok(None);
            }
            let ffprobe = ffprobe.unwrap();
            let duration = ffprobe.duration();
            let id = generate_id();
            let image_path = self
                .preview_generator
                .generate_preview(&id, &path, duration.map_or(0.0, |d| d / 2.0))
                .await?;
            let file_created = path.metadata()?.created().ok().map(|time| {
                time.duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64
            });

            let create_video = CreateVideo {
                id,
                file_path: path.to_string(),
                interactive,
                source: VideoSource::Folder,
                duration: duration.unwrap_or_default(),
                video_preview_image: Some(image_path.to_string()),
                stash_scene_id: None,
                title: Some(path.file_stem().unwrap().to_string()),
                tags: Some(tags_to_string(tags)),
                created_on: file_created,
            };
            info!("inserting new video {create_video:#?}");
            let video = self.database.videos.persist_video(&create_video).await?;
            self.database.ffprobe.set_info(&video.id, &ffprobe).await?;
            Ok(Some(video))
        } else {
            Ok(None)
        }
    }

    async fn add_new_local_videos(
        &self,
        path: impl AsRef<Utf8Path>,
        recurse: bool,
        tags: Vec<String>,
    ) -> Result<Vec<DbVideo>> {
        let start = Instant::now();
        let entries = self.gather_files(path.as_ref().to_owned(), recurse).await?;
        debug!("found files {entries:?} (recurse = {recurse})");
        let futures = entries
            .into_iter()
            .map(|path| self.add_local_video(path, &tags));
        let videos = parallelize(futures)
            .await
            .into_iter()
            .flatten()
            .flatten()
            .collect_vec();
        let elapsed = start.elapsed();
        info!(
            "added {videos} videos in {elapsed:?}",
            videos = videos.len(),
            elapsed = elapsed
        );
        Ok(videos)
    }

    async fn download_video(&self, url: Url) -> Result<(String, Utf8PathBuf)> {
        info!("downloading video {url}");
        let downloader = YtDlp::new(self.directories.clone());
        let options = YtDlpOptions {
            url,
            extract_audio: false,
            destination: FolderType::DownloadedVideo,
        };
        let result = downloader.run(&options, &self.ffmpeg_location).await?;
        Ok((result.generated_id, result.downloaded_file))
    }

    async fn persist_stash_marker(&self, marker: StashMarker, video: &DbVideo) -> Result<DbMarker> {
        let scene_id: i64 = marker.scene_id.parse()?;
        let stream_url = self.stash_api.get_stream_url(scene_id);

        let preview_path = self
            .preview_generator
            .generate_preview(&video.id, &stream_url, marker.start)
            .await?;
        let create_marker = CreateMarker {
            video_id: video.id.clone(),
            start: marker.start,
            end: marker.end,
            title: marker.primary_tag,
            index_within_video: marker.index_within_video as i64,
            preview_image_path: Some(preview_path.to_string()),
            video_interactive: video.interactive,
            created_on: Some(marker.created_on),
            marker_stash_id: Some(marker.id.parse()?),
        };
        self.database.markers.create_new_marker(create_marker).await
    }

    async fn persist_downloaded_video(
        &self,
        id: String,
        path: Utf8PathBuf,
        tags: Vec<String>,
    ) -> Result<DbVideo> {
        let ffprobe = ffprobe(path.as_str(), &self.ffmpeg_location).await?;
        let duration = ffprobe.duration();
        let image_path = self
            .preview_generator
            .generate_preview(&id, &path, duration.map_or(0.0, |d| d / 2.0))
            .await?;

        let video = CreateVideo {
            id,
            file_path: path.as_str().to_string(),
            interactive: false,
            source: VideoSource::Download,
            duration: duration.unwrap_or_default(),
            video_preview_image: Some(image_path.to_string()),
            stash_scene_id: None,
            title: path.file_stem().map(String::from),
            tags: Some(tags_to_string(&tags)),
            created_on: None,
        };
        info!("persisting downloaded video {video:#?}");

        let video = self.database.videos.persist_video(&video).await?;
        self.database.ffprobe.set_info(&video.id, &ffprobe).await?;
        Ok(video)
    }

    async fn persist_stash_videos(&self, scene_ids: Vec<i64>) -> Result<Vec<DbVideo>> {
        let stash_config = self.database.settings.fetch().await?.stash;
        info!("adding videos from stash with IDs {scene_ids:?}");

        let scenes = self.stash_api.find_scenes_by_ids(scene_ids).await?;
        let stash_markers: Vec<_> = scenes
            .iter()
            .flat_map(|s| StashMarker::from_scene(s.clone(), stash_config.api_key.as_deref()))
            .collect();
        let scene_urls = scenes
            .iter()
            .map(|s| self.stash_api.get_stream_url(s.id.parse().unwrap()))
            .collect_vec();
        let ffprobe_info = parallelize(
            scene_urls
                .into_iter()
                .map(|url| ffprobe(url, &self.ffmpeg_location))
                .collect_vec(),
        )
        .await;

        let create_videos: Vec<_> = scenes
            .into_iter()
            .zip(ffprobe_info)
            .map(|(scene, ffprobe)| {
                let title = scene
                    .title
                    .filter(|t| !t.is_empty())
                    .or(scene.files.get(0).map(|f| f.basename.clone()));
                info!("inserting video from stash with title {title:?}");
                let stash_id = scene.id.parse().unwrap();
                let created_on = OffsetDateTime::parse(&scene.created_at, &Rfc3339)
                    .map(|time| time.unix_timestamp())
                    .ok();
                let mut tags = scene.tags.iter().map(|t| t.name.as_str()).collect_vec();
                tags.extend(scene.performers.iter().map(|p| p.name.as_str()));
                if let Some(studio) = &scene.studio {
                    tags.push(studio.name.as_str());
                }
                let tags = serde_json::to_string(&tags).expect("tags must be serializable");
                let id = generate_id();
                let ffprobe_info = ffprobe.ok();

                let create_video = CreateVideo {
                    id,
                    file_path: self.stash_api.get_stream_url(scene.id.parse().unwrap()),
                    interactive: scene.interactive,
                    source: VideoSource::Stash,
                    duration: scene.files[0].duration,
                    video_preview_image: Some(self.stash_api.get_screenshot_url(stash_id)),
                    stash_scene_id: Some(scene.id.parse().unwrap()),
                    title,
                    tags: Some(tags),
                    created_on,
                };

                let performers: Vec<_> = scene
                    .performers
                    .into_iter()
                    .map(|p| CreatePerformer {
                        name: p.name,
                        image_url: p.image_path,
                        stash_id: Some(p.id),
                        gender: p.gender.map(From::from),
                    })
                    .collect();

                (create_video, ffprobe_info, performers)
            })
            .collect();

        let mut videos = vec![];
        for (create_video, ffprobe, performers) in &create_videos {
            let id = create_video.id.clone();
            let result = self.database.videos.persist_video(create_video).await?;
            if let Some(ffprobe) = ffprobe {
                self.database.ffprobe.set_info(&id, &ffprobe).await?;
            }
            self.database
                .performers
                .insert_for_video(performers, &id)
                .await?;

            videos.push(result);
        }

        for marker in stash_markers {
            let scene_id = marker.scene_id.parse()?;
            let video = videos
                .iter()
                .find(|v| v.stash_scene_id == Some(scene_id))
                .expect("video must exist");
            self.persist_stash_marker(marker, &video).await?;
        }

        Ok(videos)
    }

    pub async fn add_videos(&self, request: AddVideosRequest) -> Result<Vec<DbVideo>> {
        match request {
            AddVideosRequest::Local {
                path,
                recurse,
                tags,
            } => {
                self.add_new_local_videos(path, recurse, tags.unwrap_or_default())
                    .await
            }
            AddVideosRequest::Download { urls, tags } => {
                let futures = future::try_join_all(urls.into_iter().map(|url| {
                    let tags = tags.clone().unwrap_or_default();
                    async move {
                        let (id, path) = self.download_video(url.parse()?).await?;
                        self.persist_downloaded_video(id, path, tags).await
                    }
                }));

                futures.await
            }
            AddVideosRequest::Stash { scene_ids } => self.persist_stash_videos(scene_ids).await,
        }
    }

    pub async fn cleanup_videos(&self) -> Result<u32> {
        self.database.videos.cleanup_videos().await
    }

    fn has_overlap<M: MarkerLike>(&self, stash_markers: &[M], start: f64, end: f64) -> bool {
        stash_markers.iter().any(|m| {
            let range = m.start()..m.end();
            range.contains(&start) || range.contains(&end)
        })
    }

    pub async fn merge_stash_scene(&self, video_id: &str) -> Result<ListVideoDto> {
        let stash_config = self.database.settings.fetch().await?.stash;

        let mut video = self
            .database
            .videos
            .get_video(video_id)
            .await?
            .ok_or_else(|| eyre!("video not found"))?;

        if let Some(stash_scene_id) = video.stash_scene_id {
            let scene = self.stash_api.find_scene(stash_scene_id).await?;
            let scene_markers =
                StashMarker::from_scene(scene.clone(), stash_config.api_key.as_deref());

            let new_title = scene
                .title
                .filter(|t| !t.is_empty() && video.video_title.as_deref() != Some(t));
            info!("setting video title to {new_title:?} (not changing if None)");

            let mut new_tags = scene.tags.iter().map(|t| t.name.clone()).collect_vec();
            new_tags.extend(scene.performers.iter().map(|p| p.name.clone()));
            let new_tags = Some(new_tags).filter(|t| !t.is_empty());

            video.video_tags =
                Some(serde_json::to_string(&new_tags).expect("tags must be serializable"));
            video.video_title = new_title.clone();

            self.database
                .videos
                .update_video(
                    &video.id,
                    VideoUpdate {
                        title: new_title,
                        tags: new_tags,
                    },
                )
                .await?;

            let db_markers = self
                .database
                .markers
                .get_markers_for_video(&video.id)
                .await?;

            let mut new_markers = 0;
            let stored_ids: Vec<_> = db_markers.iter().flat_map(|m| m.marker_stash_id).collect();
            for marker in &scene_markers {
                let stash_id = marker.id.parse()?;
                if !stored_ids.contains(&stash_id)
                    && !self.has_overlap(&db_markers, marker.start, marker.end)
                {
                    info!("creating marker {marker:?} in database");
                    self.persist_stash_marker(marker.clone(), &video).await?;
                    new_markers += 1;
                } else {
                    info!("marker {stash_id} already exists, updating it");
                    let db_id = db_markers
                        .iter()
                        .find(|m| m.marker_stash_id == Some(stash_id))
                        .unwrap()
                        .rowid
                        .unwrap();

                    // Format times as MM:SS for easier reading and logging for updating markers
                    if tracing::enabled!(Level::DEBUG) {
                        let format_time = |seconds: f64| {
                            let total_seconds = seconds.round() as i64;
                            let minutes = total_seconds / 60;
                            let seconds = total_seconds % 60;
                            format!("{:02}:{:02}", minutes, seconds)
                        };

                        debug!(
                            "Marker #{}: Updating from {}→{} ({} to {}) to {}→{} ({} to {})",
                            db_id,
                            db_markers
                                .iter()
                                .find(|m| m.marker_stash_id == Some(stash_id))
                                .unwrap()
                                .start_time,
                            db_markers
                                .iter()
                                .find(|m| m.marker_stash_id == Some(stash_id))
                                .unwrap()
                                .end_time,
                            format_time(
                                db_markers
                                    .iter()
                                    .find(|m| m.marker_stash_id == Some(stash_id))
                                    .unwrap()
                                    .start_time
                            ),
                            format_time(
                                db_markers
                                    .iter()
                                    .find(|m| m.marker_stash_id == Some(stash_id))
                                    .unwrap()
                                    .end_time
                            ),
                            marker.start,
                            marker.end,
                            format_time(marker.start),
                            format_time(marker.end)
                        );
                    }

                    self.database
                        .markers
                        .update_marker(
                            db_id,
                            UpdateMarker {
                                start: Some(marker.start),
                                end: Some(marker.end),
                                title: Some(marker.primary_tag.clone()),
                                ..Default::default()
                            },
                        )
                        .await?;
                }
            }
            let marker_count = db_markers.len() + new_markers;

            // for every marker in the local database, but not on stash: create it in stash
            for marker in db_markers {
                if marker.marker_stash_id.is_none()
                    && !self.has_overlap(&scene_markers, marker.start_time, marker.end_time)
                {
                    info!("creating marker {marker:?} in stash");
                    let rowid = marker.rowid.unwrap();
                    let scene_id = video.stash_scene_id.unwrap();
                    let stash_marker_id = self
                        .stash_api
                        .add_marker(marker, scene_id.to_string())
                        .await?;
                    self.database
                        .markers
                        .update_marker(
                            rowid,
                            UpdateMarker {
                                stash_marker_id: Some(stash_marker_id),
                                ..Default::default()
                            },
                        )
                        .await?;
                }
            }

            Ok(ListVideoDto {
                video: video.into(),
                marker_count,
            })
        } else {
            bail!("video is not from stash")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Result;
    use crate::service::fixtures;

    #[tokio::test]
    #[ignore]
    async fn test_add_local_videos() -> Result<()> {
        let _server = fixtures::stash_mock_server().await;

        Ok(())
    }
}
