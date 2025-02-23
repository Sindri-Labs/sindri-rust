/*
 * Sindri Labs API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.15.1
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenVerifyInputSchema {
    #[serde(rename = "token")]
    pub token: String,
}

impl TokenVerifyInputSchema {
    pub fn new(token: String) -> TokenVerifyInputSchema {
        TokenVerifyInputSchema { token }
    }
}
