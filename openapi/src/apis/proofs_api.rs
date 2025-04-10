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

/// struct for typed errors of method [`proof_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProofDeleteError {
    Status404(models::ProofDoesNotExistResponse),
    Status500(models::SindriInternalErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`proof_detail`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProofDetailError {
    Status404(models::ProofDoesNotExistResponse),
    Status500(models::SindriInternalErrorResponse),
    Status501(models::ComingSoonResponse),
    UnknownValue(serde_json::Value),
}

/// Delete a specific proof.
pub async fn proof_delete(
    configuration: &configuration::Configuration,
    proof_id: &str,
) -> Result<models::ActionResponse, Error<ProofDeleteError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_proof_id = proof_id;

    let uri_str = format!(
        "{}/api/v1/proof/{proof_id}/delete",
        configuration.base_path,
        proof_id = crate::apis::urlencode(p_proof_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<ProofDeleteError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Get info for a specific proof.
pub async fn proof_detail(
    configuration: &configuration::Configuration,
    proof_id: &str,
    include_proof: Option<bool>,
    include_public: Option<bool>,
    include_smart_contract_calldata: Option<bool>,
    include_verification_key: Option<bool>,
) -> Result<models::ProofInfoResponse, Error<ProofDetailError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_proof_id = proof_id;
    let p_include_proof = include_proof;
    let p_include_public = include_public;
    let p_include_smart_contract_calldata = include_smart_contract_calldata;
    let p_include_verification_key = include_verification_key;

    let uri_str = format!(
        "{}/api/v1/proof/{proof_id}/detail",
        configuration.base_path,
        proof_id = crate::apis::urlencode(p_proof_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_include_proof {
        req_builder = req_builder.query(&[("include_proof", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_include_public {
        req_builder = req_builder.query(&[("include_public", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_include_smart_contract_calldata {
        req_builder =
            req_builder.query(&[("include_smart_contract_calldata", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_include_verification_key {
        req_builder = req_builder.query(&[("include_verification_key", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<ProofDetailError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
