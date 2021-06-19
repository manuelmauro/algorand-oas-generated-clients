/*
 * Indexer
 *
 * Algorand ledger analytics API.
 *
 * The version of the OpenAPI document: 2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TealKeyValue : Represents a key-value pair in an application store.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TealKeyValue {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: Box<crate::models::TealValue>,
}

impl TealKeyValue {
    /// Represents a key-value pair in an application store.
    pub fn new(key: String, value: crate::models::TealValue) -> TealKeyValue {
        TealKeyValue {
            key,
            value: Box::new(value),
        }
    }
}


