# BlockUpgradeState

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**current_protocol** | **String** | \\[proto\\] The current protocol version. | 
**next_protocol** | Option<**String**> | \\[nextproto\\] The next proposed protocol version. | [optional]
**next_protocol_approvals** | Option<**i32**> | \\[nextyes\\] Number of blocks which approved the protocol upgrade. | [optional]
**next_protocol_switch_on** | Option<**i32**> | \\[nextswitch\\] Round on which the protocol upgrade will take effect. | [optional]
**next_protocol_vote_before** | Option<**i32**> | \\[nextbefore\\] Deadline round for this protocol upgrade (No votes will be consider after this round). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


