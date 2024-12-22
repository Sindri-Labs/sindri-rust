# \ProofsApi

All URIs are relative to *https://sindri.app*

Method | HTTP request | Description
------------- | ------------- | -------------
[**proof_delete**](ProofsApi.md#proof_delete) | **DELETE** /api/v1/proof/{proof_id}/delete | Delete Proof
[**proof_detail**](ProofsApi.md#proof_detail) | **GET** /api/v1/proof/{proof_id}/detail | Proof Detail



## proof_delete

> models::ActionResponse proof_delete(proof_id)
Delete Proof

Delete a specific proof.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proof_id** | **String** | The UUID4 identifier associated with this proof. | [required] |

### Return type

[**models::ActionResponse**](ActionResponse.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## proof_detail

> models::ProofInfoResponse proof_detail(proof_id, include_proof, include_public, include_smart_contract_calldata, include_verification_key)
Proof Detail

Get info for a specific proof.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proof_id** | **String** | The UUID4 identifier associated with this proof. | [required] |
**include_proof** | Option<**bool**> | Indicates whether to include the proof in the response. |  |[default to true]
**include_public** | Option<**bool**> | Indicates whether to include public inputs in the response. |  |[default to true]
**include_smart_contract_calldata** | Option<**bool**> | Indicates whether to include the proof and public formatted as smart contract calldata in the response. |  |[default to true]
**include_verification_key** | Option<**bool**> | Indicates whether to include the circuit's verification key in the response. |  |[default to true]

### Return type

[**models::ProofInfoResponse**](ProofInfoResponse.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

