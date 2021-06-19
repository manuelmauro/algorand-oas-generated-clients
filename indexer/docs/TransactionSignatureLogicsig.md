# TransactionSignatureLogicsig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**args** | Option<**Vec<String>**> | \\[arg\\] Logic arguments, base64 encoded. | [optional]
**logic** | **String** | \\[l\\] Program signed by a signature or multi signature, or hashed to be the address of ana ccount. Base64 encoded TEAL program. | 
**multisig_signature** | Option<[**crate::models::TransactionSignatureMultisig**](TransactionSignatureMultisig.md)> |  | [optional]
**signature** | Option<**String**> | \\[sig\\] ed25519 signature. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


