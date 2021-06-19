# \PrivateApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**abort_catchup**](PrivateApi.md#abort_catchup) | **delete** /v2/catchup/{catchpoint} | Aborts a catchpoint catchup.
[**register_participation_keys**](PrivateApi.md#register_participation_keys) | **post** /v2/register-participation-keys/{address} | 
[**shutdown_node**](PrivateApi.md#shutdown_node) | **post** /v2/shutdown | 
[**start_catchup**](PrivateApi.md#start_catchup) | **post** /v2/catchup/{catchpoint} | Starts a catchpoint catchup.



## abort_catchup

> crate::models::InlineResponse2004 abort_catchup(catchpoint)
Aborts a catchpoint catchup.

Given a catchpoint, it aborts catching up to this catchpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**catchpoint** | **String** | A catch point | [required] |

### Return type

[**crate::models::InlineResponse2004**](inline_response_200_4.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_participation_keys

> crate::models::InlineResponse2006 register_participation_keys(address, fee, key_dilution, round_last_valid, no_wait)


Generate (or renew) and register participation keys on the node for a given account address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The `account-id` to update, or `all` to update all accounts. | [required] |
**fee** | Option<**i32**> | The fee to use when submitting key registration transactions. Defaults to the suggested fee. |  |[default to 1000]
**key_dilution** | Option<**i32**> | value to use for two-level participation key. |  |
**round_last_valid** | Option<**i32**> | The last round for which the generated participation keys will be valid. |  |
**no_wait** | Option<**bool**> | Don't wait for transaction to commit before returning response. |  |

### Return type

[**crate::models::InlineResponse2006**](inline_response_200_6.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shutdown_node

> serde_json::Value shutdown_node(timeout)


Special management endpoint to shutdown the node. Optionally provide a timeout parameter to indicate that the node should begin shutting down after a number of seconds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timeout** | Option<**i32**> |  |  |[default to 0]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_catchup

> crate::models::InlineResponse2003 start_catchup(catchpoint)
Starts a catchpoint catchup.

Given a catchpoint, it starts catching up to this catchpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**catchpoint** | **String** | A catch point | [required] |

### Return type

[**crate::models::InlineResponse2003**](inline_response_200_3.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

