# TransactionApplication

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accounts** | Option<**Vec<String>**> | \\[apat\\] List of accounts in addition to the sender that may be accessed from the application's approval-program and clear-state-program. | [optional]
**application_args** | Option<**Vec<String>**> | \\[apaa\\] transaction specific arguments accessed from the application's approval-program and clear-state-program. | [optional]
**application_id** | **i32** | \\[apid\\] ID of the application being configured or empty if creating. | 
**approval_program** | Option<**String**> | \\[apap\\] Logic executed for every application transaction, except when on-completion is set to \"clear\". It can read and write global state for the application, as well as account-specific local state. Approval programs may reject the transaction. | [optional]
**clear_state_program** | Option<**String**> | \\[apsu\\] Logic executed for application transactions with on-completion set to \"clear\". It can read and write global state for the application, as well as account-specific local state. Clear state programs cannot reject the transaction. | [optional]
**foreign_apps** | Option<**Vec<i32>**> | \\[apfa\\] Lists the applications in addition to the application-id whose global states may be accessed by this application's approval-program and clear-state-program. The access is read-only. | [optional]
**foreign_assets** | Option<**Vec<i32>**> | \\[apas\\] lists the assets whose parameters may be accessed by this application's ApprovalProgram and ClearStateProgram. The access is read-only. | [optional]
**global_state_schema** | Option<[**crate::models::StateSchema**](StateSchema.md)> |  | [optional]
**local_state_schema** | Option<[**crate::models::StateSchema**](StateSchema.md)> |  | [optional]
**on_completion** | [**crate::models::OnCompletion**](OnCompletion.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


