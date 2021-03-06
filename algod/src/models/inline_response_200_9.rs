/*
 * Algod REST API.
 *
 * API endpoint for algod operations.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: contact@algorand.com
 * Generated by: https://openapi-generator.tech
 */

/// InlineResponse2009 : NodeStatus contains the information about a node status



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InlineResponse2009 {
    /// The current catchpoint that is being caught up to
    #[serde(rename = "catchpoint", skip_serializing_if = "Option::is_none")]
    pub catchpoint: Option<String>,
    /// The number of blocks that have already been obtained by the node as part of the catchup
    #[serde(rename = "catchpoint-acquired-blocks", skip_serializing_if = "Option::is_none")]
    pub catchpoint_acquired_blocks: Option<i32>,
    /// The number of accounts from the current catchpoint that have been processed so far as part of the catchup
    #[serde(rename = "catchpoint-processed-accounts", skip_serializing_if = "Option::is_none")]
    pub catchpoint_processed_accounts: Option<i32>,
    /// The total number of accounts included in the current catchpoint
    #[serde(rename = "catchpoint-total-accounts", skip_serializing_if = "Option::is_none")]
    pub catchpoint_total_accounts: Option<i32>,
    /// The total number of blocks that are required to complete the current catchpoint catchup
    #[serde(rename = "catchpoint-total-blocks", skip_serializing_if = "Option::is_none")]
    pub catchpoint_total_blocks: Option<i32>,
    /// The number of accounts from the current catchpoint that have been verified so far as part of the catchup
    #[serde(rename = "catchpoint-verified-accounts", skip_serializing_if = "Option::is_none")]
    pub catchpoint_verified_accounts: Option<i32>,
    /// CatchupTime in nanoseconds
    #[serde(rename = "catchup-time")]
    pub catchup_time: i32,
    /// The last catchpoint seen by the node
    #[serde(rename = "last-catchpoint", skip_serializing_if = "Option::is_none")]
    pub last_catchpoint: Option<String>,
    /// LastRound indicates the last round seen
    #[serde(rename = "last-round")]
    pub last_round: i32,
    /// LastVersion indicates the last consensus version supported
    #[serde(rename = "last-version")]
    pub last_version: String,
    /// NextVersion of consensus protocol to use
    #[serde(rename = "next-version")]
    pub next_version: String,
    /// NextVersionRound is the round at which the next consensus version will apply
    #[serde(rename = "next-version-round")]
    pub next_version_round: i32,
    /// NextVersionSupported indicates whether the next consensus version is supported by this node
    #[serde(rename = "next-version-supported")]
    pub next_version_supported: bool,
    /// StoppedAtUnsupportedRound indicates that the node does not support the new rounds and has stopped making progress
    #[serde(rename = "stopped-at-unsupported-round")]
    pub stopped_at_unsupported_round: bool,
    /// TimeSinceLastRound in nanoseconds
    #[serde(rename = "time-since-last-round")]
    pub time_since_last_round: i32,
}

impl InlineResponse2009 {
    /// NodeStatus contains the information about a node status
    pub fn new(catchup_time: i32, last_round: i32, last_version: String, next_version: String, next_version_round: i32, next_version_supported: bool, stopped_at_unsupported_round: bool, time_since_last_round: i32) -> InlineResponse2009 {
        InlineResponse2009 {
            catchpoint: None,
            catchpoint_acquired_blocks: None,
            catchpoint_processed_accounts: None,
            catchpoint_total_accounts: None,
            catchpoint_total_blocks: None,
            catchpoint_verified_accounts: None,
            catchup_time,
            last_catchpoint: None,
            last_round,
            last_version,
            next_version,
            next_version_round,
            next_version_supported,
            stopped_at_unsupported_round,
            time_since_last_round,
        }
    }
}


