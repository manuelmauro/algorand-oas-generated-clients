/*
 * Algod REST API.
 *
 * API endpoint for algod operations.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: contact@algorand.com
 * Generated by: https://openapi-generator.tech
 */

/// Box : Box name and its content.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Box {
    /// \\[name\\] box name, base64 encoded
    #[serde(rename = "name")]
    pub name: String,
    /// \\[value\\] box value, base64 encoded.
    #[serde(rename = "value")]
    pub value: String,
}

impl Box {
    /// Box name and its content.
    pub fn new(name: String, value: String) -> Box {
        Box {
            name,
            value,
        }
    }
}


