/*
 * Indexer
 *
 * Algorand ledger analytics API.
 *
 * The version of the OpenAPI document: 2.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `search_for_accounts`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchForAccountsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `search_for_applications`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchForApplicationsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `search_for_assets`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchForAssetsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `search_for_transactions`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchForTransactionsError {
    Status500(crate::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}


/// Search for accounts.
pub async fn search_for_accounts(configuration: &configuration::Configuration, asset_id: Option<i32>, limit: Option<i32>, next: Option<&str>, currency_greater_than: Option<i32>, include_all: Option<bool>, currency_less_than: Option<i32>, auth_addr: Option<&str>, round: Option<i32>, application_id: Option<i32>) -> Result<crate::models::InlineResponse200, Error<SearchForAccountsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v2/accounts", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = asset_id {
        local_var_req_builder = local_var_req_builder.query(&[("asset-id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = next {
        local_var_req_builder = local_var_req_builder.query(&[("next", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = currency_greater_than {
        local_var_req_builder = local_var_req_builder.query(&[("currency-greater-than", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include_all {
        local_var_req_builder = local_var_req_builder.query(&[("include-all", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = currency_less_than {
        local_var_req_builder = local_var_req_builder.query(&[("currency-less-than", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = auth_addr {
        local_var_req_builder = local_var_req_builder.query(&[("auth-addr", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = round {
        local_var_req_builder = local_var_req_builder.query(&[("round", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = application_id {
        local_var_req_builder = local_var_req_builder.query(&[("application-id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SearchForAccountsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Search for applications
pub async fn search_for_applications(configuration: &configuration::Configuration, application_id: Option<i32>, include_all: Option<bool>, limit: Option<i32>, next: Option<&str>) -> Result<crate::models::InlineResponse2003, Error<SearchForApplicationsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v2/applications", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = application_id {
        local_var_req_builder = local_var_req_builder.query(&[("application-id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include_all {
        local_var_req_builder = local_var_req_builder.query(&[("include-all", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = next {
        local_var_req_builder = local_var_req_builder.query(&[("next", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SearchForApplicationsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Search for assets.
pub async fn search_for_assets(configuration: &configuration::Configuration, include_all: Option<bool>, limit: Option<i32>, next: Option<&str>, creator: Option<&str>, name: Option<&str>, unit: Option<&str>, asset_id: Option<i32>) -> Result<crate::models::InlineResponse2005, Error<SearchForAssetsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v2/assets", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = include_all {
        local_var_req_builder = local_var_req_builder.query(&[("include-all", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = next {
        local_var_req_builder = local_var_req_builder.query(&[("next", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = creator {
        local_var_req_builder = local_var_req_builder.query(&[("creator", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = name {
        local_var_req_builder = local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = unit {
        local_var_req_builder = local_var_req_builder.query(&[("unit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = asset_id {
        local_var_req_builder = local_var_req_builder.query(&[("asset-id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SearchForAssetsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Search for transactions.
pub async fn search_for_transactions(configuration: &configuration::Configuration, limit: Option<i32>, next: Option<&str>, note_prefix: Option<&str>, tx_type: Option<&str>, sig_type: Option<&str>, txid: Option<&str>, round: Option<i32>, min_round: Option<i32>, max_round: Option<i32>, asset_id: Option<i32>, before_time: Option<String>, after_time: Option<String>, currency_greater_than: Option<i32>, currency_less_than: Option<i32>, address: Option<&str>, address_role: Option<&str>, exclude_close_to: Option<bool>, rekey_to: Option<bool>, application_id: Option<i32>) -> Result<crate::models::InlineResponse2002, Error<SearchForTransactionsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v2/transactions", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = next {
        local_var_req_builder = local_var_req_builder.query(&[("next", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = note_prefix {
        local_var_req_builder = local_var_req_builder.query(&[("note-prefix", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = tx_type {
        local_var_req_builder = local_var_req_builder.query(&[("tx-type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sig_type {
        local_var_req_builder = local_var_req_builder.query(&[("sig-type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = txid {
        local_var_req_builder = local_var_req_builder.query(&[("txid", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = round {
        local_var_req_builder = local_var_req_builder.query(&[("round", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = min_round {
        local_var_req_builder = local_var_req_builder.query(&[("min-round", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_round {
        local_var_req_builder = local_var_req_builder.query(&[("max-round", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = asset_id {
        local_var_req_builder = local_var_req_builder.query(&[("asset-id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = before_time {
        local_var_req_builder = local_var_req_builder.query(&[("before-time", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = after_time {
        local_var_req_builder = local_var_req_builder.query(&[("after-time", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = currency_greater_than {
        local_var_req_builder = local_var_req_builder.query(&[("currency-greater-than", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = currency_less_than {
        local_var_req_builder = local_var_req_builder.query(&[("currency-less-than", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = address {
        local_var_req_builder = local_var_req_builder.query(&[("address", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = address_role {
        local_var_req_builder = local_var_req_builder.query(&[("address-role", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = exclude_close_to {
        local_var_req_builder = local_var_req_builder.query(&[("exclude-close-to", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = rekey_to {
        local_var_req_builder = local_var_req_builder.query(&[("rekey-to", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = application_id {
        local_var_req_builder = local_var_req_builder.query(&[("application-id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SearchForTransactionsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
