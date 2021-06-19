/*
 * Algod REST API.
 *
 * API endpoint for algod operations.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: contact@algorand.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse2002 {
    /// Index of the transaction in the block's payset.
    #[serde(rename = "idx")]
    pub idx: i32,
    /// Merkle proof of transaction membership.
    #[serde(rename = "proof")]
    pub proof: String,
    /// Hash of SignedTxnInBlock for verifying proof.
    #[serde(rename = "stibhash")]
    pub stibhash: String,
}

impl InlineResponse2002 {
    pub fn new(idx: i32, proof: String, stibhash: String) -> InlineResponse2002 {
        InlineResponse2002 {
            idx,
            proof,
            stibhash,
        }
    }
}

