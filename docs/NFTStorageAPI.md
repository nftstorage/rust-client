# \NFTStorageAPI

All URIs are relative to *https://nft.storage/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete**](NFTStorageAPI.md#delete) | **delete** /{cid} | Stop storing the content with the passed CID
[**list**](NFTStorageAPI.md#list) | **get** / | List all stored files
[**status**](NFTStorageAPI.md#status) | **get** /{cid} | Get information for the stored file CID
[**store**](NFTStorageAPI.md#store) | **post** /upload | Store a file



## delete

> crate::models::DeleteResponse delete(cid)
Stop storing the content with the passed CID

Stop storing the content with the passed CID on nft.storage. - Unpin the item from the underlying IPFS pinning service. - Cease renewals for expired Filecoin deals involving the CID.    ⚠️ This does not remove the content from the network.  - Does not terminate any established Filecoin deal. - Does not remove the content from other IPFS nodes in the network that already cached or pinned the CID.    Note: the content will remain available if another user has stored the CID with nft.storage. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** | CID for the NFT | [required] |

### Return type

[**crate::models::DeleteResponse**](DeleteResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list

> crate::models::ListResponse list()
List all stored files

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListResponse**](ListResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## status

> crate::models::GetResponse status(cid)
Get information for the stored file CID

Includes the IPFS pinning state and the Filecoin deal state.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** | CID for the NFT | [required] |

### Return type

[**crate::models::GetResponse**](GetResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store

> crate::models::UploadResponse store(body)
Store a file

Store a file with nft.storage.  - Submit a HTTP `POST` request passing the file data in the request body. - To store multiple files in a directory, submit a `multipart/form-data` HTTP `POST` request.  Use the `Content-Disposition` header for each part to specify a filename. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **std::path::PathBuf** |  | [required] |

### Return type

[**crate::models::UploadResponse**](UploadResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: image/png, application/octet-stream, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

