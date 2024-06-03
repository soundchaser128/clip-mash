/*
 * clip-mash
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.22.0-pre.1
 *
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VideoDto {
    #[serde(rename = "createdOn")]
    pub created_on: i64,
    #[serde(rename = "duration")]
    pub duration: f64,
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(
        rename = "filePath",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub file_path: Option<Option<String>>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "interactive")]
    pub interactive: bool,
    #[serde(rename = "performers")]
    pub performers: Vec<String>,
    #[serde(rename = "source")]
    pub source: models::VideoSource,
    #[serde(
        rename = "stashSceneId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stash_scene_id: Option<Option<i64>>,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "title")]
    pub title: String,
}

impl VideoDto {
    pub fn new(
        created_on: i64,
        duration: f64,
        file_name: String,
        id: String,
        interactive: bool,
        performers: Vec<String>,
        source: models::VideoSource,
        tags: Vec<String>,
        title: String,
    ) -> VideoDto {
        VideoDto {
            created_on,
            duration,
            file_name,
            file_path: None,
            id,
            interactive,
            performers,
            source,
            stash_scene_id: None,
            tags,
            title,
        }
    }
}
