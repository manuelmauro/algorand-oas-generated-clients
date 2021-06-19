# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_information**](DefaultApi.md#account_information) | **get** /v2/accounts/{address} | Get account information.
[**get_application_by_id**](DefaultApi.md#get_application_by_id) | **get** /v2/applications/{application-id} | Get application information.
[**get_asset_by_id**](DefaultApi.md#get_asset_by_id) | **get** /v2/assets/{asset-id} | Get asset information.
[**get_block**](DefaultApi.md#get_block) | **get** /v2/blocks/{round} | Get the block for the given round.
[**get_pending_transactions**](DefaultApi.md#get_pending_transactions) | **get** /v2/transactions/pending | Get a list of unconfirmed transactions currently in the transaction pool.
[**get_pending_transactions_by_address**](DefaultApi.md#get_pending_transactions_by_address) | **get** /v2/accounts/{address}/transactions/pending | Get a list of unconfirmed transactions currently in the transaction pool by address.
[**get_proof**](DefaultApi.md#get_proof) | **get** /v2/blocks/{round}/transactions/{txid}/proof | Get a Merkle proof for a transaction in a block.
[**get_status**](DefaultApi.md#get_status) | **get** /v2/status | Gets the current node status.
[**get_supply**](DefaultApi.md#get_supply) | **get** /v2/ledger/supply | Get the current supply reported by the ledger.
[**pending_transaction_information**](DefaultApi.md#pending_transaction_information) | **get** /v2/transactions/pending/{txid} | Get a specific pending transaction.
[**raw_transaction**](DefaultApi.md#raw_transaction) | **post** /v2/transactions | Broadcasts a raw transaction to the network.
[**teal_compile**](DefaultApi.md#teal_compile) | **post** /v2/teal/compile | Compile TEAL source code to binary, produce its hash
[**teal_dryrun**](DefaultApi.md#teal_dryrun) | **post** /v2/teal/dryrun | Provide debugging information for a transaction (or group).
[**transaction_params**](DefaultApi.md#transaction_params) | **get** /v2/transactions/params | Get parameters for constructing a new transaction
[**wait_for_block**](DefaultApi.md#wait_for_block) | **get** /v2/status/wait-for-block-after/{round} | Gets the node status after waiting for the given round.



## account_information

> crate::models::Account account_information(address, format)
Get account information.

Given a specific account public key, this call returns the accounts status, balance and spendable amounts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | An account public key | [required] |
**format** | Option<**String**> | Configures whether the response object is JSON or MessagePack encoded. |  |

### Return type

[**crate::models::Account**](Account.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_application_by_id

> crate::models::Application get_application_by_id(application_id)
Get application information.

Given a application id, it returns application information including creator, approval and clear programs, global and local schemas, and global state.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **i32** | An application identifier | [required] |

### Return type

[**crate::models::Application**](Application.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_asset_by_id

> crate::models::Asset get_asset_by_id(asset_id)
Get asset information.

Given a asset id, it returns asset information including creator, name, total supply and special addresses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **i32** | An asset identifier | [required] |

### Return type

[**crate::models::Asset**](Asset.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_block

> crate::models::InlineResponse2001 get_block(round, format)
Get the block for the given round.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**round** | **i32** | The round from which to fetch block information. | [required] |
**format** | Option<**String**> | Configures whether the response object is JSON or MessagePack encoded. |  |

### Return type

[**crate::models::InlineResponse2001**](inline_response_200_1.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pending_transactions

> crate::models::InlineResponse200 get_pending_transactions(max, format)
Get a list of unconfirmed transactions currently in the transaction pool.

Get the list of pending transactions, sorted by priority, in decreasing order, truncated at the end at MAX. If MAX = 0, returns all pending transactions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max** | Option<**i32**> | Truncated number of transactions to display. If max=0, returns all pending txns. |  |
**format** | Option<**String**> | Configures whether the response object is JSON or MessagePack encoded. |  |

### Return type

[**crate::models::InlineResponse200**](inline_response_200.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pending_transactions_by_address

> crate::models::InlineResponse200 get_pending_transactions_by_address(address, max, format)
Get a list of unconfirmed transactions currently in the transaction pool by address.

Get the list of pending transactions by address, sorted by priority, in decreasing order, truncated at the end at MAX. If MAX = 0, returns all pending transactions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | An account public key | [required] |
**max** | Option<**i32**> | Truncated number of transactions to display. If max=0, returns all pending txns. |  |
**format** | Option<**String**> | Configures whether the response object is JSON or MessagePack encoded. |  |

### Return type

[**crate::models::InlineResponse200**](inline_response_200.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_proof

> crate::models::InlineResponse2002 get_proof(round, txid, format)
Get a Merkle proof for a transaction in a block.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**round** | **i32** | The round in which the transaction appears. | [required] |
**txid** | **String** | The transaction ID for which to generate a proof. | [required] |
**format** | Option<**String**> | Configures whether the response object is JSON or MessagePack encoded. |  |

### Return type

[**crate::models::InlineResponse2002**](inline_response_200_2.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_status

> crate::models::InlineResponse2007 get_status()
Gets the current node status.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse2007**](inline_response_200_7.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_supply

> crate::models::InlineResponse2005 get_supply()
Get the current supply reported by the ledger.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse2005**](inline_response_200_5.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pending_transaction_information

> crate::models::InlineResponse20011 pending_transaction_information(txid, format)
Get a specific pending transaction.

Given a transaction id of a recently submitted transaction, it returns information about it.  There are several cases when this might succeed: - transaction committed (committed round > 0) - transaction still in the pool (committed round = 0, pool error = \"\") - transaction removed from pool due to error (committed round = 0, pool error != \"\") Or the transaction may have happened sufficiently long ago that the node no longer remembers it, and this will return an error. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**txid** | **String** | A transaction id | [required] |
**format** | Option<**String**> | Configures whether the response object is JSON or MessagePack encoded. |  |

### Return type

[**crate::models::InlineResponse20011**](inline_response_200_11.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## raw_transaction

> crate::models::InlineResponse2006 raw_transaction(rawtxn)
Broadcasts a raw transaction to the network.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rawtxn** | **std::path::PathBuf** | The byte encoded signed transaction to broadcast to network | [required] |

### Return type

[**crate::models::InlineResponse2006**](inline_response_200_6.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/x-binary
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teal_compile

> crate::models::InlineResponse2008 teal_compile(source)
Compile TEAL source code to binary, produce its hash

Given TEAL source code in plain text, return base64 encoded program bytes and base32 SHA512_256 hash of program bytes (Address style). This endpoint is only enabled when a node's configureation file sets EnableDeveloperAPI to true.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source** | **std::path::PathBuf** | TEAL source code to be compiled | [required] |

### Return type

[**crate::models::InlineResponse2008**](inline_response_200_8.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teal_dryrun

> crate::models::InlineResponse2009 teal_dryrun(request)
Provide debugging information for a transaction (or group).

Executes TEAL program(s) in context and returns debugging information about the execution. This endpoint is only enabled when a node's configureation file sets EnableDeveloperAPI to true.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request** | Option<[**DryrunRequest**](DryrunRequest.md)> | Transaction (or group) and any accompanying state-simulation data. |  |

### Return type

[**crate::models::InlineResponse2009**](inline_response_200_9.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json, application/msgpack
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transaction_params

> crate::models::InlineResponse20010 transaction_params()
Get parameters for constructing a new transaction

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse20010**](inline_response_200_10.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wait_for_block

> crate::models::InlineResponse2007 wait_for_block(round)
Gets the node status after waiting for the given round.

Waits for a block to appear after round {round} and returns the node's status at the time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**round** | **i32** | The round to wait until returning status | [required] |

### Return type

[**crate::models::InlineResponse2007**](inline_response_200_7.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

