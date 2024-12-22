# \InternalApi

All URIs are relative to *https://stage.sindri.app*

Method | HTTP request | Description
------------- | ------------- | -------------
[**circuit_download**](InternalApi.md#circuit_download) | **GET** /api/v1/circuit/{circuit_id}/download | Circuit File Download
[**circuit_proofs_paginated**](InternalApi.md#circuit_proofs_paginated) | **GET** /api/v1/circuit/{circuit_id}/proofs/paginated | Circuit Proofs
[**circuit_smart_contract_verifier**](InternalApi.md#circuit_smart_contract_verifier) | **GET** /api/v1/circuit/{circuit_id}/smart_contract_verifier | Circuit Smart Contract Verifier
[**circuit_status**](InternalApi.md#circuit_status) | **GET** /api/v1/circuit/{circuit_id}/status | Circuit Status
[**password_change_with_jwt_auth**](InternalApi.md#password_change_with_jwt_auth) | **POST** /api/v1/password/change | Change Password
[**project_circuits**](InternalApi.md#project_circuits) | **GET** /api/v1/project/{project_id}/circuits | Project Circuits
[**project_circuits_paginated**](InternalApi.md#project_circuits_paginated) | **GET** /api/v1/project/{project_id}/circuits/paginated | Project Circuits
[**project_delete**](InternalApi.md#project_delete) | **DELETE** /api/v1/project/{project_id}/delete | Delete Project
[**project_detail**](InternalApi.md#project_detail) | **GET** /api/v1/project/{project_id}/detail | Project Detail
[**project_list**](InternalApi.md#project_list) | **POST** /api/v1/project/list | Project List
[**project_list_paginated**](InternalApi.md#project_list_paginated) | **POST** /api/v1/project/list/paginated | Project List
[**project_proofs**](InternalApi.md#project_proofs) | **GET** /api/v1/project/{project_id}/proofs | Project Proofs
[**project_proofs_paginated**](InternalApi.md#project_proofs_paginated) | **GET** /api/v1/project/{project_id}/proofs/paginated | Project Proofs
[**project_settings**](InternalApi.md#project_settings) | **POST** /api/v1/project/{project_name}/settings | Update Project Settings
[**proof_list**](InternalApi.md#proof_list) | **POST** /api/v1/proof/list | Proof List
[**proof_list_paginated**](InternalApi.md#proof_list_paginated) | **POST** /api/v1/proof/list/paginated | Proof List
[**proof_status**](InternalApi.md#proof_status) | **GET** /api/v1/proof/{proof_id}/status | Proof Status
[**sindri_manifest_schema**](InternalApi.md#sindri_manifest_schema) | **GET** /api/v1/sindri-manifest-schema.json | Sindri Manifest Schema
[**team_avatar_upload**](InternalApi.md#team_avatar_upload) | **POST** /api/v1/team/avatar/upload | Avatar Upload
[**team_detail**](InternalApi.md#team_detail) | **GET** /api/v1/team/{team_name}/detail | Team Detail
[**team_me**](InternalApi.md#team_me) | **GET** /api/v1/team/me | Team Me
[**user_me_with_jwt_auth**](InternalApi.md#user_me_with_jwt_auth) | **GET** /api/v1/user/me | User Me



## circuit_download

> circuit_download(circuit_id, path)
Circuit File Download

Obtain circuit file(s).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**circuit_id** | [**serde_json::Value**](.md) | The circuit identifer of the circuit. This can take one of the following forms:  1. `<CIRCUIT_ID>` - The unique UUID4 ID for an exact version of a compiled circuit. 2. `<CIRCUIT_NAME>` - The name of a circuit owned by the authenticated team. This will default to     the most recent version of the circuit tagged as `latest`. 3. `<CIRCUIT_NAME>:<TAG>` - The name of a circuit owned by the authenticated team and an explicit     tag. This corresponds to the most recent compilation of the circuit with the specified tag. 4. `<TEAM_NAME>/<CIRCUIT_NAME>` - The name of a circuit owned by the specified team.  This will     default to the most recent version of the circuit tagged as `latest`. 5. `<TEAM_NAME>/<CIRCUIT_NAME>:<TAG>` - The name of a circuit owned by a specified team and an     explicit tag. This corresponds to the most recent compilation of the team's circuit with the     specified tag. | [required] |
**path** | Option<[**serde_json::Value**](.md)> | The optional file path within the circuit package to download. |  |

### Return type

 (empty response body)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuit_proofs_paginated

> crate::models::PagedProofInfoResponse circuit_proofs_paginated(circuit_id, limit, offset)
Circuit Proofs

List all proofs for a circuit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**circuit_id** | [**serde_json::Value**](.md) | The circuit identifer of the circuit. This can take one of the following forms:  1. `<CIRCUIT_ID>` - The unique UUID4 ID for an exact version of a compiled circuit. 2. `<CIRCUIT_NAME>` - The name of a circuit owned by the authenticated team. This will default to     the most recent version of the circuit tagged as `latest`. 3. `<CIRCUIT_NAME>:<TAG>` - The name of a circuit owned by the authenticated team and an explicit     tag. This corresponds to the most recent compilation of the circuit with the specified tag. 4. `<TEAM_NAME>/<CIRCUIT_NAME>` - The name of a circuit owned by the specified team.  This will     default to the most recent version of the circuit tagged as `latest`. 5. `<TEAM_NAME>/<CIRCUIT_NAME>:<TAG>` - The name of a circuit owned by a specified team and an     explicit tag. This corresponds to the most recent compilation of the team's circuit with the     specified tag. | [required] |
**limit** | Option<[**serde_json::Value**](.md)> | The number of proofs to return. |  |[default to 100]
**offset** | Option<[**serde_json::Value**](.md)> | The number of proofs to skip. |  |[default to 0]

### Return type

[**crate::models::PagedProofInfoResponse**](PagedProofInfoResponse.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuit_smart_contract_verifier

> crate::models::SmartContractVerifierResponse circuit_smart_contract_verifier(circuit_id)
Circuit Smart Contract Verifier

Get smart contract verifier for existing circuit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**circuit_id** | [**serde_json::Value**](.md) | The circuit identifer of the circuit. This can take one of the following forms:  1. `<CIRCUIT_ID>` - The unique UUID4 ID for an exact version of a compiled circuit. 2. `<CIRCUIT_NAME>` - The name of a circuit owned by the authenticated team. This will default to     the most recent version of the circuit tagged as `latest`. 3. `<CIRCUIT_NAME>:<TAG>` - The name of a circuit owned by the authenticated team and an explicit     tag. This corresponds to the most recent compilation of the circuit with the specified tag. 4. `<TEAM_NAME>/<CIRCUIT_NAME>` - The name of a circuit owned by the specified team.  This will     default to the most recent version of the circuit tagged as `latest`. 5. `<TEAM_NAME>/<CIRCUIT_NAME>:<TAG>` - The name of a circuit owned by a specified team and an     explicit tag. This corresponds to the most recent compilation of the team's circuit with the     specified tag. | [required] |

### Return type

[**crate::models::SmartContractVerifierResponse**](SmartContractVerifierResponse.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuit_status

> crate::models::CircuitStatusResponse circuit_status(circuit_id)
Circuit Status

Get status for a specific circuit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**circuit_id** | [**serde_json::Value**](.md) | The UUID4 identifier associated with this circuit. | [required] |

### Return type

[**crate::models::CircuitStatusResponse**](CircuitStatusResponse.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## password_change_with_jwt_auth

> crate::models::ActionResponse password_change_with_jwt_auth(password_change_input)
Change Password

Change user password. Requires JWT authentication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**password_change_input** | [**PasswordChangeInput**](PasswordChangeInput.md) |  | [required] |

### Return type

[**crate::models::ActionResponse**](ActionResponse.md)

### Authorization

[JWTAuth](../README.md#JWTAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_circuits

> serde_json::Value project_circuits(project_id)
Project Circuits

List all circuits for a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | [**serde_json::Value**](.md) | The project identifer of the project. This can take one of the following forms:  1. `<PROJECT_ID>` - The unique UUID4 ID for a project. 2. `<TEAM_NAME>/<PROJECT_NAME>` - The name of a project owned by the specified team. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_circuits_paginated

> crate::models::PagedCircuitInfoResponse project_circuits_paginated(project_id, limit, offset)
Project Circuits

List all circuits for a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | [**serde_json::Value**](.md) | The project identifer of the project. This can take one of the following forms:  1. `<PROJECT_ID>` - The unique UUID4 ID for a project. 2. `<TEAM_NAME>/<PROJECT_NAME>` - The name of a project owned by the specified team. | [required] |
**limit** | Option<[**serde_json::Value**](.md)> | The number of circuits to return. |  |[default to 100]
**offset** | Option<[**serde_json::Value**](.md)> | The number of circuits to skip. |  |[default to 0]

### Return type

[**crate::models::PagedCircuitInfoResponse**](PagedCircuitInfoResponse.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_delete

> crate::models::ActionResponse project_delete(project_id)
Delete Project

Delete a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | [**serde_json::Value**](.md) | The project identifer of the project. This can take one of the following forms:  1. `<PROJECT_ID>` - The unique UUID4 ID for a project. 2. `<TEAM_NAME>/<PROJECT_NAME>` - The name of a project owned by the specified team. | [required] |

### Return type

[**crate::models::ActionResponse**](ActionResponse.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_detail

> crate::models::ProjectInfoResponse project_detail(project_id)
Project Detail

Get info for a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | [**serde_json::Value**](.md) | The project identifer of the project. This can take one of the following forms:  1. `<PROJECT_ID>` - The unique UUID4 ID for a project. 2. `<TEAM_NAME>/<PROJECT_NAME>` - The name of a project owned by the specified team. | [required] |

### Return type

[**crate::models::ProjectInfoResponse**](ProjectInfoResponse.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_list

> serde_json::Value project_list(project_list_input)
Project List

List all projects meeting filter criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_list_input** | [**ProjectListInput**](ProjectListInput.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_list_paginated

> crate::models::PagedProjectInfoResponse project_list_paginated(project_list_input, limit, offset)
Project List

List all projects meeting filter criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_list_input** | [**ProjectListInput**](ProjectListInput.md) |  | [required] |
**limit** | Option<[**serde_json::Value**](.md)> | The number of projects to return. |  |[default to 100]
**offset** | Option<[**serde_json::Value**](.md)> | The number of projects to skip. |  |[default to 0]

### Return type

[**crate::models::PagedProjectInfoResponse**](PagedProjectInfoResponse.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_proofs

> serde_json::Value project_proofs(project_id)
Project Proofs

Get all proofs for a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | [**serde_json::Value**](.md) | The project identifer of the project. This can take one of the following forms:  1. `<PROJECT_ID>` - The unique UUID4 ID for a project. 2. `<TEAM_NAME>/<PROJECT_NAME>` - The name of a project owned by the specified team. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_proofs_paginated

> crate::models::PagedProofInfoResponse project_proofs_paginated(project_id, limit, offset)
Project Proofs

Get all proofs for a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | [**serde_json::Value**](.md) | The project identifer of the project. This can take one of the following forms:  1. `<PROJECT_ID>` - The unique UUID4 ID for a project. 2. `<TEAM_NAME>/<PROJECT_NAME>` - The name of a project owned by the specified team. | [required] |
**limit** | Option<[**serde_json::Value**](.md)> | The number of proofs to return. |  |[default to 100]
**offset** | Option<[**serde_json::Value**](.md)> | The number of proofs to skip. |  |[default to 0]

### Return type

[**crate::models::PagedProofInfoResponse**](PagedProofInfoResponse.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_settings

> crate::models::ProjectInfoResponse project_settings(project_name, project_settings_input)
Update Project Settings

Update project settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | [**serde_json::Value**](.md) | The name of a project associated with the team. | [required] |
**project_settings_input** | [**ProjectSettingsInput**](ProjectSettingsInput.md) |  | [required] |

### Return type

[**crate::models::ProjectInfoResponse**](ProjectInfoResponse.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## proof_list

> serde_json::Value proof_list(proof_list_input)
Proof List

List proofs for the requesting team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proof_list_input** | [**ProofListInput**](ProofListInput.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## proof_list_paginated

> crate::models::PagedProofInfoResponse proof_list_paginated(proof_list_input, limit, offset)
Proof List

List proofs for the requesting team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proof_list_input** | [**ProofListInput**](ProofListInput.md) |  | [required] |
**limit** | Option<[**serde_json::Value**](.md)> | The number of proofs to return. |  |[default to 100]
**offset** | Option<[**serde_json::Value**](.md)> | The number of proofs to skip. |  |[default to 0]

### Return type

[**crate::models::PagedProofInfoResponse**](PagedProofInfoResponse.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## proof_status

> crate::models::ProofStatusResponse proof_status(proof_id)
Proof Status

Get status for a specific proof.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proof_id** | [**serde_json::Value**](.md) | The UUID4 identifier associated with this proof. | [required] |

### Return type

[**crate::models::ProofStatusResponse**](ProofStatusResponse.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sindri_manifest_schema

> serde_json::Value sindri_manifest_schema()
Sindri Manifest Schema

Return Sindri manifest schema as JSON.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_avatar_upload

> crate::models::TeamMeResponse team_avatar_upload(files)
Avatar Upload

Upload avatar for the team

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**files** | Option<[**serde_json::Value**](serde_json::Value.md)> |  | [required] |

### Return type

[**crate::models::TeamMeResponse**](TeamMeResponse.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_detail

> crate::models::TeamDetail team_detail(team_name)
Team Detail

Return details for the specified team

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_name** | [**serde_json::Value**](.md) |  | [required] |

### Return type

[**crate::models::TeamDetail**](TeamDetail.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_me

> crate::models::TeamMeResponse team_me()
Team Me

Obtain team details for the currently authenticated team

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::TeamMeResponse**](TeamMeResponse.md)

### Authorization

[SindriAPIKeyBearerAuth](../README.md#SindriAPIKeyBearerAuth), [SindriJWTBearerAuth](../README.md#SindriJWTBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_me_with_jwt_auth

> crate::models::UserMeResponse user_me_with_jwt_auth()
User Me

Obtain user details. Requires JWT authentication.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UserMeResponse**](UserMeResponse.md)

### Authorization

[JWTAuth](../README.md#JWTAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

