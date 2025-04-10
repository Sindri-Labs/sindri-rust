/*
 * Sindri Labs API
 *
 *  ## About [Sindri Labs](https://www.sindri.app/)' API simplifies the developer experience to enable fast and scalable zero-knowledge proof generation.  Front-End Dashboard: [https://sindri.app/login](https://sindri.app/login)  ## Documentation The [Sindri Documentation](https://sindri.app/docs) contains everything you need to get started!  ## Sindri Resources The [sindri-resources GitHub repo](https://github.com/Sindri-Labs/sindri-resources) contains contains resources and sample data for the Sindri API.  ## Using this Page This is a standard [OpenAPI (Swagger)](https://swagger.io/specification/) API documentation page. It provides detailed documentation for each endpoint.  This page enables easy prototyping via the \"Try it out\" feature!  Since all Sindri endpoints require a valid API Key, in order to use the \"Try it out\" feature for any endpoint in this documentation you must first obtain an API key. Do this in one of two ways: 1. Enter your username and password in the `/api/apikey/generate` endpoint of the **Authorization** section below. Use the API key returned in the `access` field of the response. 2. Obtain an API key from the Sindri Dashboard team \"Account Settings\".  After obtaining your API key, authorize your page session by entering your API Key in the `SindriAPIKeyBearerAuth` section, reached by clicking \"Authorize\" below.  Proving Backend Version: v1.2.8
 *
 * The version of the OpenAPI document: v1.17.12
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ObtainApikeyInput : Client input to obtain an API key.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObtainApikeyInput {
    /// Your account username.
    #[serde(rename = "username")]
    pub username: String,
    /// Your account password.
    #[serde(rename = "password")]
    pub password: String,
    /// A human readable name for your API key used to identify it when managing keys.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ObtainApikeyInput {
    /// Client input to obtain an API key.
    pub fn new(username: String, password: String) -> ObtainApikeyInput {
        ObtainApikeyInput {
            username,
            password,
            name: None,
        }
    }
}
