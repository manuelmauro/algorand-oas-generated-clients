# TransactionAssetTransfer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | **i32** | \\[aamt\\] Amount of asset to transfer. A zero amount transferred to self allocates that asset in the account's Assets map. | 
**asset_id** | **i32** | \\[xaid\\] ID of the asset being transferred. | 
**close_amount** | Option<**i32**> | Number of assets transfered to the close-to account as part of the transaction. | [optional]
**close_to** | Option<**String**> | \\[aclose\\] Indicates that the asset should be removed from the account's Assets map, and specifies where the remaining asset holdings should be transferred.  It's always valid to transfer remaining asset holdings to the creator account. | [optional]
**receiver** | **String** | \\[arcv\\] Recipient address of the transfer. | 
**sender** | Option<**String**> | \\[asnd\\] The effective sender during a clawback transactions. If this is not a zero value, the real transaction sender must be the Clawback address from the AssetParams. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


