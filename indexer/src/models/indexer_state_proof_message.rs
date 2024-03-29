/*
 * Indexer
 *
 * Algorand ledger analytics API.
 *
 * The version of the OpenAPI document: 2.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IndexerStateProofMessage {
    /// \\[b\\]
    #[serde(rename = "block-headers-commitment", skip_serializing_if = "Option::is_none")]
    pub block_headers_commitment: Option<String>,
    /// \\[f\\]
    #[serde(rename = "first-attested-round", skip_serializing_if = "Option::is_none")]
    pub first_attested_round: Option<i32>,
    /// \\[l\\]
    #[serde(rename = "latest-attested-round", skip_serializing_if = "Option::is_none")]
    pub latest_attested_round: Option<i32>,
    /// \\[P\\]
    #[serde(rename = "ln-proven-weight", skip_serializing_if = "Option::is_none")]
    pub ln_proven_weight: Option<i32>,
    /// \\[v\\]
    #[serde(rename = "voters-commitment", skip_serializing_if = "Option::is_none")]
    pub voters_commitment: Option<String>,
}

impl IndexerStateProofMessage {
    pub fn new() -> IndexerStateProofMessage {
        IndexerStateProofMessage {
            block_headers_commitment: None,
            first_attested_round: None,
            latest_attested_round: None,
            ln_proven_weight: None,
            voters_commitment: None,
        }
    }
}


