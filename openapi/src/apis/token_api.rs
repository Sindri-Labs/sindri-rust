/*
 * Sindri Labs API
 *
 *  ## About [Sindri Labs](https://www.sindri.app/)' API simplifies the developer experience to enable fast and scalable zero-knowledge proof generation.  Front-End Dashboard: [https://sindri.app/login](https://sindri.app/login)  ## Documentation The [Sindri Documentation](https://sindri.app/docs) contains everything you need to get started!  ## Sindri Resources The [sindri-resources GitHub repo](https://github.com/Sindri-Labs/sindri-resources) contains contains resources and sample data for the Sindri API.  ## Using this Page This is a standard [OpenAPI (Swagger)](https://swagger.io/specification/) API documentation page. It provides detailed documentation for each endpoint.  This page enables easy prototyping via the \"Try it out\" feature!  Since all Sindri endpoints require a valid API Key, in order to use the \"Try it out\" feature for any endpoint in this documentation you must first obtain an API key. Do this in one of two ways: 1. Enter your username and password in the `/api/apikey/generate` endpoint of the **Authorization** section below. Use the API key returned in the `access` field of the response. 2. Obtain an API key from the Sindri Dashboard team \"Account Settings\".  After obtaining your API key, authorize your page session by entering your API Key in the `SindriAPIKeyBearerAuth` section, reached by clicking \"Authorize\" below.  Proving Backend Version: v1.2.8
 *
 * The version of the OpenAPI document: v1.17.12
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize};

/// struct for typed errors of method [`jwt_token_generate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JwtTokenGenerateError {
    Status403(models::JwtTokenErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`jwt_token_refresh`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JwtTokenRefreshError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`jwt_token_verify`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JwtTokenVerifyError {
    UnknownValue(serde_json::Value),
}

/// Override the ninja_jwt default `obtain_token` method in order to add email verification check before generating a token.
pub async fn jwt_token_generate(
    configuration: &configuration::Configuration,
    token_obtain_pair_input_schema: models::TokenObtainPairInputSchema,
) -> Result<models::TokenObtainPairOutputSchema, Error<JwtTokenGenerateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_token_obtain_pair_input_schema = token_obtain_pair_input_schema;

    let uri_str = format!("{}/api/token/pair", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_token_obtain_pair_input_schema);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<JwtTokenGenerateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn jwt_token_refresh(
    configuration: &configuration::Configuration,
    token_refresh_input_schema: models::TokenRefreshInputSchema,
) -> Result<models::TokenRefreshOutputSchema, Error<JwtTokenRefreshError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_token_refresh_input_schema = token_refresh_input_schema;

    let uri_str = format!("{}/api/token/refresh", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_token_refresh_input_schema);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<JwtTokenRefreshError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn jwt_token_verify(
    configuration: &configuration::Configuration,
    token_verify_input_schema: models::TokenVerifyInputSchema,
) -> Result<serde_json::Value, Error<JwtTokenVerifyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_token_verify_input_schema = token_verify_input_schema;

    let uri_str = format!("{}/api/token/verify", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_token_verify_input_schema);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<JwtTokenVerifyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
