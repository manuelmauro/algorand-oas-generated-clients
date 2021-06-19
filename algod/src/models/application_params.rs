/*
 * Algod REST API.
 *
 * API endpoint for algod operations.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: contact@algorand.com
 * Generated by: https://openapi-generator.tech
 */

/// ApplicationParams : Stores the global information associated with an application.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationParams {
    /// \\[approv\\] approval program.
    #[serde(rename = "approval-program")]
    pub approval_program: String,
    /// \\[clearp\\] approval program.
    #[serde(rename = "clear-state-program")]
    pub clear_state_program: String,
    /// The address that created this application. This is the address where the parameters and global state for this application can be found.
    #[serde(rename = "creator")]
    pub creator: String,
    /// Represents a key-value store for use in an application.
    #[serde(rename = "global-state", skip_serializing_if = "Option::is_none")]
    pub global_state: Option<Vec<crate::models::TealKeyValue>>,
    #[serde(rename = "global-state-schema", skip_serializing_if = "Option::is_none")]
    pub global_state_schema: Option<Box<crate::models::ApplicationStateSchema>>,
    #[serde(rename = "local-state-schema", skip_serializing_if = "Option::is_none")]
    pub local_state_schema: Option<Box<crate::models::ApplicationStateSchema>>,
}

impl ApplicationParams {
    /// Stores the global information associated with an application.
    pub fn new(approval_program: String, clear_state_program: String, creator: String) -> ApplicationParams {
        ApplicationParams {
            approval_program,
            clear_state_program,
            creator,
            global_state: None,
            global_state_schema: None,
            local_state_schema: None,
        }
    }
}


