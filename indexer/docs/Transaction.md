# Transaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**application_transaction** | Option<[**crate::models::TransactionApplication**](TransactionApplication.md)> |  | [optional]
**asset_config_transaction** | Option<[**crate::models::TransactionAssetConfig**](TransactionAssetConfig.md)> |  | [optional]
**asset_freeze_transaction** | Option<[**crate::models::TransactionAssetFreeze**](TransactionAssetFreeze.md)> |  | [optional]
**asset_transfer_transaction** | Option<[**crate::models::TransactionAssetTransfer**](TransactionAssetTransfer.md)> |  | [optional]
**auth_addr** | Option<**String**> | \\[sgnr\\] The address used to sign the transaction. This is used for rekeyed accounts to indicate that the sender address did not sign the transaction. | [optional]
**close_rewards** | Option<**i32**> | \\[rc\\] rewards applied to close-remainder-to account. | [optional]
**closing_amount** | Option<**i32**> | \\[ca\\] closing amount for transaction. | [optional]
**confirmed_round** | Option<**i32**> | Round when the transaction was confirmed. | [optional]
**created_application_index** | Option<**i32**> | Specifies an application index (ID) if an application was created with this transaction. | [optional]
**created_asset_index** | Option<**i32**> | Specifies an asset index (ID) if an asset was created with this transaction. | [optional]
**fee** | **i32** | \\[fee\\] Transaction fee. | 
**first_valid** | **i32** | \\[fv\\] First valid round for this transaction. | 
**genesis_hash** | Option<**String**> | \\[gh\\] Hash of genesis block. | [optional]
**genesis_id** | Option<**String**> | \\[gen\\] genesis block ID. | [optional]
**global_state_delta** | Option<[**Vec<crate::models::EvalDeltaKeyValue>**](EvalDeltaKeyValue.md)> | Application state delta. | [optional]
**group** | Option<**String**> | \\[grp\\] Base64 encoded byte array of a sha512/256 digest. When present indicates that this transaction is part of a transaction group and the value is the sha512/256 hash of the transactions in that group. | [optional]
**id** | **String** | Transaction ID | 
**intra_round_offset** | Option<**i32**> | Offset into the round where this transaction was confirmed. | [optional]
**keyreg_transaction** | Option<[**crate::models::TransactionKeyreg**](TransactionKeyreg.md)> |  | [optional]
**last_valid** | **i32** | \\[lv\\] Last valid round for this transaction. | 
**lease** | Option<**String**> | \\[lx\\] Base64 encoded 32-byte array. Lease enforces mutual exclusion of transactions.  If this field is nonzero, then once the transaction is confirmed, it acquires the lease identified by the (Sender, Lease) pair of the transaction until the LastValid round passes.  While this transaction possesses the lease, no other transaction specifying this lease can be confirmed. | [optional]
**local_state_delta** | Option<[**Vec<crate::models::AccountStateDelta>**](AccountStateDelta.md)> | \\[ld\\] Local state key/value changes for the application being executed by this transaction. | [optional]
**note** | Option<**String**> | \\[note\\] Free form data. | [optional]
**payment_transaction** | Option<[**crate::models::TransactionPayment**](TransactionPayment.md)> |  | [optional]
**receiver_rewards** | Option<**i32**> | \\[rr\\] rewards applied to receiver account. | [optional]
**rekey_to** | Option<**String**> | \\[rekey\\] when included in a valid transaction, the accounts auth addr will be updated with this value and future signatures must be signed with the key represented by this address. | [optional]
**round_time** | Option<**i32**> | Time when the block this transaction is in was confirmed. | [optional]
**sender** | **String** | \\[snd\\] Sender's address. | 
**sender_rewards** | Option<**i32**> | \\[rs\\] rewards applied to sender account. | [optional]
**signature** | [**crate::models::TransactionSignature**](TransactionSignature.md) |  | 
**tx_type** | **String** | \\[type\\] Indicates what type of transaction this is. Different types have different fields.  Valid types, and where their fields are stored: * \\[pay\\] payment-transaction * \\[keyreg\\] keyreg-transaction * \\[acfg\\] asset-config-transaction * \\[axfer\\] asset-transfer-transaction * \\[afrz\\] asset-freeze-transaction * \\[appl\\] application-transaction | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


