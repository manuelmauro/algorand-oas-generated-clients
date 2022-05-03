# ApplicationLocalState

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**closed_out_at_round** | Option<**i32**> | Round when account closed out of the application. | [optional]
**deleted** | Option<**bool**> | Whether or not the application local state is currently deleted from its account. | [optional]
**id** | **i32** | The application which this local state is for. | 
**key_value** | Option<[**Vec<crate::models::TealKeyValue>**](TealKeyValue.md)> | Represents a key-value store for use in an application. | [optional]
**opted_in_at_round** | Option<**i32**> | Round when the account opted into the application. | [optional]
**schema** | [**crate::models::ApplicationStateSchema**](ApplicationStateSchema.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


