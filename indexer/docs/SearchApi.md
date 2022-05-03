# \SearchApi

All URIs are relative to *https://example.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search_for_accounts**](SearchApi.md#search_for_accounts) | **GET** /v2/accounts | 
[**search_for_applications**](SearchApi.md#search_for_applications) | **GET** /v2/applications | 
[**search_for_assets**](SearchApi.md#search_for_assets) | **GET** /v2/assets | 
[**search_for_transactions**](SearchApi.md#search_for_transactions) | **GET** /v2/transactions | 



## search_for_accounts

> crate::models::InlineResponse200 search_for_accounts(asset_id, limit, next, currency_greater_than, include_all, exclude, currency_less_than, auth_addr, round, application_id)


Search for accounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | Option<**i32**> | Asset ID |  |
**limit** | Option<**i32**> | Maximum number of results to return. There could be additional pages even if the limit is not reached. |  |
**next** | Option<**String**> | The next page of results. Use the next token provided by the previous results. |  |
**currency_greater_than** | Option<**i32**> | Results should have an amount greater than this value. MicroAlgos are the default currency unless an asset-id is provided, in which case the asset will be used. |  |
**include_all** | Option<**bool**> | Include all items including closed accounts, deleted applications, destroyed assets, opted-out asset holdings, and closed-out application localstates. |  |
**exclude** | Option<[**Vec<String>**](String.md)> | Exclude additional items such as asset holdings, application local data stored for this account, asset parameters created by this account, and application parameters created by this account. |  |
**currency_less_than** | Option<**i32**> | Results should have an amount less than this value. MicroAlgos are the default currency unless an asset-id is provided, in which case the asset will be used. |  |
**auth_addr** | Option<**String**> | Include accounts configured to use this spending key. |  |
**round** | Option<**i32**> | Include results for the specified round. For performance reasons, this parameter may be disabled on some configurations. |  |
**application_id** | Option<**i32**> | Application ID |  |

### Return type

[**crate::models::InlineResponse200**](inline_response_200.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_for_applications

> crate::models::InlineResponse2004 search_for_applications(application_id, creator, include_all, limit, next)


Search for applications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | Option<**i32**> | Application ID |  |
**creator** | Option<**String**> | Filter just applications with the given creator address. |  |
**include_all** | Option<**bool**> | Include all items including closed accounts, deleted applications, destroyed assets, opted-out asset holdings, and closed-out application localstates. |  |
**limit** | Option<**i32**> | Maximum number of results to return. There could be additional pages even if the limit is not reached. |  |
**next** | Option<**String**> | The next page of results. Use the next token provided by the previous results. |  |

### Return type

[**crate::models::InlineResponse2004**](inline_response_200_4.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_for_assets

> crate::models::InlineResponse2005 search_for_assets(include_all, limit, next, creator, name, unit, asset_id)


Search for assets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_all** | Option<**bool**> | Include all items including closed accounts, deleted applications, destroyed assets, opted-out asset holdings, and closed-out application localstates. |  |
**limit** | Option<**i32**> | Maximum number of results to return. There could be additional pages even if the limit is not reached. |  |
**next** | Option<**String**> | The next page of results. Use the next token provided by the previous results. |  |
**creator** | Option<**String**> | Filter just assets with the given creator address. |  |
**name** | Option<**String**> | Filter just assets with the given name. |  |
**unit** | Option<**String**> | Filter just assets with the given unit. |  |
**asset_id** | Option<**i32**> | Asset ID |  |

### Return type

[**crate::models::InlineResponse2005**](inline_response_200_5.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_for_transactions

> crate::models::InlineResponse2006 search_for_transactions(limit, next, note_prefix, tx_type, sig_type, txid, round, min_round, max_round, asset_id, before_time, after_time, currency_greater_than, currency_less_than, address, address_role, exclude_close_to, rekey_to, application_id)


Search for transactions. Transactions are returned oldest to newest unless the address parameter is used, in which case results are returned newest to oldest.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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
**address** | Option<**String**> | Only include transactions with this address in one of the transaction fields. |  |
**address_role** | Option<**String**> | Combine with the address parameter to define what type of address to search for. |  |
**exclude_close_to** | Option<**bool**> | Combine with address and address-role parameters to define what type of address to search for. The close to fields are normally treated as a receiver, if you would like to exclude them set this parameter to true. |  |
**rekey_to** | Option<**bool**> | Include results which include the rekey-to field. |  |
**application_id** | Option<**i32**> | Application ID |  |

### Return type

[**crate::models::InlineResponse2006**](inline_response_200_6.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

