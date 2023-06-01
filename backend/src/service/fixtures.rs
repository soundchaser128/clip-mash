use fake::faker::filesystem::en::FilePath;
use fake::faker::lorem::en::Sentence;
use fake::{Fake, Faker};
use nanoid::nanoid;

use super::Marker;
use crate::data::database::{CreateMarker, Database, DbMarker, DbVideo, LocalVideoSource};
use crate::service::{MarkerId, MarkerInfo, VideoId};
use crate::Result;

pub fn markers() -> Vec<Marker> {
    vec![
        Marker {
            id: MarkerId::LocalFile(1),
            start_time: 0.0,
            end_time: 171.7162,
            index_within_video: 0,
            video_id: VideoId::LocalFile("go8DbGFE".into()),
            title: "Blowjob".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(1),
                    video_id: "go8DbGFE".into(),
                    start_time: 0.0,
                    end_time: 171.7162,
                    title: "Blowjob".into(),
                    file_path: "/videos/Resident Evil 3 - Jill Sloppy Blowjob - Bulging Senpai.mp4"
                        .into(),
                    index_within_video: 0,
                },
            },
        },
        Marker {
            id: MarkerId::LocalFile(2),
            start_time: 19.178596,
            end_time: 130.772832,
            index_within_video: 0,
            video_id: VideoId::LocalFile("Rtdyb1xW".into()),
            title: "1".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(2),
                    video_id: "Rtdyb1xW".into(),
                    start_time: 19.178596,
                    end_time: 130.772832,
                    title: "1".into(),
                    file_path: "/videos/Black Widow Casting (Full ver.) [3104140].mp4".into(),
                    index_within_video: 0,
                },
            },
        },
        Marker {
            id: MarkerId::LocalFile(3),
            start_time: 0.0,
            end_time: 111.389977,
            index_within_video: 0,
            video_id: VideoId::LocalFile("ejS9HLKF".into()),
            title: "sex".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(3),
                    video_id: "ejS9HLKF".into(),
                    start_time: 0.0,
                    end_time: 111.389977,
                    title: "sex".into(),
                    file_path: "/videos/[HydraFXX] Tifa x Cloud Halloween (Extended).mp4".into(),
                    index_within_video: 0,
                },
            },
        },
        Marker {
            id: MarkerId::LocalFile(10),
            start_time: 0.0,
            end_time: 39.487,
            index_within_video: 0,
            video_id: VideoId::LocalFile("D2FF-fJW".into()),
            title: "sex".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(10),
                    video_id: "D2FF-fJW".into(),
                    start_time: 0.0,
                    end_time: 39.487,
                    title: "sex".into(),
                    file_path: "/videos/[HydraFXX] Widowmaker Riding [4K].mp4".into(),
                    index_within_video: 0,
                },
            },
        },
        Marker {
            id: MarkerId::LocalFile(7),
            start_time: 0.0,
            end_time: 36.055767,
            index_within_video: 0,
            video_id: VideoId::LocalFile("fZB8OPxc".into()),
            title: "sex".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(7),
                    video_id: "fZB8OPxc".into(),
                    start_time: 0.0,
                    end_time: 36.055767,
                    title: "sex".into(),
                    file_path: "/videos/[ent duke] Widowmaker HJ.mp4".into(),
                    index_within_video: 0,
                },
            },
        },
        Marker {
            id: MarkerId::LocalFile(4),
            start_time: 0.0,
            end_time: 57.77,
            index_within_video: 0,
            video_id: VideoId::LocalFile("EqF5ShQY".into()),
            title: "asdasd".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(4),
                    video_id: "EqF5ShQY".into(),
                    start_time: 0.0,
                    end_time: 57.77,
                    title: "asdasd".into(),
                    file_path: "/videos/tifa-lockhart-leading-juicyneko_2160p.mp4".into(),
                    index_within_video: 0,
                },
            },
        },
        Marker {
            id: MarkerId::LocalFile(9),
            start_time: 0.0,
            end_time: 60.996935,
            index_within_video: 0,
            video_id: VideoId::LocalFile("6P3h5aSl".into()),
            title: "sex".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(9),
                    video_id: "6P3h5aSl".into(),
                    start_time: 0.0,
                    end_time: 60.996935,
                    title: "sex".into(),
                    file_path: "/videos/4k-dokkaebi-idemi_2160p.mp4".into(),
                    index_within_video: 0,
                },
            },
        },
        Marker {
            id: MarkerId::LocalFile(5),
            start_time: 0.0,
            end_time: 34.597007,
            index_within_video: 0,
            video_id: VideoId::LocalFile("peso3Tzd".into()),
            title: "asd".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(5),
                    video_id: "peso3Tzd".into(),
                    start_time: 0.0,
                    end_time: 34.597007,
                    title: "asd".into(),
                    file_path: "/videos/tifa-spooning-juicyneko_2160p.mp4".into(),
                    index_within_video: 0,
                },
            },
        },
        Marker {
            id: MarkerId::LocalFile(6),
            start_time: 0.0,
            end_time: 137.472,
            index_within_video: 0,
            video_id: VideoId::LocalFile("MJxGMsjP".into()),
            title: "sex".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(6),
                    video_id: "MJxGMsjP".into(),
                    start_time: 0.0,
                    end_time: 137.472,
                    title: "sex".into(),
                    file_path: "/videos/testdir/testdir 2/(nagoonimation) Cammy Round 1.mp4".into(),
                    index_within_video: 0,
                },
            },
        },
        Marker {
            id: MarkerId::LocalFile(8),
            start_time: 0.0,
            end_time: 165.368725,
            index_within_video: 0,
            video_id: VideoId::LocalFile("mCg07LPG".into()),
            title: "sex".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(8),
                    video_id: "mCg07LPG".into(),
                    start_time: 0.0,
                    end_time: 165.368725,
                    title: "sex".into(),
                    file_path: "/videos/(Hydrafxx) Rachel Amber.mp4".into(),
                    index_within_video: 0,
                },
            },
        },
    ]
}

