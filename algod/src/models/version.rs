/*
 * Algod REST API.
 *
 * API endpoint for algod operations.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: contact@algorand.com
 * Generated by: https://openapi-generator.tech
 */

/// Version : algod version information.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Version {
    #[serde(rename = "build")]
    pub build: Box<crate::models::BuildVersion>,
    #[serde(rename = "genesis_hash_b64")]
    pub genesis_hash_b64: String,
    #[serde(rename = "genesis_id")]
    pub genesis_id: String,
    #[serde(rename = "versions")]
    pub versions: Vec<String>,
}

impl Version {
    /// algod version information.
    pub fn new(build: crate::models::BuildVersion, genesis_hash_b64: String, genesis_id: String, versions: Vec<String>) -> Version {
        Version {
            build: Box::new(build),
            genesis_hash_b64,
            genesis_id,
            versions,
        }
    }
}

