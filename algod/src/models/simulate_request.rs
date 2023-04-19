/*
 * Algod REST API.
 *
 * API endpoint for algod operations.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: contact@algorand.com
 * Generated by: https://openapi-generator.tech
 */

/// SimulateRequest : Request type for simulation endpoint.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulateRequest {
    /// The transaction groups to simulate.
    #[serde(rename = "txn-groups")]
    pub txn_groups: Vec<crate::models::SimulateRequestTransactionGroup>,
}

impl SimulateRequest {
    /// Request type for simulation endpoint.
    pub fn new(txn_groups: Vec<crate::models::SimulateRequestTransactionGroup>) -> SimulateRequest {
        SimulateRequest {
            txn_groups,
        }
    }
}


