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
pub struct MeasureCountOneOf1 {
    #[serde(rename = "max")]
    pub max: i32,
    #[serde(rename = "min")]
    pub min: i32,
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl MeasureCountOneOf1 {
    pub fn new(max: i32, min: i32, r#type: Type) -> MeasureCountOneOf1 {
        MeasureCountOneOf1 { max, min, r#type }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "random")]
    Random,
}

impl Default for Type {
    fn default() -> Type {
        Self::Random
    }
}
