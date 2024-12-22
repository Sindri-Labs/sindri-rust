# \TokenApi

All URIs are relative to *https://stage.sindri.app*

Method | HTTP request | Description
------------- | ------------- | -------------
[**jwt_token_generate**](TokenApi.md#jwt_token_generate) | **POST** /api/token/pair | Generate JWT Token Pair
[**jwt_token_refresh**](TokenApi.md#jwt_token_refresh) | **POST** /api/token/refresh | Refresh Token
[**jwt_token_verify**](TokenApi.md#jwt_token_verify) | **POST** /api/token/verify | Verify Token



## jwt_token_generate

> models::TokenObtainPairOutputSchema jwt_token_generate(token_obtain_pair_input_schema)
Generate JWT Token Pair

Override the ninja_jwt default `obtain_token` method in order to add email verification check before generating a token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_obtain_pair_input_schema** | [**TokenObtainPairInputSchema**](TokenObtainPairInputSchema.md) |  | [required] |

### Return type

[**models::TokenObtainPairOutputSchema**](TokenObtainPairOutputSchema.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jwt_token_refresh

> models::TokenRefreshOutputSchema jwt_token_refresh(token_refresh_input_schema)
Refresh Token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_refresh_input_schema** | [**TokenRefreshInputSchema**](TokenRefreshInputSchema.md) |  | [required] |

### Return type

[**models::TokenRefreshOutputSchema**](TokenRefreshOutputSchema.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jwt_token_verify

> serde_json::Value jwt_token_verify(token_verify_input_schema)
Verify Token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_verify_input_schema** | [**TokenVerifyInputSchema**](TokenVerifyInputSchema.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

