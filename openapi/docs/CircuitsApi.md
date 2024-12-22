# \CircuitsApi

All URIs are relative to *https://stage.sindri.app*

Method | HTTP request | Description
------------- | ------------- | -------------
[**circuit_create**](CircuitsApi.md#circuit_create) | **POST** /api/v1/circuit/create | Create Circuit
[**circuit_delete**](CircuitsApi.md#circuit_delete) | **DELETE** /api/v1/circuit/{circuit_id}/delete | Delete Circuit
[**circuit_detail**](CircuitsApi.md#circuit_detail) | **GET** /api/v1/circuit/{circuit_id}/detail | Circuit Detail
[**circuit_list**](CircuitsApi.md#circuit_list) | **GET** /api/v1/circuit/list | Circuit List
[**circuit_proofs**](CircuitsApi.md#circuit_proofs) | **GET** /api/v1/circuit/{circuit_id}/proofs | Circuit Proofs
[**proof_create**](CircuitsApi.md#proof_create) | **POST** /api/v1/circuit/{circuit_id}/prove | Create Proof for Circuit



## circuit_create

> serde_json::Value circuit_create(files, tags)
Create Circuit

Create a circuit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**files** | Option<[**serde_json::Value**](serde_json::Value.md)> |  | [required] |
**tags** | Option<[**serde_json::Value**](serde_json::Value.md)> | Tags for a circuit. |  |[default to ["latest"]]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuit_delete

> crate::models::ActionResponse circuit_delete(circuit_id)
Delete Circuit

Delete a circuit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**circuit_id** | [**serde_json::Value**](.md) | The circuit identifer of the circuit. This can take one of the following forms:  1. `<CIRCUIT_ID>` - The unique UUID4 ID for an exact version of a compiled circuit. 2. `<CIRCUIT_NAME>` - The name of a circuit owned by the authenticated team. This will default to     the most recent version of the circuit tagged as `latest`. 3. `<CIRCUIT_NAME>:<TAG>` - The name of a circuit owned by the authenticated team and an explicit     tag. This corresponds to the most recent compilation of the circuit with the specified tag. 4. `<TEAM_NAME>/<CIRCUIT_NAME>` - The name of a circuit owned by the specified team.  This will     default to the most recent version of the circuit tagged as `latest`. 5. `<TEAM_NAME>/<CIRCUIT_NAME>:<TAG>` - The name of a circuit owned by a specified team and an     explicit tag. This corresponds to the most recent compilation of the team's circuit with the     specified tag. | [required] |

### Return type

[**crate::models::ActionResponse**](ActionResponse.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuit_detail

> serde_json::Value circuit_detail(circuit_id, include_verification_key)
Circuit Detail

Get info for an existing circuit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**circuit_id** | [**serde_json::Value**](.md) | The circuit identifer of the circuit. This can take one of the following forms:  1. `<CIRCUIT_ID>` - The unique UUID4 ID for an exact version of a compiled circuit. 2. `<CIRCUIT_NAME>` - The name of a circuit owned by the authenticated team. This will default to     the most recent version of the circuit tagged as `latest`. 3. `<CIRCUIT_NAME>:<TAG>` - The name of a circuit owned by the authenticated team and an explicit     tag. This corresponds to the most recent compilation of the circuit with the specified tag. 4. `<TEAM_NAME>/<CIRCUIT_NAME>` - The name of a circuit owned by the specified team.  This will     default to the most recent version of the circuit tagged as `latest`. 5. `<TEAM_NAME>/<CIRCUIT_NAME>:<TAG>` - The name of a circuit owned by a specified team and an     explicit tag. This corresponds to the most recent compilation of the team's circuit with the     specified tag. | [required] |
**include_verification_key** | Option<[**serde_json::Value**](.md)> | Indicates whether to include the verification key in the response. |  |[default to true]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuit_list

> serde_json::Value circuit_list()
Circuit List

List all circuits owned by team.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuit_proofs

> serde_json::Value circuit_proofs(circuit_id)
Circuit Proofs

List all proofs for a circuit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**circuit_id** | [**serde_json::Value**](.md) | The circuit identifer of the circuit. This can take one of the following forms:  1. `<CIRCUIT_ID>` - The unique UUID4 ID for an exact version of a compiled circuit. 2. `<CIRCUIT_NAME>` - The name of a circuit owned by the authenticated team. This will default to     the most recent version of the circuit tagged as `latest`. 3. `<CIRCUIT_NAME>:<TAG>` - The name of a circuit owned by the authenticated team and an explicit     tag. This corresponds to the most recent compilation of the circuit with the specified tag. 4. `<TEAM_NAME>/<CIRCUIT_NAME>` - The name of a circuit owned by the specified team.  This will     default to the most recent version of the circuit tagged as `latest`. 5. `<TEAM_NAME>/<CIRCUIT_NAME>:<TAG>` - The name of a circuit owned by a specified team and an     explicit tag. This corresponds to the most recent compilation of the team's circuit with the     specified tag. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## proof_create

> crate::models::ProofInfoResponse proof_create(circuit_id, circuit_prove_input)
Create Proof for Circuit

Prove a circuit with specific inputs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**circuit_id** | [**serde_json::Value**](.md) | The circuit identifer of the circuit. This can take one of the following forms:  1. `<CIRCUIT_ID>` - The unique UUID4 ID for an exact version of a compiled circuit. 2. `<CIRCUIT_NAME>` - The name of a circuit owned by the authenticated team. This will default to     the most recent version of the circuit tagged as `latest`. 3. `<CIRCUIT_NAME>:<TAG>` - The name of a circuit owned by the authenticated team and an explicit     tag. This corresponds to the most recent compilation of the circuit with the specified tag. 4. `<TEAM_NAME>/<CIRCUIT_NAME>` - The name of a circuit owned by the specified team.  This will     default to the most recent version of the circuit tagged as `latest`. 5. `<TEAM_NAME>/<CIRCUIT_NAME>:<TAG>` - The name of a circuit owned by a specified team and an     explicit tag. This corresponds to the most recent compilation of the team's circuit with the     specified tag. | [required] |
**circuit_prove_input** | [**CircuitProveInput**](CircuitProveInput.md) |  | [required] |

### Return type

[**crate::models::ProofInfoResponse**](ProofInfoResponse.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

