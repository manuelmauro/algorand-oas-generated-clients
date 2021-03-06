# Rust API client for openapi

Algorand ledger analytics API.

For more information, please visit [https://www.algorand.com/get-in-touch/contact](https://www.algorand.com/get-in-touch/contact)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 2.0
- Package version: 2.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *https://example.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*CommonApi* | [**make_health_check**](docs/CommonApi.md#make_health_check) | **GET** /health | Returns 200 if healthy.
*LookupApi* | [**lookup_account_app_local_states**](docs/LookupApi.md#lookup_account_app_local_states) | **GET** /v2/accounts/{account-id}/apps-local-state | 
*LookupApi* | [**lookup_account_assets**](docs/LookupApi.md#lookup_account_assets) | **GET** /v2/accounts/{account-id}/assets | 
*LookupApi* | [**lookup_account_by_id**](docs/LookupApi.md#lookup_account_by_id) | **GET** /v2/accounts/{account-id} | 
*LookupApi* | [**lookup_account_created_applications**](docs/LookupApi.md#lookup_account_created_applications) | **GET** /v2/accounts/{account-id}/created-applications | 
*LookupApi* | [**lookup_account_created_assets**](docs/LookupApi.md#lookup_account_created_assets) | **GET** /v2/accounts/{account-id}/created-assets | 
*LookupApi* | [**lookup_account_transactions**](docs/LookupApi.md#lookup_account_transactions) | **GET** /v2/accounts/{account-id}/transactions | 
*LookupApi* | [**lookup_application_by_id**](docs/LookupApi.md#lookup_application_by_id) | **GET** /v2/applications/{application-id} | 
*LookupApi* | [**lookup_application_logs_by_id**](docs/LookupApi.md#lookup_application_logs_by_id) | **GET** /v2/applications/{application-id}/logs | 
*LookupApi* | [**lookup_asset_balances**](docs/LookupApi.md#lookup_asset_balances) | **GET** /v2/assets/{asset-id}/balances | 
*LookupApi* | [**lookup_asset_by_id**](docs/LookupApi.md#lookup_asset_by_id) | **GET** /v2/assets/{asset-id} | 
*LookupApi* | [**lookup_asset_transactions**](docs/LookupApi.md#lookup_asset_transactions) | **GET** /v2/assets/{asset-id}/transactions | 
*LookupApi* | [**lookup_block**](docs/LookupApi.md#lookup_block) | **GET** /v2/blocks/{round-number} | 
*LookupApi* | [**lookup_transaction**](docs/LookupApi.md#lookup_transaction) | **GET** /v2/transactions/{txid} | 
*SearchApi* | [**search_for_accounts**](docs/SearchApi.md#search_for_accounts) | **GET** /v2/accounts | 
*SearchApi* | [**search_for_applications**](docs/SearchApi.md#search_for_applications) | **GET** /v2/applications | 
*SearchApi* | [**search_for_assets**](docs/SearchApi.md#search_for_assets) | **GET** /v2/assets | 
*SearchApi* | [**search_for_transactions**](docs/SearchApi.md#search_for_transactions) | **GET** /v2/transactions | 


## Documentation For Models

 - [Account](docs/Account.md)
 - [AccountParticipation](docs/AccountParticipation.md)
 - [AccountStateDelta](docs/AccountStateDelta.md)
 - [Application](docs/Application.md)
 - [ApplicationLocalState](docs/ApplicationLocalState.md)
 - [ApplicationLogData](docs/ApplicationLogData.md)
 - [ApplicationParams](docs/ApplicationParams.md)
 - [ApplicationStateSchema](docs/ApplicationStateSchema.md)
 - [Asset](docs/Asset.md)
 - [AssetHolding](docs/AssetHolding.md)
 - [AssetParams](docs/AssetParams.md)
 - [Block](docs/Block.md)
 - [BlockRewards](docs/BlockRewards.md)
 - [BlockUpgradeState](docs/BlockUpgradeState.md)
 - [BlockUpgradeVote](docs/BlockUpgradeVote.md)
 - [EvalDelta](docs/EvalDelta.md)
 - [EvalDeltaKeyValue](docs/EvalDeltaKeyValue.md)
 - [HealthCheck](docs/HealthCheck.md)
 - [InlineResponse200](docs/InlineResponse200.md)
 - [InlineResponse2001](docs/InlineResponse2001.md)
 - [InlineResponse20011](docs/InlineResponse20011.md)
 - [InlineResponse20012](docs/InlineResponse20012.md)
 - [InlineResponse20015](docs/InlineResponse20015.md)
 - [InlineResponse2002](docs/InlineResponse2002.md)
 - [InlineResponse2003](docs/InlineResponse2003.md)
 - [InlineResponse2004](docs/InlineResponse2004.md)
 - [InlineResponse2005](docs/InlineResponse2005.md)
 - [InlineResponse2006](docs/InlineResponse2006.md)
 - [InlineResponse2008](docs/InlineResponse2008.md)
 - [InlineResponse2009](docs/InlineResponse2009.md)
 - [InlineResponse400](docs/InlineResponse400.md)
 - [MiniAssetHolding](docs/MiniAssetHolding.md)
 - [OnCompletion](docs/OnCompletion.md)
 - [StateSchema](docs/StateSchema.md)
 - [TealKeyValue](docs/TealKeyValue.md)
 - [TealValue](docs/TealValue.md)
 - [Transaction](docs/Transaction.md)
 - [TransactionApplication](docs/TransactionApplication.md)
 - [TransactionAssetConfig](docs/TransactionAssetConfig.md)
 - [TransactionAssetFreeze](docs/TransactionAssetFreeze.md)
 - [TransactionAssetTransfer](docs/TransactionAssetTransfer.md)
 - [TransactionKeyreg](docs/TransactionKeyreg.md)
 - [TransactionPayment](docs/TransactionPayment.md)
 - [TransactionSignature](docs/TransactionSignature.md)
 - [TransactionSignatureLogicsig](docs/TransactionSignatureLogicsig.md)
 - [TransactionSignatureMultisig](docs/TransactionSignatureMultisig.md)
 - [TransactionSignatureMultisigSubsignature](docs/TransactionSignatureMultisigSubsignature.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



