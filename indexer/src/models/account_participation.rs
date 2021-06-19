/*
 * Indexer
 *
 * Algorand ledger analytics API.
 *
 * The version of the OpenAPI document: 2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AccountParticipation : AccountParticipation describes the parameters used by this account in consensus protocol.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountParticipation {
    /// \\[sel\\] Selection public key (if any) currently registered for this round.
    #[serde(rename = "selection-participation-key")]
    pub selection_participation_key: String,
    /// \\[voteFst\\] First round for which this participation is valid.
    #[serde(rename = "vote-first-valid")]
    pub vote_first_valid: i32,
    /// \\[voteKD\\] Number of subkeys in each batch of participation keys.
    #[serde(rename = "vote-key-dilution")]
    pub vote_key_dilution: i32,
    /// \\[voteLst\\] Last round for which this participation is valid.
    #[serde(rename = "vote-last-valid")]
    pub vote_last_valid: i32,
    /// \\[vote\\] root participation public key (if any) currently registered for this round.
    #[serde(rename = "vote-participation-key")]
    pub vote_participation_key: String,
}

impl AccountParticipation {
    /// AccountParticipation describes the parameters used by this account in consensus protocol.
    pub fn new(selection_participation_key: String, vote_first_valid: i32, vote_key_dilution: i32, vote_last_valid: i32, vote_participation_key: String) -> AccountParticipation {
        AccountParticipation {
            selection_participation_key,
            vote_first_valid,
            vote_key_dilution,
            vote_last_valid,
            vote_participation_key,
        }
    }
}