#[allow(unused)]
pub fn create_marker(start_time: f64, end_time: f64, index: usize) -> Marker {
    Marker {
        id: MarkerId::LocalFile(1),
        start_time,
        end_time,
        index_within_video: index,
        video_id: VideoId::LocalFile(nanoid!(8)),
        title: Faker.fake(),
        info: MarkerInfo::LocalFile {
            marker: DbMarker {
                end_time,
                start_time,
                rowid: None,
                title: Faker.fake(),
                video_id: Faker.fake(),
                file_path: FilePath().fake(),
                index_within_video: index as i64,
            },
        },
    }
}

pub fn create_marker_video_id(
    id: i64,
    start_time: f64,
    end_time: f64,
    index: usize,
    video_id: &str,
) -> Marker {
    Marker {
        id: MarkerId::LocalFile(id),
        start_time,
        end_time,
        index_within_video: index,
        video_id: VideoId::LocalFile(video_id.to_string()),
        title: Faker.fake(),
        info: MarkerInfo::LocalFile {
            marker: DbMarker {
                end_time,
                start_time,
                rowid: None,
                title: Faker.fake(),
                video_id: video_id.to_string(),
                file_path: FilePath().fake(),
                index_within_video: index as i64,
            },
        },
    }
}

pub async fn persist_video(db: &Database) -> Result<DbVideo> {
    let expected = DbVideo {
        file_path: FilePath().fake(),
        id: nanoid!(8),
        interactive: false,
        source: LocalVideoSource::Folder,
    };
    db.persist_video(expected.clone()).await?;
    Ok(expected)
}

pub async fn persist_video_with_source(db: &Database, source: LocalVideoSource) -> Result<DbVideo> {
    let video = DbVideo {
        file_path: FilePath().fake(),
        id: nanoid!(8),
        interactive: false,
        source,
    };
    db.persist_video(video.clone()).await?;
    Ok(video)
}

pub async fn persist_marker(
    db: &Database,
    video_id: &str,
    index: i64,
    start: f64,
    end: f64,
) -> Result<DbMarker> {
    let marker = CreateMarker {
        video_id: video_id.to_string(),
        start,
        end,
        index_within_video: index,
        title: Sentence(5..8).fake(),
    };
    db.persist_marker(marker).await
}
