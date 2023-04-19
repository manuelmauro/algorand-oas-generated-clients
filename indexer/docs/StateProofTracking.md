# StateProofTracking

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**next_round** | Option<**i32**> | \\[n\\] Next round for which we will accept a state proof transaction. | [optional]
**online_total_weight** | Option<**i32**> | \\[t\\] The total number of microalgos held by the online accounts during the StateProof round. | [optional]
**r#type** | Option<**i32**> | State Proof Type. Note the raw object uses map with this as key. | [optional]
**voters_commitment** | Option<**String**> | \\[v\\] Root of a vector commitment containing online accounts that will help sign the proof. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


