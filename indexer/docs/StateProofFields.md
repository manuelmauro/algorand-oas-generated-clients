# StateProofFields

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**part_proofs** | Option<[**crate::models::MerkleArrayProof**](MerkleArrayProof.md)> |  | [optional]
**positions_to_reveal** | Option<**Vec<i32>**> | \\[pr\\] Sequence of reveal positions. | [optional]
**reveals** | Option<[**Vec<crate::models::StateProofReveal>**](StateProofReveal.md)> | \\[r\\] Note that this is actually stored as a map[uint64] - Reveal in the actual msgp | [optional]
**salt_version** | Option<**i32**> | \\[v\\] Salt version of the merkle signature. | [optional]
**sig_commit** | Option<**String**> | \\[c\\] | [optional]
**sig_proofs** | Option<[**crate::models::MerkleArrayProof**](MerkleArrayProof.md)> |  | [optional]
**signed_weight** | Option<**i32**> | \\[w\\] | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


