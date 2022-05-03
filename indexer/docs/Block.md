# Block

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**genesis_hash** | **String** | \\[gh\\] hash to which this block belongs. | 
**genesis_id** | **String** | \\[gen\\] ID to which this block belongs. | 
**previous_block_hash** | **String** | \\[prev\\] Previous block hash. | 
**rewards** | Option<[**crate::models::BlockRewards**](BlockRewards.md)> |  | [optional]
**round** | **i32** | \\[rnd\\] Current round on which this block was appended to the chain. | 
**seed** | **String** | \\[seed\\] Sortition seed. | 
**timestamp** | **i32** | \\[ts\\] Block creation timestamp in seconds since eposh | 
**transactions** | Option<[**Vec<crate::models::Transaction>**](Transaction.md)> | \\[txns\\] list of transactions corresponding to a given round. | [optional]
**transactions_root** | **String** | \\[txn\\] TransactionsRoot authenticates the set of transactions appearing in the block. More specifically, it's the root of a merkle tree whose leaves are the block's Txids, in lexicographic order. For the empty block, it's 0. Note that the TxnRoot does not authenticate the signatures on the transactions, only the transactions themselves. Two blocks with the same transactions but in a different order and with different signatures will have the same TxnRoot. | 
**txn_counter** | Option<**i32**> | \\[tc\\] TxnCounter counts the number of transactions committed in the ledger, from the time at which support for this feature was introduced.  Specifically, TxnCounter is the number of the next transaction that will be committed after this block.  It is 0 when no transactions have ever been committed (since TxnCounter started being supported). | [optional]
**upgrade_state** | Option<[**crate::models::BlockUpgradeState**](BlockUpgradeState.md)> |  | [optional]
**upgrade_vote** | Option<[**crate::models::BlockUpgradeVote**](BlockUpgradeVote.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


