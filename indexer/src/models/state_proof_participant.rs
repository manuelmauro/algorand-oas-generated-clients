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
pub struct StateProofParticipant {
    #[serde(rename = "verifier", skip_serializing_if = "Option::is_none")]
    pub verifier: Option<Box<crate::models::StateProofVerifier>>,
    /// \\[w\\]
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

impl StateProofParticipant {
    pub fn new() -> StateProofParticipant {
        StateProofParticipant {
            verifier: None,
            weight: None,
        }
    }
}


