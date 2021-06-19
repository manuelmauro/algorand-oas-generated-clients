/*
 * Algod REST API.
 *
 * API endpoint for algod operations.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: contact@algorand.com
 * Generated by: https://openapi-generator.tech
 */

/// InlineResponse2004 : An catchpoint abort response.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse2004 {
    /// Catchup abort response string
    #[serde(rename = "catchup-message")]
    pub catchup_message: String,
}

impl InlineResponse2004 {
    /// An catchpoint abort response.
    pub fn new(catchup_message: String) -> InlineResponse2004 {
        InlineResponse2004 {
            catchup_message,
        }
    }
}

