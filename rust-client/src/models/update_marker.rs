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
pub struct UpdateMarker {
    #[serde(
        rename = "end",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub end: Option<Option<f64>>,
    #[serde(
        rename = "start",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub start: Option<Option<f64>>,
    #[serde(
        rename = "stashMarkerId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stash_marker_id: Option<Option<i64>>,
    #[serde(
        rename = "title",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<Option<String>>,
}

impl UpdateMarker {
    pub fn new() -> UpdateMarker {
        UpdateMarker {
            end: None,
            start: None,
            stash_marker_id: None,
            title: None,
        }
    }
}
