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
            id: MarkerId::LocalFile(5),
            start_time: 0.0,
            end_time: 36.153941,
            index_within_video: 0,
            video_id: VideoId::LocalFile("2H0r8zLH".into()),
            title: "Handjiob".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(5),
                    video_id: "2H0r8zLH".into(),
                    start_time: 0.0,
                    end_time: 36.153941,
                    title: "Handjiob".into(),
                    file_path: "/home/martin/videos/[ent duke] Widowmaker HJ.mp4".into(),
                    index_within_video: 0,
                },
            },
        },
        Marker {
            id: MarkerId::LocalFile(2),
            start_time: 0.0,
            end_time: 146.014932,
            index_within_video: 0,
            video_id: VideoId::LocalFile("PxTxOTfX".into()),
            title: "Doggy Style".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(2),
                    video_id: "PxTxOTfX".into(),
                    start_time: 0.0,
                    end_time: 146.014932,
                    title: "Doggy Style".into(),
                    file_path:
                        "/home/martin/videos/[HydraFXX] Tifa x Cloud Halloween (Extended).mp4"
                            .into(),
                    index_within_video: 0,
                },
            },
        },
        Marker {
            id: MarkerId::LocalFile(6),
            start_time: 0.0,
            end_time: 61.034,
            index_within_video: 0,
            video_id: VideoId::LocalFile("R43ZTr0w".into()),
            title: "Sideways".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(6),
                    video_id: "R43ZTr0w".into(),
                    start_time: 0.0,
                    end_time: 61.034,
                    title: "Sideways".into(),
                    file_path: "/home/martin/videos/4k-dokkaebi-idemi_2160p.mp4".into(),
                    index_within_video: 0,
                },
            },
        },
        Marker {
            id: MarkerId::LocalFile(4),
            start_time: 14.43444,
            end_time: 130.941,
            index_within_video: 0,
            video_id: VideoId::LocalFile("R_fDbo2f".into()),
            title: "Mating Press".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(4),
                    video_id: "R_fDbo2f".into(),
                    start_time: 14.43444,
                    end_time: 130.941,
                    title: "Mating Press".into(),
                    file_path: "/home/martin/videos/yeero d.va hard anal 1080p.mp4".into(),
                    index_within_video: 0,
                },
            },
        },
        Marker {
            id: MarkerId::LocalFile(1),
            start_time: 0.0,
            end_time: 57.126817,
            index_within_video: 0,
            video_id: VideoId::LocalFile("RrTBwBZG".into()),
            title: "Cowgirl".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(1),
                    video_id: "RrTBwBZG".into(),
                    start_time: 0.0,
                    end_time: 57.126817,
                    title: "Cowgirl".into(),
                    file_path: "/home/martin/videos/tifa-lockhart-leading-juicyneko_2160p.mp4"
                        .into(),
                    index_within_video: 0,
                },
            },
        },
        Marker {
            id: MarkerId::LocalFile(7),
            start_time: 0.0,
            end_time: 137.472,
            index_within_video: 0,
            video_id: VideoId::LocalFile("ZZtG7qbI".into()),
            title: "Doggy Style".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(7),
                    video_id: "ZZtG7qbI".into(),
                    start_time: 0.0,
                    end_time: 137.472,
                    title: "Doggy Style".into(),
                    file_path: "/home/martin/videos/(nagoonimation) Cammy Round 1.mp4".into(),
                    index_within_video: 0,
                },
            },
        },
        Marker {
            id: MarkerId::LocalFile(9),
            start_time: 0.0,
            end_time: 162.447575,
            index_within_video: 0,
            video_id: VideoId::LocalFile("bJTtKsIe".into()),
            title: "Missionary".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(9),
                    video_id: "bJTtKsIe".into(),
                    start_time: 0.0,
                    end_time: 162.447575,
                    title: "Missionary".into(),
                    file_path: "/home/martin/videos/mercy's appointment nagoonimation 720p.mp4"
                        .into(),
                    index_within_video: 0,
                },
            },
        },
        Marker {
            id: MarkerId::LocalFile(3),
            start_time: 0.0,
            end_time: 39.487,
            index_within_video: 0,
            video_id: VideoId::LocalFile("rDxeypDY".into()),
            title: "Cowgirl".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(3),
                    video_id: "rDxeypDY".into(),
                    start_time: 0.0,
                    end_time: 39.487,
                    title: "Cowgirl".into(),
                    file_path: "/home/martin/videos/[HydraFXX] Widowmaker Riding [4K].mp4".into(),
                    index_within_video: 0,
                },
            },
        },
        Marker {
            id: MarkerId::LocalFile(10),
            start_time: 0.0,
            end_time: 166.0,
            index_within_video: 0,
            video_id: VideoId::LocalFile("wkjHYedN".into()),
            title: "Sex".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(10),
                    video_id: "wkjHYedN".into(),
                    start_time: 0.0,
                    end_time: 166.0,
                    title: "Sex".into(),
                    file_path: "/home/martin/videos/(Hydrafxx) Rachel Amber.mp4".into(),
                    index_within_video: 0,
                },
            },
        },
        Marker {
            id: MarkerId::LocalFile(8),
            start_time: 0.0,
            end_time: 34.597007,
            index_within_video: 0,
            video_id: VideoId::LocalFile("yObK_Z7p".into()),
            title: "Sideways".into(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    rowid: Some(8),
                    video_id: "yObK_Z7p".into(),
                    start_time: 0.0,
                    end_time: 34.597007,
                    title: "Sideways".into(),
                    file_path: "/home/martin/videos/tifa-spooning-juicyneko_2160p.mp4".into(),
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
