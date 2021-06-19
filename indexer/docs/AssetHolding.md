# AssetHolding

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | **i32** | \\[a\\] number of units held. | 
**asset_id** | **i32** | Asset ID of the holding. | 
**creator** | **String** | Address that created this asset. This is the address where the parameters for this asset can be found, and also the address where unwanted asset units can be sent in the worst case. | 
**deleted** | Option<**bool**> | Whether or not the asset holding is currently deleted from its account. | [optional]
**is_frozen** | **bool** | \\[f\\] whether or not the holding is frozen. | 
**opted_in_at_round** | Option<**i32**> | Round during which the account opted into this asset holding. | [optional]
**opted_out_at_round** | Option<**i32**> | Round during which the account opted out of this asset holding. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


