/*
 * Indexer
 *
 * Algorand ledger analytics API.
 *
 * The version of the OpenAPI document: 2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TealValue : Represents a TEAL value.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TealValue {
    /// \\[tb\\] bytes value.
    #[serde(rename = "bytes")]
    pub bytes: String,
    /// \\[tt\\] value type.
    #[serde(rename = "type")]
    pub _type: i32,
    /// \\[ui\\] uint value.
    #[serde(rename = "uint")]
    pub uint: i32,
}

impl TealValue {
    /// Represents a TEAL value.
    pub fn new(bytes: String, _type: i32, uint: i32) -> TealValue {
        TealValue {
            bytes,
            _type,
            uint,
        }
    }
}

