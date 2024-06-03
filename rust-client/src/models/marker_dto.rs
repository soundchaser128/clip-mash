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
pub struct MarkerDto {
    #[serde(rename = "createdOn")]
    pub created_on: i64,
    #[serde(rename = "end")]
    pub end: f64,
    #[serde(
        rename = "fileName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub file_name: Option<Option<String>>,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "indexWithinVideo")]
    pub index_within_video: i32,
    #[serde(rename = "primaryTag")]
    pub primary_tag: String,
    #[serde(rename = "sceneInteractive")]
    pub scene_interactive: bool,
    #[serde(
        rename = "sceneTitle",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub scene_title: Option<Option<String>>,
    #[serde(rename = "screenshotUrl")]
    pub screenshot_url: String,
    #[serde(rename = "source")]
    pub source: models::VideoSource,
    #[serde(rename = "start")]
    pub start: f64,
    #[serde(rename = "streamUrl")]
    pub stream_url: String,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "videoId")]
    pub video_id: String,
}

impl MarkerDto {
    pub fn new(
        created_on: i64,
        end: f64,
        id: i64,
        index_within_video: i32,
        primary_tag: String,
        scene_interactive: bool,
        screenshot_url: String,
        source: models::VideoSource,
        start: f64,
        stream_url: String,
        tags: Vec<String>,
        video_id: String,
    ) -> MarkerDto {
        MarkerDto {
            created_on,
            end,
            file_name: None,
            id,
            index_within_video,
            primary_tag,
            scene_interactive,
            scene_title: None,
            screenshot_url,
            source,
            start,
            stream_url,
            tags,
            video_id,
        }
    }
}
