use std::sync::atomic::{AtomicI64, Ordering};

use fake::faker::filesystem::en::FilePath;
use fake::faker::lorem::en::Sentence;
use fake::{Fake, Faker};
use lazy_static::lazy_static;

use super::Marker;
use crate::data::database::{CreateMarker, Database, DbMarker, DbVideo, LocalVideoSource};
use crate::service::{MarkerId, MarkerInfo, VideoId};
use crate::util::generate_id;
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
            title: "Blowjob".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(2),
                    video_id: "Rtdyb1xW".into(),
                    start_time: 19.178596,
                    end_time: 130.772832,
                    title: "Blowjob".into(),
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
            title: "Doggy Style".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(3),
                    video_id: "ejS9HLKF".into(),
                    start_time: 0.0,
                    end_time: 111.389977,
                    title: "Doggy Style".into(),
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
            title: "Doggy Style".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(10),
                    video_id: "D2FF-fJW".into(),
                    start_time: 0.0,
                    end_time: 39.487,
                    title: "Doggy Style".into(),
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
            title: "Blowjob".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(7),
                    video_id: "fZB8OPxc".into(),
                    start_time: 0.0,
                    end_time: 36.055767,
                    title: "Blowjob".into(),
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
            title: "Cowgirl".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(4),
                    video_id: "EqF5ShQY".into(),
                    start_time: 0.0,
                    end_time: 57.77,
                    title: "Cowgirl".into(),
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
            title: "Cowgirl".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(9),
                    video_id: "6P3h5aSl".into(),
                    start_time: 0.0,
                    end_time: 60.996935,
                    title: "Cowgirl".into(),
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
            title: "Cowgirl".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(5),
                    video_id: "peso3Tzd".into(),
                    start_time: 0.0,
                    end_time: 34.597007,
                    title: "Cowgirl".into(),
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
            title: "Blowjob".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(6),
                    video_id: "MJxGMsjP".into(),
                    start_time: 0.0,
                    end_time: 137.472,
                    title: "Blowjob".into(),
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
            title: "Reverse Cowgirl".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(8),
                    video_id: "mCg07LPG".into(),
                    start_time: 0.0,
                    end_time: 165.368725,
                    title: "Reverse Cowgirl".into(),
                    file_path: "/videos/(Hydrafxx) Rachel Amber.mp4".into(),
                    index_within_video: 0,
                },
            },
        },
    ]
}

