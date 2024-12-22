# \CircuitsApi

All URIs are relative to *https://sindri.app*

Method | HTTP request | Description
------------- | ------------- | -------------
[**circuit_create**](CircuitsApi.md#circuit_create) | **POST** /api/v1/circuit/create | Create Circuit
[**circuit_delete**](CircuitsApi.md#circuit_delete) | **DELETE** /api/v1/circuit/{circuit_id}/delete | Delete Circuit
[**circuit_detail**](CircuitsApi.md#circuit_detail) | **GET** /api/v1/circuit/{circuit_id}/detail | Circuit Detail
[**circuit_list**](CircuitsApi.md#circuit_list) | **GET** /api/v1/circuit/list | Circuit List
[**circuit_proofs**](CircuitsApi.md#circuit_proofs) | **GET** /api/v1/circuit/{circuit_id}/proofs | Circuit Proofs
[**proof_create**](CircuitsApi.md#proof_create) | **POST** /api/v1/circuit/{circuit_id}/prove | Create Proof for Circuit



## circuit_create

> models::CircuitInfoResponse circuit_create(files, meta, tags)
Create Circuit

Create a circuit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**files** | [**Vec<std::path::PathBuf>**](std::path::PathBuf.md) |  | [required] |
**meta** | Option<[**std::collections::HashMap<String, String>**](std::collections::HashMap.md)> | An arbitrary mapping of metadata keys to string values. This can be used to track additional information about the circuit such as an ID from an external system. |  |[default to {}]
**tags** | Option<[**Vec<String>**](String.md)> | Tags for a circuit. |  |

### Return type

[**models::CircuitInfoResponse**](CircuitInfoResponse.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuit_delete

> models::ActionResponse circuit_delete(circuit_id)
Delete Circuit

Delete a circuit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**circuit_id** | **String** | The circuit identifer of the circuit. This can take one of the following forms:  1. `<CIRCUIT_ID>` - The unique UUID4 ID for an exact version of a compiled circuit. 2. `<CIRCUIT_NAME>` - The name of a circuit owned by the authenticated team. This will default to     the most recent version of the circuit tagged as `latest`. 3. `<CIRCUIT_NAME>:<TAG>` - The name of a circuit owned by the authenticated team and an explicit     tag. This corresponds to the most recent compilation of the circuit with the specified tag. 4. `<TEAM_NAME>/<CIRCUIT_NAME>` - The name of a circuit owned by the specified team.  This will     default to the most recent version of the circuit tagged as `latest`. 5. `<TEAM_NAME>/<CIRCUIT_NAME>:<TAG>` - The name of a circuit owned by a specified team and an     explicit tag. This corresponds to the most recent compilation of the team's circuit with the     specified tag. | [required] |

### Return type

[**models::ActionResponse**](ActionResponse.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuit_detail

> models::CircuitInfoResponse circuit_detail(circuit_id, include_verification_key)
Circuit Detail

Get info for an existing circuit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**circuit_id** | **String** | The circuit identifer of the circuit. This can take one of the following forms:  1. `<CIRCUIT_ID>` - The unique UUID4 ID for an exact version of a compiled circuit. 2. `<CIRCUIT_NAME>` - The name of a circuit owned by the authenticated team. This will default to     the most recent version of the circuit tagged as `latest`. 3. `<CIRCUIT_NAME>:<TAG>` - The name of a circuit owned by the authenticated team and an explicit     tag. This corresponds to the most recent compilation of the circuit with the specified tag. 4. `<TEAM_NAME>/<CIRCUIT_NAME>` - The name of a circuit owned by the specified team.  This will     default to the most recent version of the circuit tagged as `latest`. 5. `<TEAM_NAME>/<CIRCUIT_NAME>:<TAG>` - The name of a circuit owned by a specified team and an     explicit tag. This corresponds to the most recent compilation of the team's circuit with the     specified tag. | [required] |
**include_verification_key** | Option<**bool**> | Indicates whether to include the verification key in the response. |  |[default to true]

### Return type

[**models::CircuitInfoResponse**](CircuitInfoResponse.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuit_list

> Vec<models::CircuitInfoResponse> circuit_list()
Circuit List

List all circuits owned by team.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::CircuitInfoResponse>**](CircuitInfoResponse.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuit_proofs

> Vec<models::ProofInfoResponse> circuit_proofs(circuit_id)
Circuit Proofs

List all proofs for a circuit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**circuit_id** | **String** | The circuit identifer of the circuit. This can take one of the following forms:  1. `<CIRCUIT_ID>` - The unique UUID4 ID for an exact version of a compiled circuit. 2. `<CIRCUIT_NAME>` - The name of a circuit owned by the authenticated team. This will default to     the most recent version of the circuit tagged as `latest`. 3. `<CIRCUIT_NAME>:<TAG>` - The name of a circuit owned by the authenticated team and an explicit     tag. This corresponds to the most recent compilation of the circuit with the specified tag. 4. `<TEAM_NAME>/<CIRCUIT_NAME>` - The name of a circuit owned by the specified team.  This will     default to the most recent version of the circuit tagged as `latest`. 5. `<TEAM_NAME>/<CIRCUIT_NAME>:<TAG>` - The name of a circuit owned by a specified team and an     explicit tag. This corresponds to the most recent compilation of the team's circuit with the     specified tag. | [required] |

### Return type

[**Vec<models::ProofInfoResponse>**](ProofInfoResponse.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## proof_create

> models::ProofInfoResponse proof_create(circuit_id, circuit_prove_input)
Create Proof for Circuit

Prove a circuit with specific inputs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**circuit_id** | **String** | The circuit identifer of the circuit. This can take one of the following forms:  1. `<CIRCUIT_ID>` - The unique UUID4 ID for an exact version of a compiled circuit. 2. `<CIRCUIT_NAME>` - The name of a circuit owned by the authenticated team. This will default to     the most recent version of the circuit tagged as `latest`. 3. `<CIRCUIT_NAME>:<TAG>` - The name of a circuit owned by the authenticated team and an explicit     tag. This corresponds to the most recent compilation of the circuit with the specified tag. 4. `<TEAM_NAME>/<CIRCUIT_NAME>` - The name of a circuit owned by the specified team.  This will     default to the most recent version of the circuit tagged as `latest`. 5. `<TEAM_NAME>/<CIRCUIT_NAME>:<TAG>` - The name of a circuit owned by a specified team and an     explicit tag. This corresponds to the most recent compilation of the team's circuit with the     specified tag. | [required] |
**circuit_prove_input** | [**CircuitProveInput**](CircuitProveInput.md) |  | [required] |

### Return type

[**models::ProofInfoResponse**](ProofInfoResponse.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

