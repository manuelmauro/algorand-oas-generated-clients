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
pub struct StateProofSigSlot {
    /// \\[l\\] The total weight of signatures in the lower-numbered slots.
    #[serde(rename = "lower-sig-weight", skip_serializing_if = "Option::is_none")]
    pub lower_sig_weight: Option<i32>,
    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<Box<crate::models::StateProofSignature>>,
}

impl StateProofSigSlot {
    pub fn new() -> StateProofSigSlot {
        StateProofSigSlot {
            lower_sig_weight: None,
            signature: None,
        }
    }
}