pub fn other_markers() -> Vec<Marker> {
    vec![
        Marker {
            id: MarkerId::LocalFile(
                17,
            ),
            start_time: 0.0,
            end_time: 137.472,
            index_within_video: 0,
            video_id: VideoId::LocalFile(
                "A-JRTNFn".into(),
            ),
            title: "Doggy Style".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(
                        17,
                    ),
                    video_id: "A-JRTNFn".into(),
                    start_time: 0.0,
                    end_time: 137.472,
                    title: "Doggy Style".into(),
                    file_path: "/Users/martin/MEGAsync Downloads/testdir/testdir 2/(nagoonimation) Cammy Round 1.mp4".into(),
                    index_within_video: 0,
                },
            },
        },
        Marker {
            id: MarkerId::LocalFile(
                14,
            ),
            start_time: 0.0,
            end_time: 56.441634,
            index_within_video: 0,
            video_id: VideoId::LocalFile(
                "F1JU9FvO".into(),
            ),
            title: "Cowgril".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(
                        14,
                    ),
                    video_id: "F1JU9FvO".into(),
                    start_time: 0.0,
                    end_time: 56.441634,
                    title: "Cowgril".into(),
                    file_path: "/Users/martin/MEGAsync Downloads/tifa-lockhart-leading-juicyneko_2160p.mp4".into(),
                    index_within_video: 0,
                },
            },
        },
        Marker {
            id: MarkerId::LocalFile(
                11,
            ),
            start_time: 0.0,
            end_time: 162.172677,
            index_within_video: 0,
            video_id: VideoId::LocalFile(
                "ZY_BXNn2".into(),
            ),
            title: "Sex".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(
                        11,
                    ),
                    video_id: "ZY_BXNn2".into(),
                    start_time: 0.0,
                    end_time: 162.172677,
                    title: "Sex".into(),
                    file_path: "/Users/martin/MEGAsync Downloads/(Hydrafxx) Rachel Amber.mp4".into(),
                    index_within_video: 0,
                },
            },
        },
        Marker {
            id: MarkerId::LocalFile(
                15,
            ),
            start_time: 0.0,
            end_time: 39.0,
            index_within_video: 0,
            video_id: VideoId::LocalFile(
                "bZInxWGc".into(),
            ),
            title: "Riding".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(
                        15,
                    ),
                    video_id: "bZInxWGc".into(),
                    start_time: 0.0,
                    end_time: 39.0,
                    title: "Riding".into(),
                    file_path: "/Users/martin/MEGAsync Downloads/[HydraFXX] Widowmaker Riding [4K].mp4".into(),
                    index_within_video: 0,
                },
            },
        },
        Marker {
            id: MarkerId::LocalFile(
                13,
            ),
            start_time: 0.0,
            end_time: 127.395227,
            index_within_video: 0,
            video_id: VideoId::LocalFile(
                "fSiovaad".into(),
            ),
            title: "Doggy Style".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(
                        13,
                    ),
                    video_id: "fSiovaad".into(),
                    start_time: 0.0,
                    end_time: 127.395227,
                    title: "Doggy Style".into(),
                    file_path: "/Users/martin/MEGAsync Downloads/[HydraFXX] Tifa x Cloud Halloween (Extended).mp4".into(),
                    index_within_video: 0,
                },
            },
        },
        Marker {
            id: MarkerId::LocalFile(
                12,
            ),
            start_time: 7.621721,
            end_time: 116.355262,
            index_within_video: 0,
            video_id: VideoId::LocalFile(
                "g2DxP1Bl".into(),
            ),
            title: "Blowjob".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(
                        12,
                    ),
                    video_id: "g2DxP1Bl".into(),
                    start_time: 7.621721,
                    end_time: 116.355262,
                    title: "Blowjob".into(),
                    file_path: "/Users/martin/MEGAsync Downloads/Resident Evil 3 - Jill Sloppy Blowjob - Bulging Senpai.mp4".into(),
                    index_within_video: 0,
                },
            },
        },
        Marker {
            id: MarkerId::LocalFile(
                16,
            ),
            start_time: 0.0,
            end_time: 61.0,
            index_within_video: 0,
            video_id: VideoId::LocalFile(
                "uBK2K4hS".into(),
            ),
            title: "Sideways".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(
                        16,
                    ),
                    video_id: "uBK2K4hS".into(),
                    start_time: 0.0,
                    end_time: 61.0,
                    title: "Sideways".into(),
                    file_path: "/Users/martin/MEGAsync Downloads/4k-dokkaebi-idemi_2160p.mp4".into(),
                    index_within_video: 0,
                },
            },
        },
    ]
}

pub fn create_marker(title: &str, start_time: f64, end_time: f64, index: usize) -> Marker {
    lazy_static! {
        static ref ID: AtomicI64 = AtomicI64::new(0);
    }

    let id = ID.fetch_add(1, Ordering::SeqCst);

    Marker {
        id: MarkerId::LocalFile(id),
        start_time,
        end_time,
        index_within_video: index,
        video_id: VideoId::LocalFile(generate_id()),
        title: title.to_string(),
        info: MarkerInfo::LocalFile {
            marker: DbMarker {
                start_time,
                end_time,
                rowid: None,
                title: title.to_string(),
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
        id: generate_id(),
        interactive: false,
        source: LocalVideoSource::Folder,
    };
    db.persist_video(expected.clone()).await?;
    Ok(expected)
}

pub async fn persist_video_with_source(db: &Database, source: LocalVideoSource) -> Result<DbVideo> {
    let video = DbVideo {
        file_path: FilePath().fake(),
        id: generate_id(),
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
