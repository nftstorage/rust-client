# Rust API client for nftstorage

# API Reference


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0
- Package version: 1.0.0
- Build package: org.openapitools.codegen.languages.RustClientCodegen
For more information, please visit [https://nft.storage](https://nft.storage)

## Installation

Put the package under your project folder and add the following to `Cargo.toml` under `[dependencies]`:

```
    openapi = { path = "./generated" }
```

## Documentation for API Endpoints

All URIs are relative to *https://nft.storage/api*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*NFTStorageAPI* | [**delete**](docs/NFTStorageAPI.md#delete) | **delete** /{cid} | Stop storing the content with the passed CID
*NFTStorageAPI* | [**list**](docs/NFTStorageAPI.md#list) | **get** / | List all stored files
*NFTStorageAPI* | [**status**](docs/NFTStorageAPI.md#status) | **get** /{cid} | Get information for the stored file CID
*NFTStorageAPI* | [**store**](docs/NFTStorageAPI.md#store) | **post** /upload | Store a file


## Documentation For Models

 - [Deal](docs/Deal.md)
 - [DeleteResponse](docs/DeleteResponse.md)
 - [ErrorResponse](docs/ErrorResponse.md)
 - [ErrorResponseError](docs/ErrorResponseError.md)
 - [ForbiddenErrorResponse](docs/ForbiddenErrorResponse.md)
 - [ForbiddenErrorResponseError](docs/ForbiddenErrorResponseError.md)
 - [GetResponse](docs/GetResponse.md)
 - [Links](docs/Links.md)
 - [LinksFile](docs/LinksFile.md)
 - [ListResponse](docs/ListResponse.md)
 - [Nft](docs/Nft.md)
 - [NftDeals](docs/NftDeals.md)
 - [Pin](docs/Pin.md)
 - [PinStatus](docs/PinStatus.md)
 - [UnauthorizedErrorResponse](docs/UnauthorizedErrorResponse.md)
 - [UnauthorizedErrorResponseError](docs/UnauthorizedErrorResponseError.md)
 - [UploadResponse](docs/UploadResponse.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author


