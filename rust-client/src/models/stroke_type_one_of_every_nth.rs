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

/// StrokeTypeOneOfEveryNth : Creates a stroke every `n` beats
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StrokeTypeOneOfEveryNth {
    #[serde(rename = "n")]
    pub n: i32,
}

impl StrokeTypeOneOfEveryNth {
    /// Creates a stroke every `n` beats
    pub fn new(n: i32) -> StrokeTypeOneOfEveryNth {
        StrokeTypeOneOfEveryNth { n }
    }
}
