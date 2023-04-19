# \LookupApi

All URIs are relative to *https://example.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**lookup_account_app_local_states**](LookupApi.md#lookup_account_app_local_states) | **GET** /v2/accounts/{account-id}/apps-local-state | 
[**lookup_account_assets**](LookupApi.md#lookup_account_assets) | **GET** /v2/accounts/{account-id}/assets | 
[**lookup_account_by_id**](LookupApi.md#lookup_account_by_id) | **GET** /v2/accounts/{account-id} | 
[**lookup_account_created_applications**](LookupApi.md#lookup_account_created_applications) | **GET** /v2/accounts/{account-id}/created-applications | 
[**lookup_account_created_assets**](LookupApi.md#lookup_account_created_assets) | **GET** /v2/accounts/{account-id}/created-assets | 
[**lookup_account_transactions**](LookupApi.md#lookup_account_transactions) | **GET** /v2/accounts/{account-id}/transactions | 
[**lookup_application_box_by_id_and_name**](LookupApi.md#lookup_application_box_by_id_and_name) | **GET** /v2/applications/{application-id}/box | Get box information for a given application.
[**lookup_application_by_id**](LookupApi.md#lookup_application_by_id) | **GET** /v2/applications/{application-id} | 
[**lookup_application_logs_by_id**](LookupApi.md#lookup_application_logs_by_id) | **GET** /v2/applications/{application-id}/logs | 
[**lookup_asset_balances**](LookupApi.md#lookup_asset_balances) | **GET** /v2/assets/{asset-id}/balances | 
[**lookup_asset_by_id**](LookupApi.md#lookup_asset_by_id) | **GET** /v2/assets/{asset-id} | 
[**lookup_asset_transactions**](LookupApi.md#lookup_asset_transactions) | **GET** /v2/assets/{asset-id}/transactions | 
[**lookup_block**](LookupApi.md#lookup_block) | **GET** /v2/blocks/{round-number} | 
[**lookup_transaction**](LookupApi.md#lookup_transaction) | **GET** /v2/transactions/{txid} | 



## lookup_account_app_local_states

> crate::models::LookupAccountAppLocalStates200Response lookup_account_app_local_states(account_id, application_id, include_all, limit, next)


Lookup an account's asset holdings, optionally for a specific ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account string | [required] |
**application_id** | Option<**i32**> | Application ID |  |
**include_all** | Option<**bool**> | Include all items including closed accounts, deleted applications, destroyed assets, opted-out asset holdings, and closed-out application localstates. |  |
**limit** | Option<**i32**> | Maximum number of results to return. There could be additional pages even if the limit is not reached. |  |
**next** | Option<**String**> | The next page of results. Use the next token provided by the previous results. |  |

### Return type

[**crate::models::LookupAccountAppLocalStates200Response**](lookupAccountAppLocalStates_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_account_assets

> crate::models::LookupAccountAssets200Response lookup_account_assets(account_id, asset_id, include_all, limit, next)


Lookup an account's asset holdings, optionally for a specific ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account string | [required] |
**asset_id** | Option<**i32**> | Asset ID |  |
**include_all** | Option<**bool**> | Include all items including closed accounts, deleted applications, destroyed assets, opted-out asset holdings, and closed-out application localstates. |  |
**limit** | Option<**i32**> | Maximum number of results to return. There could be additional pages even if the limit is not reached. |  |
**next** | Option<**String**> | The next page of results. Use the next token provided by the previous results. |  |

### Return type

[**crate::models::LookupAccountAssets200Response**](lookupAccountAssets_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_account_by_id

> crate::models::LookupAccountById200Response lookup_account_by_id(account_id, round, include_all, exclude)


Lookup account information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account string | [required] |
**round** | Option<**i32**> | Include results for the specified round. |  |
**include_all** | Option<**bool**> | Include all items including closed accounts, deleted applications, destroyed assets, opted-out asset holdings, and closed-out application localstates. |  |
**exclude** | Option<[**Vec<String>**](String.md)> | Exclude additional items such as asset holdings, application local data stored for this account, asset parameters created by this account, and application parameters created by this account. |  |

### Return type

[**crate::models::LookupAccountById200Response**](lookupAccountByID_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_account_created_applications

> crate::models::LookupAccountCreatedApplications200Response lookup_account_created_applications(account_id, application_id, include_all, limit, next)


Lookup an account's created application parameters, optionally for a specific ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account string | [required] |
**application_id** | Option<**i32**> | Application ID |  |
**include_all** | Option<**bool**> | Include all items including closed accounts, deleted applications, destroyed assets, opted-out asset holdings, and closed-out application localstates. |  |
**limit** | Option<**i32**> | Maximum number of results to return. There could be additional pages even if the limit is not reached. |  |
**next** | Option<**String**> | The next page of results. Use the next token provided by the previous results. |  |

### Return type

[**crate::models::LookupAccountCreatedApplications200Response**](lookupAccountCreatedApplications_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_account_created_assets

> crate::models::LookupAccountCreatedAssets200Response lookup_account_created_assets(account_id, asset_id, include_all, limit, next)


Lookup an account's created asset parameters, optionally for a specific ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account string | [required] |
**asset_id** | Option<**i32**> | Asset ID |  |
**include_all** | Option<**bool**> | Include all items including closed accounts, deleted applications, destroyed assets, opted-out asset holdings, and closed-out application localstates. |  |
**limit** | Option<**i32**> | Maximum number of results to return. There could be additional pages even if the limit is not reached. |  |
**next** | Option<**String**> | The next page of results. Use the next token provided by the previous results. |  |

### Return type

[**crate::models::LookupAccountCreatedAssets200Response**](lookupAccountCreatedAssets_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_account_transactions

> crate::models::LookupAccountTransactions200Response lookup_account_transactions(account_id, limit, next, note_prefix, tx_type, sig_type, txid, round, min_round, max_round, asset_id, before_time, after_time, currency_greater_than, currency_less_than, rekey_to)


Lookup account transactions. Transactions are returned newest to oldest.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account string | [required] |
**limit** | Option<**i32**> | Maximum number of results to return. There could be additional pages even if the limit is not reached. |  |
**next** | Option<**String**> | The next page of results. Use the next token provided by the previous results. |  |
**note_prefix** | Option<**String**> | Specifies a prefix which must be contained in the note field. |  |
**tx_type** | Option<**String**> |  |  |
**sig_type** | Option<**String**> | SigType filters just results using the specified type of signature: * sig - Standard * msig - MultiSig * lsig - LogicSig |  |
**txid** | Option<**String**> | Lookup the specific transaction by ID. |  |
**round** | Option<**i32**> | Include results for the specified round. |  |
**min_round** | Option<**i32**> | Include results at or after the specified min-round. |  |
**max_round** | Option<**i32**> | Include results at or before the specified max-round. |  |
**asset_id** | Option<**i32**> | Asset ID |  |
**before_time** | Option<**String**> | Include results before the given time. Must be an RFC 3339 formatted string. |  |
**after_time** | Option<**String**> | Include results after the given time. Must be an RFC 3339 formatted string. |  |
**currency_greater_than** | Option<**i32**> | Results should have an amount greater than this value. MicroAlgos are the default currency unless an asset-id is provided, in which case the asset will be used. |  |
**currency_less_than** | Option<**i32**> | Results should have an amount less than this value. MicroAlgos are the default currency unless an asset-id is provided, in which case the asset will be used. |  |
**rekey_to** | Option<**bool**> | Include results which include the rekey-to field. |  |

### Return type

[**crate::models::LookupAccountTransactions200Response**](lookupAccountTransactions_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_application_box_by_id_and_name

> crate::models::Box lookup_application_box_by_id_and_name(application_id, name)
Get box information for a given application.

Given an application ID and box name, returns base64 encoded box name and value. Box names must be in the goal app call arg form 'encoding:value'. For ints, use the form 'int:1234'. For raw bytes, encode base 64 and use 'b64' prefix as in 'b64:A=='. For printable strings, use the form 'str:hello'. For addresses, use the form 'addr:XYZ...'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **i32** |  | [required] |
**name** | **String** | A box name in goal-arg form 'encoding:value'. For ints, use the form 'int:1234'. For raw bytes, use the form 'b64:A=='. For printable strings, use the form 'str:hello'. For addresses, use the form 'addr:XYZ...'. | [required] |

### Return type

[**crate::models::Box**](Box.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_application_by_id

> crate::models::LookupApplicationById200Response lookup_application_by_id(application_id, include_all)


Lookup application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **i32** |  | [required] |
**include_all** | Option<**bool**> | Include all items including closed accounts, deleted applications, destroyed assets, opted-out asset holdings, and closed-out application localstates. |  |

### Return type

[**crate::models::LookupApplicationById200Response**](lookupApplicationByID_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_application_logs_by_id

> crate::models::LookupApplicationLogsById200Response lookup_application_logs_by_id(application_id, limit, next, txid, min_round, max_round, sender_address)


Lookup application logs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **i32** |  | [required] |
**limit** | Option<**i32**> | Maximum number of results to return. There could be additional pages even if the limit is not reached. |  |
**next** | Option<**String**> | The next page of results. Use the next token provided by the previous results. |  |
**txid** | Option<**String**> | Lookup the specific transaction by ID. |  |
**min_round** | Option<**i32**> | Include results at or after the specified min-round. |  |
**max_round** | Option<**i32**> | Include results at or before the specified max-round. |  |
**sender_address** | Option<**String**> | Only include transactions with this sender address. |  |

### Return type

[**crate::models::LookupApplicationLogsById200Response**](lookupApplicationLogsByID_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_asset_balances

> crate::models::LookupAssetBalances200Response lookup_asset_balances(asset_id, include_all, limit, next, currency_greater_than, currency_less_than)


Lookup the list of accounts who hold this asset 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **i32** |  | [required] |
**include_all** | Option<**bool**> | Include all items including closed accounts, deleted applications, destroyed assets, opted-out asset holdings, and closed-out application localstates. |  |
**limit** | Option<**i32**> | Maximum number of results to return. There could be additional pages even if the limit is not reached. |  |
**next** | Option<**String**> | The next page of results. Use the next token provided by the previous results. |  |
**currency_greater_than** | Option<**i32**> | Results should have an amount greater than this value. MicroAlgos are the default currency unless an asset-id is provided, in which case the asset will be used. |  |
**currency_less_than** | Option<**i32**> | Results should have an amount less than this value. MicroAlgos are the default currency unless an asset-id is provided, in which case the asset will be used. |  |

### Return type

[**crate::models::LookupAssetBalances200Response**](lookupAssetBalances_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_asset_by_id

> crate::models::LookupAssetById200Response lookup_asset_by_id(asset_id, include_all)


Lookup asset information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **i32** |  | [required] |
**include_all** | Option<**bool**> | Include all items including closed accounts, deleted applications, destroyed assets, opted-out asset holdings, and closed-out application localstates. |  |

### Return type

[**crate::models::LookupAssetById200Response**](lookupAssetByID_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_asset_transactions

> crate::models::LookupAccountTransactions200Response lookup_asset_transactions(asset_id, limit, next, note_prefix, tx_type, sig_type, txid, round, min_round, max_round, before_time, after_time, currency_greater_than, currency_less_than, address, address_role, exclude_close_to, rekey_to)


Lookup transactions for an asset. Transactions are returned oldest to newest.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **i32** |  | [required] |
**limit** | Option<**i32**> | Maximum number of results to return. There could be additional pages even if the limit is not reached. |  |
**next** | Option<**String**> | The next page of results. Use the next token provided by the previous results. |  |
**note_prefix** | Option<**String**> | Specifies a prefix which must be contained in the note field. |  |
**tx_type** | Option<**String**> |  |  |
**sig_type** | Option<**String**> | SigType filters just results using the specified type of signature: * sig - Standard * msig - MultiSig * lsig - LogicSig |  |
**txid** | Option<**String**> | Lookup the specific transaction by ID. |  |
**round** | Option<**i32**> | Include results for the specified round. |  |
**min_round** | Option<**i32**> | Include results at or after the specified min-round. |  |
**max_round** | Option<**i32**> | Include results at or before the specified max-round. |  |
**before_time** | Option<**String**> | Include results before the given time. Must be an RFC 3339 formatted string. |  |
**after_time** | Option<**String**> | Include results after the given time. Must be an RFC 3339 formatted string. |  |
**currency_greater_than** | Option<**i32**> | Results should have an amount greater than this value. MicroAlgos are the default currency unless an asset-id is provided, in which case the asset will be used. |  |
**currency_less_than** | Option<**i32**> | Results should have an amount less than this value. MicroAlgos are the default currency unless an asset-id is provided, in which case the asset will be used. |  |
**address** | Option<**String**> | Only include transactions with this address in one of the transaction fields. |  |
**address_role** | Option<**String**> | Combine with the address parameter to define what type of address to search for. |  |
**exclude_close_to** | Option<**bool**> | Combine with address and address-role parameters to define what type of address to search for. The close to fields are normally treated as a receiver, if you would like to exclude them set this parameter to true. |  |
**rekey_to** | Option<**bool**> | Include results which include the rekey-to field. |  |

### Return type

[**crate::models::LookupAccountTransactions200Response**](lookupAccountTransactions_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_block

> crate::models::Block lookup_block(round_number, header_only)


Lookup block.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**round_number** | **i32** | Round number | [required] |
**header_only** | Option<**bool**> | Header only flag. When this is set to true, returned block does not contain the transactions |  |

### Return type

[**crate::models::Block**](Block.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_transaction

> crate::models::LookupTransaction200Response lookup_transaction(txid)


Lookup a single transaction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**txid** | **String** |  | [required] |

### Return type

[**crate::models::LookupTransaction200Response**](lookupTransaction_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

