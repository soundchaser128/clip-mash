use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, Ordering};

use camino::Utf8Path;
use fake::faker::filesystem::en::FilePath;
use fake::faker::lorem::en::{Sentence, Word};
use fake::{Fake, Faker};
use graphql_parser::query::{Definition, OperationDefinition};
use lazy_static::lazy_static;
use serde::Deserialize;
use serde_json::Value;
use tokio::process::Command;
use tracing::info;
use wiremock::matchers::{method, path};
use wiremock::{Match, Mock, MockServer, ResponseTemplate};

use super::Marker;
use crate::Result;
use crate::data::database::markers::DbMarker;
use crate::data::database::performers::{CreatePerformer, DbPerformer, Gender};
use crate::data::database::videos::{CreateVideo, DbVideo, VideoSource};
use crate::data::database::{Database, unix_timestamp_now};
use crate::helpers::random::generate_id;
use crate::types::{Beats, CreateMarker};

#[derive(Debug, Deserialize)]
struct GraphQlQuery {
    query: String,
    #[allow(unused)]
    variables: Option<HashMap<String, Value>>,
}

pub struct GraphQlQueryMatcher<'a> {
    query: &'a str,
}

pub fn graphql_query(query: &str) -> GraphQlQueryMatcher {
    GraphQlQueryMatcher { query }
}

impl<'a> Match for GraphQlQueryMatcher<'a> {
    fn matches(&self, request: &wiremock::Request) -> bool {
        let query: GraphQlQuery =
            serde_json::from_slice(&request.body).expect("failed to parse query json");
        let document = graphql_parser::parse_query::<&str>(&query.query)
            .expect("failed to parse graphql query");

        for def in document.definitions {
            if let Definition::Operation(OperationDefinition::Query(query)) = def {
                if query.name == Some(self.query) {
                    return true;
                }
            }
        }

        false
    }
}

