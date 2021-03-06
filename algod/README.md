# Rust API client for openapi

API endpoint for algod operations.

For more information, please visit [https://www.algorand.com/get-in-touch/contact](https://www.algorand.com/get-in-touch/contact)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 0.0.1
- Package version: 0.0.1
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*CommonApi* | [**get_genesis**](docs/CommonApi.md#get_genesis) | **GET** /genesis | Gets the genesis information.
*CommonApi* | [**get_version**](docs/CommonApi.md#get_version) | **GET** /versions | 
*CommonApi* | [**health_check**](docs/CommonApi.md#health_check) | **GET** /health | Returns OK if healthy.
*CommonApi* | [**metrics**](docs/CommonApi.md#metrics) | **GET** /metrics | Return metrics about algod functioning.
*CommonApi* | [**swagger_json**](docs/CommonApi.md#swagger_json) | **GET** /swagger.json | Gets the current swagger spec.
*DefaultApi* | [**account_application_information**](docs/DefaultApi.md#account_application_information) | **GET** /v2/accounts/{address}/applications/{application-id} | Get account information about a given app.
*DefaultApi* | [**account_asset_information**](docs/DefaultApi.md#account_asset_information) | **GET** /v2/accounts/{address}/assets/{asset-id} | Get account information about a given asset.
*DefaultApi* | [**account_information**](docs/DefaultApi.md#account_information) | **GET** /v2/accounts/{address} | Get account information.
*DefaultApi* | [**get_application_by_id**](docs/DefaultApi.md#get_application_by_id) | **GET** /v2/applications/{application-id} | Get application information.
*DefaultApi* | [**get_asset_by_id**](docs/DefaultApi.md#get_asset_by_id) | **GET** /v2/assets/{asset-id} | Get asset information.
*DefaultApi* | [**get_block**](docs/DefaultApi.md#get_block) | **GET** /v2/blocks/{round} | Get the block for the given round.
*DefaultApi* | [**get_pending_transactions**](docs/DefaultApi.md#get_pending_transactions) | **GET** /v2/transactions/pending | Get a list of unconfirmed transactions currently in the transaction pool.
*DefaultApi* | [**get_pending_transactions_by_address**](docs/DefaultApi.md#get_pending_transactions_by_address) | **GET** /v2/accounts/{address}/transactions/pending | Get a list of unconfirmed transactions currently in the transaction pool by address.
*DefaultApi* | [**get_proof**](docs/DefaultApi.md#get_proof) | **GET** /v2/blocks/{round}/transactions/{txid}/proof | Get a Merkle proof for a transaction in a block.
*DefaultApi* | [**get_status**](docs/DefaultApi.md#get_status) | **GET** /v2/status | Gets the current node status.
*DefaultApi* | [**get_supply**](docs/DefaultApi.md#get_supply) | **GET** /v2/ledger/supply | Get the current supply reported by the ledger.
*DefaultApi* | [**pending_transaction_information**](docs/DefaultApi.md#pending_transaction_information) | **GET** /v2/transactions/pending/{txid} | Get a specific pending transaction.
*DefaultApi* | [**raw_transaction**](docs/DefaultApi.md#raw_transaction) | **POST** /v2/transactions | Broadcasts a raw transaction to the network.
*DefaultApi* | [**teal_compile**](docs/DefaultApi.md#teal_compile) | **POST** /v2/teal/compile | Compile TEAL source code to binary, produce its hash
*DefaultApi* | [**teal_disassemble**](docs/DefaultApi.md#teal_disassemble) | **POST** /v2/teal/disassemble | Disassemble program bytes into the TEAL source code.
*DefaultApi* | [**teal_dryrun**](docs/DefaultApi.md#teal_dryrun) | **POST** /v2/teal/dryrun | Provide debugging information for a transaction (or group).
*DefaultApi* | [**transaction_params**](docs/DefaultApi.md#transaction_params) | **GET** /v2/transactions/params | Get parameters for constructing a new transaction
*DefaultApi* | [**wait_for_block**](docs/DefaultApi.md#wait_for_block) | **GET** /v2/status/wait-for-block-after/{round} | Gets the node status after waiting for the given round.
*PrivateApi* | [**abort_catchup**](docs/PrivateApi.md#abort_catchup) | **DELETE** /v2/catchup/{catchpoint} | Aborts a catchpoint catchup.
*PrivateApi* | [**add_participation_key**](docs/PrivateApi.md#add_participation_key) | **POST** /v2/participation | Add a participation key to the node
*PrivateApi* | [**append_keys**](docs/PrivateApi.md#append_keys) | **POST** /v2/participation/{participation-id} | Append state proof keys to a participation key
*PrivateApi* | [**delete_participation_key_by_id**](docs/PrivateApi.md#delete_participation_key_by_id) | **DELETE** /v2/participation/{participation-id} | Delete a given participation key by ID
*PrivateApi* | [**get_participation_key_by_id**](docs/PrivateApi.md#get_participation_key_by_id) | **GET** /v2/participation/{participation-id} | Get participation key info given a participation ID
*PrivateApi* | [**get_participation_keys**](docs/PrivateApi.md#get_participation_keys) | **GET** /v2/participation | Return a list of participation keys
*PrivateApi* | [**shutdown_node**](docs/PrivateApi.md#shutdown_node) | **POST** /v2/shutdown | 
*PrivateApi* | [**start_catchup**](docs/PrivateApi.md#start_catchup) | **POST** /v2/catchup/{catchpoint} | Starts a catchpoint catchup.


## Documentation For Models

 - [Account](docs/Account.md)
 - [AccountParticipation](docs/AccountParticipation.md)
 - [AccountStateDelta](docs/AccountStateDelta.md)
 - [Application](docs/Application.md)
 - [ApplicationLocalState](docs/ApplicationLocalState.md)
 - [ApplicationParams](docs/ApplicationParams.md)
 - [ApplicationStateSchema](docs/ApplicationStateSchema.md)
 - [Asset](docs/Asset.md)
 - [AssetHolding](docs/AssetHolding.md)
 - [AssetParams](docs/AssetParams.md)
 - [BuildVersion](docs/BuildVersion.md)
 - [DryrunRequest](docs/DryrunRequest.md)
 - [DryrunSource](docs/DryrunSource.md)
 - [DryrunState](docs/DryrunState.md)
 - [DryrunTxnResult](docs/DryrunTxnResult.md)
 - [ErrorResponse](docs/ErrorResponse.md)
 - [EvalDelta](docs/EvalDelta.md)
 - [EvalDeltaKeyValue](docs/EvalDeltaKeyValue.md)
 - [InlineResponse200](docs/InlineResponse200.md)
 - [InlineResponse2001](docs/InlineResponse2001.md)
 - [InlineResponse20011](docs/InlineResponse20011.md)
 - [InlineResponse20012](docs/InlineResponse20012.md)
 - [InlineResponse20013](docs/InlineResponse20013.md)
 - [InlineResponse20014](docs/InlineResponse20014.md)
 - [InlineResponse20015](docs/InlineResponse20015.md)
 - [InlineResponse2002](docs/InlineResponse2002.md)
 - [InlineResponse2003](docs/InlineResponse2003.md)
 - [InlineResponse2004](docs/InlineResponse2004.md)
 - [InlineResponse2005](docs/InlineResponse2005.md)
 - [InlineResponse2006](docs/InlineResponse2006.md)
 - [InlineResponse2007](docs/InlineResponse2007.md)
 - [InlineResponse2008](docs/InlineResponse2008.md)
 - [InlineResponse2009](docs/InlineResponse2009.md)
 - [ParticipationKey](docs/ParticipationKey.md)
 - [PendingTransactionResponse](docs/PendingTransactionResponse.md)
 - [TealKeyValue](docs/TealKeyValue.md)
 - [TealValue](docs/TealValue.md)
 - [Version](docs/Version.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

contact@algorand.com

