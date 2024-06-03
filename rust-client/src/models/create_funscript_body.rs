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
pub struct CreateFunscriptBody {
    #[serde(rename = "clips")]
    pub clips: Vec<models::Clip>,
}

impl CreateFunscriptBody {
    pub fn new(clips: Vec<models::Clip>) -> CreateFunscriptBody {
        CreateFunscriptBody { clips }
    }
}
