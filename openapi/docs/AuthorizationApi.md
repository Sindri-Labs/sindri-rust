# \AuthorizationApi

All URIs are relative to *https://sindri.app*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apikey_delete**](AuthorizationApi.md#apikey_delete) | **DELETE** /api/v1/apikey/{apikey_id}/delete | API Key Delete
[**apikey_generate**](AuthorizationApi.md#apikey_generate) | **POST** /api/apikey/generate | Generate API Key
[**apikey_list**](AuthorizationApi.md#apikey_list) | **GET** /api/v1/apikey/list | API Key List



## apikey_delete

> models::ActionResponse apikey_delete(apikey_id)
API Key Delete

Delete a specific API key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey_id** | **String** | The UUID4 identifier associated with this API Key. | [required] |

### Return type

[**models::ActionResponse**](ActionResponse.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apikey_generate

> models::ApiKeyResponse apikey_generate(obtain_apikey_input)
Generate API Key

Generates a long-term API Key from your account's username and password.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**obtain_apikey_input** | [**ObtainApikeyInput**](ObtainApikeyInput.md) |  | [required] |

### Return type

[**models::ApiKeyResponse**](APIKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apikey_list

> Vec<models::ApiKeyResponse> apikey_list()
API Key List

List API keys for the requesting team.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ApiKeyResponse>**](APIKeyResponse.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

