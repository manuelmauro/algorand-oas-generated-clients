# BlockRewards

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fee_sink** | **String** | \\[fees\\] accepts transaction fees, it can only spend to the incentive pool. | 
**rewards_calculation_round** | **i32** | \\[rwcalr\\] number of leftover MicroAlgos after the distribution of rewards-rate MicroAlgos for every reward unit in the next round. | 
**rewards_level** | **i32** | \\[earn\\] How many rewards, in MicroAlgos, have been distributed to each RewardUnit of MicroAlgos since genesis. | 
**rewards_pool** | **String** | \\[rwd\\] accepts periodic injections from the fee-sink and continually redistributes them as rewards. | 
**rewards_rate** | **i32** | \\[rate\\] Number of new MicroAlgos added to the participation stake from rewards at the next round. | 
**rewards_residue** | **i32** | \\[frac\\] Number of leftover MicroAlgos after the distribution of RewardsRate/rewardUnits MicroAlgos for every reward unit in the next round. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


