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
pub struct RoundRobinClipOptions {
    #[serde(rename = "clipLengths")]
    pub clip_lengths: Box<models::ClipLengthOptions>,
    #[serde(rename = "length")]
    pub length: f64,
    #[serde(rename = "lenientDuration")]
    pub lenient_duration: bool,
    #[serde(
        rename = "minClipDuration",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_clip_duration: Option<Option<f64>>,
}

impl RoundRobinClipOptions {
    pub fn new(
        clip_lengths: models::ClipLengthOptions,
        length: f64,
        lenient_duration: bool,
    ) -> RoundRobinClipOptions {
        RoundRobinClipOptions {
            clip_lengths: Box::new(clip_lengths),
            length,
            lenient_duration,
            min_clip_duration: None,
        }
    }
}
