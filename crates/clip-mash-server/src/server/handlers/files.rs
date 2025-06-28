use std::cmp::Ordering;
use std::sync::Arc;

use axum::Json;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};
use tokio::fs::DirEntry;
use utoipa::{IntoParams, ToSchema};

use crate::server::error::AppError;
use crate::server::handlers::AppState;
use clip_mash::service::directories::FolderType;

#[derive(Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct ListFileEntriesQuery {
    pub path: Option<String>,
    pub include_hidden: Option<bool>,
}

#[derive(Serialize, ToSchema, Debug)]
pub struct ListFileEntriesResponse {
    pub entries: Vec<FileSystemEntry>,
    pub directory: String,
    pub drives: Vec<String>,
}

#[derive(Serialize, ToSchema, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum FileSystemEntry {
    #[serde(rename_all = "camelCase")]
    Directory {
        file_name: String,
        full_path: String,
    },
    #[serde(rename_all = "camelCase")]
    File {
        file_name: String,
        full_path: String,
        size: u64,
    },
}

impl PartialOrd for FileSystemEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (
                FileSystemEntry::Directory { file_name: a, .. },
                FileSystemEntry::Directory { file_name: b, .. },
            ) => {
                let a = a.to_lowercase();
                let b = b.to_lowercase();
                a.partial_cmp(&b)
            }
            (
                FileSystemEntry::File { file_name: a, .. },
                FileSystemEntry::File { file_name: b, .. },
            ) => {
                let a = a.to_lowercase();
                let b = b.to_lowercase();
                a.partial_cmp(&b)
            }
            (FileSystemEntry::Directory { .. }, FileSystemEntry::File { .. }) => {
                Some(Ordering::Less)
            }
            (FileSystemEntry::File { .. }, FileSystemEntry::Directory { .. }) => {
                Some(Ordering::Greater)
            }
        }
    }
}

impl Ord for FileSystemEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl FileSystemEntry {
    pub async fn from(entry: DirEntry) -> crate::Result<Self> {
        let metadata = entry.metadata().await?;
        let path = Utf8PathBuf::from_path_buf(entry.path()).expect("path must be utf-8");
        let file_name = path.file_name().unwrap_or("").to_string();
        if metadata.is_dir() {
            Ok(FileSystemEntry::Directory {
                file_name,
                full_path: path.to_string(),
            })
        } else {
            Ok(FileSystemEntry::File {
                file_name,
                full_path: path.to_string(),
                size: metadata.len(),
            })
        }
    }
}

// FIXME
#[allow(deprecated)]
fn get_or_home_dir(path: Option<String>) -> Utf8PathBuf {
    path.map(Utf8PathBuf::from)
        .or(std::env::home_dir().and_then(|home| Utf8PathBuf::from_path_buf(home).ok()))
        .unwrap_or_default()
}

#[cfg(not(target_os = "windows"))]
fn is_hidden(file: &std::path::Path) -> bool {
    file.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}

#[cfg(target_os = "windows")]
fn is_hidden(file: &std::path::Path) -> bool {
    use std::os::windows::fs::MetadataExt;
    const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;

    let metadata = file.metadata();
    if let Ok(metadata) = metadata {
        metadata.file_attributes() & FILE_ATTRIBUTE_HIDDEN > 0
    } else {
        false
    }
}

#[cfg(target_os = "windows")]
fn list_drives() -> Vec<String> {
    use sysinfo::Disks;

    let disks = Disks::new_with_refreshed_list();
    let mut drives: Vec<_> = disks
        .into_iter()
        .map(|d| d.mount_point().to_string_lossy().to_string())
        .collect();
    drives.sort();

    drives
}

// no drives on other OSes
#[cfg(not(target_os = "windows"))]
fn list_drives() -> Vec<String> {
    vec![]
}

#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/api/library/directory",
    params(ListFileEntriesQuery),
    responses(
        (status = 200, description = "List all files in the given path", body = ListFileEntriesResponse),
    )
)]
pub async fn list_file_entries(
    Query(ListFileEntriesQuery {
        path,
        include_hidden,
    }): Query<ListFileEntriesQuery>,
) -> Result<impl IntoResponse, AppError> {
    let path = get_or_home_dir(path);
    let with_hidden = include_hidden.unwrap_or(false);

    let drives = list_drives();
    let mut entries = tokio::fs::read_dir(&path).await?;
    let mut files = vec![];
    while let Some(entry) = entries.next_entry().await? {
        if !with_hidden && is_hidden(&entry.path()) {
            continue;
        }

        let entry = FileSystemEntry::from(entry).await?;
        files.push(entry);
    }

    files.sort();

    if let Some(parent) = Utf8Path::new(&path).parent() {
        files.insert(
            0,
            FileSystemEntry::Directory {
                file_name: "..".to_string(),
                full_path: parent.to_string(),
            },
        );
    }

    Ok(Json(ListFileEntriesResponse {
        directory: path.to_string(),
        entries: files,
        drives,
    }))
}

#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/api/library/stats",
    responses(
        (status = 200, description = "Get the size of all folders", body = Vec<(FolderType, u64)>),
    )
)]
pub async fn get_file_stats(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let stats = state.directories.stats().await?;
    Ok(Json(stats))
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/api/library/cleanup/{folder_type}",
    params(
        ("folder_type" = FolderType, Path, description = "The type of folder to clean up")
    ),
    responses(
        (status = 200, description = "Cleanup the given folder", body = ()),
    )
)]
/// Deletes all generated files in the specified folder.
pub async fn cleanup_folder(
    Path(folder_type): Path<FolderType>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    state.directories.cleanup(folder_type).await?;

    Ok(Json(()))
}