pub async fn stash_mock_server() -> MockServer {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/graphql"))
        .and(graphql_query("FindScenesQuery"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    server
}

pub fn markers() -> Vec<Marker> {
    vec![
        Marker {
            loops: 1,
            source: VideoSource::Folder,
            id: 1,
            start_time: 0.0,
            end_time: 171.7162,
            index_within_video: 0,
            video_id: "go8DbGFE".into(),
            title: "Blowjob".into(),
        },
        Marker {
            loops: 1,
            source: VideoSource::Folder,
            id: 2,
            start_time: 19.178596,
            end_time: 130.772832,
            index_within_video: 0,
            video_id: "Rtdyb1xW".into(),
            title: "Blowjob".into(),
        },
        Marker {
            loops: 1,
            source: VideoSource::Folder,
            id: 3,
            start_time: 0.0,
            end_time: 111.389977,
            index_within_video: 0,
            video_id: "ejS9HLKF".into(),
            title: "Doggy Style".into(),
        },
        Marker {
            loops: 1,
            source: VideoSource::Folder,
            id: 10,
            start_time: 0.0,
            end_time: 39.487,
            index_within_video: 0,
            video_id: "D2FF-fJW".into(),
            title: "Doggy Style".into(),
        },
        Marker {
            loops: 1,
            source: VideoSource::Folder,
            id: 7,
            start_time: 0.0,
            end_time: 36.055767,
            index_within_video: 0,
            video_id: "fZB8OPxc".into(),
            title: "Blowjob".into(),
        },
        Marker {
            loops: 1,
            source: VideoSource::Folder,
            id: 4,
            start_time: 0.0,
            end_time: 57.77,
            index_within_video: 0,
            video_id: "EqF5ShQY".into(),
            title: "Cowgirl".into(),
        },
        Marker {
            loops: 1,
            source: VideoSource::Folder,
            id: 9,
            start_time: 0.0,
            end_time: 60.996935,
            index_within_video: 0,
            video_id: "6P3h5aSl".into(),
            title: "Cowgirl".into(),
        },
        Marker {
            loops: 1,
            source: VideoSource::Folder,
            id: 5,
            start_time: 0.0,
            end_time: 34.597007,
            index_within_video: 0,
            video_id: "peso3Tzd".into(),
            title: "Cowgirl".into(),
        },
        Marker {
            loops: 1,
            source: VideoSource::Folder,
            id: 6,
            start_time: 0.0,
            end_time: 137.472,
            index_within_video: 0,
            video_id: "MJxGMsjP".into(),
            title: "Blowjob".into(),
        },
        Marker {
            loops: 1,
            source: VideoSource::Folder,
            id: 8,
            start_time: 0.0,
            end_time: 165.368725,
            index_within_video: 0,
            video_id: "mCg07LPG".into(),
            title: "Reverse Cowgirl".into(),
        },
    ]
}

pub fn other_markers() -> Vec<Marker> {
    vec![
        Marker {
            loops: 1,
            source: VideoSource::Folder,
            id: 5,
            start_time: 0.0,
            end_time: 36.153941,
            index_within_video: 0,
            video_id: "2H0r8zLH".into(),
            title: "Handjiob".into(),
        },
        Marker {
            loops: 1,
            source: VideoSource::Folder,
            id: 2,
            start_time: 0.0,
            end_time: 146.014932,
            index_within_video: 0,
            video_id: "PxTxOTfX".into(),
            title: "Doggy Style".into(),
        },
        Marker {
            loops: 1,
            source: VideoSource::Folder,
            id: 6,
            start_time: 0.0,
            end_time: 61.034,
            index_within_video: 0,
            video_id: "R43ZTr0w".into(),
            title: "Sideways".into(),
        },
        Marker {
            loops: 1,
            source: VideoSource::Folder,
            id: 4,
            start_time: 14.43444,
            end_time: 130.941,
            index_within_video: 0,
            video_id: "R_fDbo2f".into(),
            title: "Mating Press".into(),
        },
        Marker {
            loops: 1,
            source: VideoSource::Folder,
            id: 1,
            start_time: 0.0,
            end_time: 57.126817,
            index_within_video: 0,
            video_id: "RrTBwBZG".into(),
            title: "Cowgirl".into(),
        },
        Marker {
            loops: 1,
            source: VideoSource::Folder,
            id: 7,
            start_time: 0.0,
            end_time: 137.472,
            index_within_video: 0,
            video_id: "ZZtG7qbI".into(),
            title: "Doggy Style".into(),
        },
        Marker {
            loops: 1,
            source: VideoSource::Folder,
            id: 9,
            start_time: 0.0,
            end_time: 162.447575,
            index_within_video: 0,
            video_id: "bJTtKsIe".into(),
            title: "Missionary".into(),
        },
        Marker {
            loops: 1,
            source: VideoSource::Folder,
            id: 3,
            start_time: 0.0,
            end_time: 39.487,
            index_within_video: 0,
            video_id: "rDxeypDY".into(),
            title: "Cowgirl".into(),
        },
        Marker {
            loops: 1,
            source: VideoSource::Folder,
            id: 10,
            start_time: 0.0,
            end_time: 166.0,
            index_within_video: 0,
            video_id: "wkjHYedN".into(),
            title: "Sex".into(),
        },
        Marker {
            loops: 1,
            source: VideoSource::Folder,
            id: 8,
            start_time: 0.0,
            end_time: 34.597007,
            index_within_video: 0,
            video_id: "yObK_Z7p".into(),
            title: "Sideways".into(),
        },
    ]
}

pub fn create_marker(title: &str, start_time: f64, end_time: f64, index: usize) -> Marker {
    lazy_static! {
        static ref ID: AtomicI64 = AtomicI64::new(0);
    }

    let id = ID.fetch_add(1, Ordering::SeqCst);

    Marker {
        loops: 1,
        id,
        start_time,
        end_time,
        index_within_video: index,
        video_id: generate_id(),
        source: VideoSource::Folder,
        title: title.to_string(),
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
        loops: 1,
        id,
        start_time,
        end_time,
        index_within_video: index,
        video_id: video_id.to_string(),
        source: VideoSource::Folder,
        title: Faker.fake(),
    }
}

pub fn create_marker_with_loops(
    id: i64,
    start_time: f64,
    end_time: f64,
    index: usize,
    video_id: &str,
    loops: usize,
) -> Marker {
    Marker {
        loops,
        id,
        start_time,
        end_time,
        index_within_video: index,
        video_id: video_id.to_string(),
        source: VideoSource::Folder,
        title: Faker.fake(),
    }
}

pub async fn persist_video(db: &Database) -> Result<DbVideo> {
    let video = CreateVideo {
        file_path: format!(
            "{}/{}",
            Word().fake::<String>(),
            FilePath().fake::<String>()
        ),
        id: generate_id(),
        interactive: false,
        source: VideoSource::Folder,
        duration: 50.0,
        video_preview_image: None,
        stash_scene_id: None,
        title: Some(Word().fake::<String>()),
        tags: Some(Word().fake::<String>()),
        created_on: None,
    };

    db.videos.persist_video(&video).await
}

pub async fn persist_video_with<F: FnOnce(&mut CreateVideo)>(
    db: &Database,
    before_insert: F,
) -> Result<DbVideo> {
    let mut video = CreateVideo {
        file_path: format!(
            "{}/{}",
            Word().fake::<String>(),
            FilePath().fake::<String>()
        ),
        id: generate_id(),
        interactive: false,
        source: VideoSource::Folder,
        duration: 50.0,
        video_preview_image: None,
        stash_scene_id: None,
        title: Some(Word().fake::<String>()),
        tags: Some(Word().fake::<String>()),
        created_on: Some(unix_timestamp_now()),
    };
    before_insert(&mut video);
    info!("inserting video {:#?}", video);
    db.videos.persist_video(&video).await
}

pub async fn persist_marker(
    db: &Database,
    video_id: &str,
    index: i64,
    start: f64,
    end: f64,
    video_interactive: bool,
) -> Result<DbMarker> {
    let marker = CreateMarker {
        video_id: video_id.to_string(),
        start,
        end,
        index_within_video: index,
        title: Sentence(5..8).fake(),
        preview_image_path: None,
        video_interactive,
        created_on: None,
        marker_stash_id: None,
    };
    db.markers.create_new_marker(marker).await
}

pub async fn persist_performer(
    db: &Database,
    name: &str,
    gender: Option<Gender>,
    image_url: Option<&str>,
    stash_id: Option<&str>,
) -> Result<DbPerformer> {
    let id = db
        .performers
        .insert(&CreatePerformer {
            name: name.into(),
            gender,
            image_url: image_url.map(From::from),
            stash_id: stash_id.map(From::from),
        })
        .await?;

    Ok(DbPerformer {
        id,
        name: name.into(),
        created_on: unix_timestamp_now(),
        image_url: image_url.map(From::from),
        stash_id: stash_id.map(From::from),
        gender,
        marker_count: 0,
        video_count: 0,
    })
}

pub fn songs() -> Vec<Beats> {
    vec![
        Beats {
            length: 177.6275,
            offsets: vec![
                2.9706666, 3.248, 3.5253334, 3.8026667, 4.08, 4.357333, 4.6346664, 4.912,
                5.1893334, 5.4666667, 5.7493334, 6.0373335, 6.3199997, 6.6026664, 6.885333,
                7.173333, 7.456, 7.7386665, 8.026667, 8.448, 8.949333, 9.4453335, 9.941333,
                10.437333, 10.970667, 11.466666, 11.967999, 12.469334, 12.970667, 13.472,
                13.994666, 14.496, 14.997334, 15.498667, 15.994666, 16.517334, 17.018667,
                17.514666, 18.016, 18.517334, 19.018667, 19.530666, 20.032, 20.533333, 21.029333,
                21.530666, 22.058666, 22.56, 23.061333, 23.562666, 24.058666, 24.56, 25.344,
                25.845333, 26.346666, 26.842667, 27.354666, 27.855999, 28.357332, 28.858665, 29.36,
                29.855999, 30.634666, 31.130667, 31.632, 32.13333, 32.634666, 33.157333, 33.658665,
                34.16, 34.66133, 35.162666, 35.674667, 36.176, 36.677334, 37.173332, 37.674667,
                38.176, 38.698666, 39.2, 39.701332, 40.197334, 40.698666, 41.216, 41.717335,
                42.218666, 42.72, 43.221333, 43.738667, 44.239998, 44.741333, 45.242664, 45.744,
                46.245335, 46.767998, 47.264, 47.76533, 48.266666, 48.767998, 49.28533, 49.786667,
                50.288, 50.789333, 51.290665, 51.792, 52.314667, 52.815998, 53.317333, 53.818665,
                54.32, 54.837334, 55.338665, 55.84, 56.34133, 56.842667, 57.343998, 57.845333,
                58.346664, 58.848, 59.34933, 59.850666, 60.378666, 60.88, 61.381332, 61.882668,
                62.378666, 62.906666, 63.402664, 63.904, 64.405334, 64.90667, 65.402664, 65.92533,
                66.42667, 66.92267, 67.423996, 67.92533, 68.442665, 68.944, 69.445335, 69.94666,
                70.448, 70.94933, 71.472, 71.973335, 72.47466, 72.976, 73.47733, 74.0, 74.501335,
                74.99733, 75.498665, 76.0, 76.517334, 77.01867, 77.52, 78.02133, 78.517334,
                79.01867, 79.541336, 80.04266, 80.544, 81.04, 81.541336, 82.069336, 82.57066,
                83.066666, 83.568, 84.069336, 84.57066, 85.09866, 85.6, 86.10133, 86.60267, 87.104,
                87.610664, 88.112, 88.608, 89.10933, 89.610664, 90.13333, 90.63467, 91.136,
                91.63733, 92.138664, 92.64, 93.16267, 93.664, 94.16533, 94.66133, 95.16267,
                95.67467, 96.176, 96.67733, 97.178665, 97.68, 98.181335, 98.704, 99.2, 99.70133,
                100.20267, 100.704, 101.2, 101.70133, 102.20267, 102.704, 103.20533, 103.706665,
                104.21333, 104.71467, 105.215996, 105.71733, 106.21333, 106.736, 107.237335,
                107.73866, 108.24, 108.736, 109.264, 109.765335, 110.26133, 110.762665, 111.264,
                111.765335, 112.28267, 112.784, 113.28533, 113.78667, 114.28267, 114.80533,
                115.30666, 115.808, 116.304, 116.80533, 117.30666, 117.82933, 118.330666, 118.832,
                119.33333, 119.83466, 120.352, 120.85333, 121.349335, 121.85066, 122.352,
                122.85333, 123.35467, 123.85066, 124.352, 124.85333, 125.35467, 125.89867, 126.4,
                126.90133, 127.39733, 127.89867, 128.416, 128.91733, 129.41867, 129.91467, 130.416,
                130.91733, 131.44533, 131.94667, 132.448, 132.944, 133.44533, 133.93066, 134.43199,
                134.93333, 135.43466, 135.93066, 136.43199, 136.95999, 137.456, 137.95734,
                138.45866, 138.95999, 139.47733, 139.97867, 140.48, 140.98134, 141.48267,
                141.97867, 142.76266, 143.26399, 143.76534, 144.26134, 144.784, 145.28534,
                145.78667, 146.288, 146.78934, 147.28534, 147.808, 148.304, 148.80533, 149.30667,
                149.808, 150.304, 150.8, 151.30133, 151.80266, 152.29866, 152.8, 153.296,
                153.79199, 154.29333, 154.79466, 155.29066, 155.67467, 156.17067, 156.672,
                157.17332, 157.66933, 158.17067, 158.69333, 159.19467, 159.696, 160.192, 160.69333,
                161.216, 161.71733, 162.21333, 162.71466, 163.216, 163.71733, 164.224, 164.72533,
                165.22667, 165.728, 166.224, 166.76266, 167.26399, 167.76, 168.26134, 168.76266,
                169.25867, 169.79199, 170.29333, 170.79466, 171.296, 171.79733, 172.29333,
                172.78934, 173.29066, 173.79199, 174.288, 174.784, 175.28532, 175.78667, 176.28267,
            ],
        },
        Beats {
            length: 294.2475,
            offsets: vec![
                3.088, 3.6053333, 4.128, 4.6506667, 5.173333, 5.7066665, 6.2293334, 6.7466664,
                7.2693334, 7.792, 8.325334, 8.848, 9.3706665, 9.893333, 10.410666, 10.944,
                11.466666, 11.984, 12.506666, 13.024, 13.546666, 14.085333, 14.602667, 15.125333,
                15.642667, 16.16, 16.704, 17.221333, 17.738667, 18.261333, 18.778666, 19.328,
                19.845333, 20.362667, 20.88, 21.397333, 21.941334, 22.464, 22.981333, 23.504,
                24.021334, 24.544, 25.082666, 25.6, 26.122667, 26.64, 27.162666, 27.712, 28.234667,
                28.751999, 29.274666, 29.792, 30.602667, 31.125334, 31.647999, 32.170666, 32.688,
                33.226665, 33.744, 34.266666, 34.784, 35.306667, 35.845333, 36.362667, 36.885334,
                37.402668, 37.92533, 38.458668, 38.975998, 39.498665, 40.016, 40.533333, 41.082664,
                41.60533, 42.122665, 42.64, 43.162666, 43.68, 44.469334, 44.992, 45.509335,
                46.026665, 46.570667, 47.088, 47.60533, 48.122665, 48.645332, 49.2, 49.71733,
                50.239998, 50.75733, 51.274666, 51.792, 52.34133, 52.864, 53.381332, 53.898666,
                54.416, 54.965332, 55.482666, 56.005333, 56.522667, 57.045334, 57.584, 58.101334,
                58.624, 59.141335, 59.658665, 60.218666, 60.736, 61.253334, 61.776, 62.29333,
                62.837334, 63.354668, 63.871998, 64.389336, 64.912, 65.42933, 65.99467, 66.512,
                67.029335, 67.552, 68.069336, 68.613335, 69.13067, 69.648, 70.17067, 70.687996,
                71.20533, 71.722664, 72.24, 72.762665, 73.28, 73.824, 74.34133, 74.858665,
                75.38133, 75.89867, 76.416, 76.74133, 77.25867, 77.781334, 78.29867, 78.816,
                79.33867, 79.861336, 80.37866, 80.895996, 81.41333, 81.96267, 82.479996, 82.99733,
                83.51466, 84.032, 84.54933, 85.37067, 85.888, 86.405334, 86.92267, 87.44, 87.95733,
                88.47466, 88.992, 89.50933, 90.026665, 90.85333, 91.376, 91.89333, 92.41067,
                92.864, 93.386665, 93.904, 94.42133, 94.93867, 95.461334, 95.984, 96.501335,
                97.01867, 97.541336, 98.05866, 98.46933, 98.992, 99.50933, 100.026665, 100.544,
                101.072, 101.594666, 102.112, 102.62933, 103.14667, 103.664, 104.234665, 104.752,
                105.26933, 105.78667, 106.304, 107.10933, 107.62666, 108.14933, 108.666664,
                109.184, 109.610664, 110.13333, 110.650665, 111.168, 111.68533, 112.10667,
                112.62933, 113.14667, 113.664, 114.181335, 114.84267, 115.36533, 115.88267, 116.4,
                116.917336, 117.6, 118.122665, 118.64533, 119.168, 119.69067, 120.208, 120.725334,
                121.24267, 121.765335, 122.28267, 122.80533, 123.36, 123.88267, 124.4, 124.92267,
                125.44, 125.97867, 126.501335, 127.01866, 127.54133, 128.05867, 128.48, 128.99733,
                129.52, 130.03734, 130.56, 131.22667, 131.74933, 132.26666, 132.78934, 133.30667,
                133.84534, 134.36267, 134.88533, 135.40266, 135.92534, 136.44267, 136.752,
                137.27466, 137.79199, 138.31467, 138.832, 139.35466, 139.872, 140.39467, 140.912,
                141.43466, 141.952, 142.50133, 143.01866, 143.54134, 144.05867, 144.58133, 145.104,
                145.62134, 146.13867, 146.66133, 147.17867, 147.73866, 148.26134, 148.77867,
                149.30133, 149.81866, 150.34134, 150.864, 151.38133, 151.89867, 152.42133,
                152.95999, 153.47733, 154.0, 154.51733, 155.04, 155.55733, 156.13867, 156.656,
                157.17867, 157.696, 158.21866, 158.73067, 159.25333, 159.77066, 160.29333,
                160.81067, 161.36, 161.88266, 162.4, 162.92267, 163.44, 164.10133, 164.61867,
                165.136, 165.65866, 166.176, 166.72533, 167.24266, 167.76, 168.28267, 168.8,
                169.344, 169.86667, 170.384, 170.90666, 171.424, 171.94133, 172.49066, 173.01334,
                173.53067, 174.05333, 174.57066, 175.11467, 175.632, 176.15466, 176.672, 177.19467,
                177.728, 178.25067, 178.768, 179.29066, 179.808, 180.224, 180.74133, 181.26399,
                181.78133, 182.304, 182.82133, 183.37067, 183.89333, 184.41066, 184.93333,
                185.45067, 186.00533, 186.528, 187.04533, 187.568, 188.08533, 188.608, 189.12534,
                189.648, 190.16533, 190.688, 191.216, 191.73866, 192.256, 192.77333, 193.296,
                193.81332, 194.36267, 194.88533, 195.40266, 195.92534, 196.44266, 196.98134,
                197.49866, 198.02133, 198.53867, 199.06133, 199.59999, 200.11732, 200.64,
                201.15733, 201.67467, 202.21866, 202.736, 203.25333, 203.776, 204.29333, 204.81067,
                205.328, 205.85066, 206.368, 206.88533, 207.408, 207.73334, 208.256, 208.77333,
                209.29066, 209.81332, 210.34132, 210.864, 211.38133, 211.89867, 212.42133,
                212.93866, 213.48799, 214.00533, 214.52266, 215.04533, 215.56267, 216.112,
                216.62933, 217.14667, 217.66933, 218.18666, 218.72533, 219.24266, 219.76,
                220.27733, 220.8, 221.35466, 221.872, 222.38933, 222.90666, 223.424, 223.96266,
                224.48, 225.00267, 225.52, 226.03734, 226.55466, 226.98666, 227.504, 228.02133,
                228.53867, 229.06133, 229.48799, 230.01067, 230.528, 231.04533, 231.568, 232.08533,
                232.61333, 233.136, 233.65334, 234.176, 234.69333, 235.216, 235.73866, 236.256,
                236.77867, 237.29599, 237.856, 238.37866, 238.896, 239.41867, 239.936, 240.60266,
                241.12534, 241.64267, 242.16533, 242.68266, 243.35466, 243.87733, 244.39467,
                244.912, 245.43466, 245.84534, 246.368, 246.88533, 247.408, 247.92532, 248.448,
                248.98666, 249.504, 250.02666, 250.54399, 251.06133, 251.472, 251.98933, 252.50667,
                253.02933, 253.54666, 254.07466, 254.592, 255.11467, 255.632, 256.14932, 256.672,
                256.99734, 257.51468, 258.03198, 258.55466, 259.072, 259.59467, 260.112, 260.62933,
                261.152, 261.66934, 262.18668, 262.704, 263.22134, 263.744, 264.26132, 264.77866,
                265.36, 265.88266, 266.4, 266.92267, 267.44, 267.97333, 268.49066, 269.008,
                269.53067, 270.048, 270.85333, 271.37067, 271.888, 272.41068, 272.928, 273.47733,
                273.99466, 274.51733, 275.03467, 275.552, 276.35733, 276.87466, 277.392, 277.90933,
                278.42667, 278.944, 279.46133, 279.97867, 280.496, 281.01334, 281.52533, 282.04266,
                282.56, 283.08267, 283.6, 284.112, 284.62933, 285.14667, 285.664, 286.18134,
                286.69867, 287.21066, 287.728, 288.24533, 288.76266, 289.28, 289.78665, 290.29333,
                290.8,
            ],
        },
    ]
}

/// Uses ffmpeg to generate a video in the desired dimensions
pub async fn generate_video(path: impl AsRef<Utf8Path>, width: u32, height: u32) -> Result<()> {
    let path = Utf8Path::new("testfiles").join(path);
    Command::new("ffmpeg")
        .args([
            "-f",
            "lavfi",
            "-i",
            &format!("color=size={width}x{height}:duration=10:rate=30:color=red"),
            path.as_str(),
        ])
        .output()
        .await
        .map(|_| ())
        .map_err(From::from)
}
