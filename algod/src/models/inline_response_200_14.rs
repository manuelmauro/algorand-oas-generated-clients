/*
 * Algod REST API.
 *
 * API endpoint for algod operations.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: contact@algorand.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InlineResponse20014 {
    /// encoding of the transaction hash.
    #[serde(rename = "txId")]
    pub tx_id: String,
}

impl InlineResponse20014 {
    pub fn new(tx_id: String) -> InlineResponse20014 {
        InlineResponse20014 {
            tx_id,
        }
    }
}


