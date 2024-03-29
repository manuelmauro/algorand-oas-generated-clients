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
pub struct StateProofTracking {
    /// \\[n\\] Next round for which we will accept a state proof transaction.
    #[serde(rename = "next-round", skip_serializing_if = "Option::is_none")]
    pub next_round: Option<i32>,
    /// \\[t\\] The total number of microalgos held by the online accounts during the StateProof round.
    #[serde(rename = "online-total-weight", skip_serializing_if = "Option::is_none")]
    pub online_total_weight: Option<i32>,
    /// State Proof Type. Note the raw object uses map with this as key.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    /// \\[v\\] Root of a vector commitment containing online accounts that will help sign the proof.
    #[serde(rename = "voters-commitment", skip_serializing_if = "Option::is_none")]
    pub voters_commitment: Option<String>,
}

impl StateProofTracking {
    pub fn new() -> StateProofTracking {
        StateProofTracking {
            next_round: None,
            online_total_weight: None,
            r#type: None,
            voters_commitment: None,
        }
    }
}


