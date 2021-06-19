# TransactionPayment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | **i32** | \\[amt\\] number of MicroAlgos intended to be transferred. | 
**close_amount** | Option<**i32**> | Number of MicroAlgos that were sent to the close-remainder-to address when closing the sender account. | [optional]
**close_remainder_to** | Option<**String**> | \\[close\\] when set, indicates that the sending account should be closed and all remaining funds be transferred to this address. | [optional]
**receiver** | **String** | \\[rcv\\] receiver's address. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


