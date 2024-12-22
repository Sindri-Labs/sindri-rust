# NoirCircuitInfoResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**circuit_id** | Option<[**serde_json::Value**](.md)> | A unique identifier generated for the circuit. UUID4 format. | 
**circuit_name** | Option<[**serde_json::Value**](.md)> |  | 
**project_name** | Option<[**serde_json::Value**](.md)> | The name of the project. This can be used in place of circuit_id for proving. This is specified during creation in the included sindri.json file. If the project is renamed, this will be the new name of the project, not the original name that was included in the sindri.json file. | 
**circuit_type** | Option<[**serde_json::Value**](serde_json::Value.md)> | The development framework used to write the circuit. This is specified during creation in the included sindri.json file. | 
**date_created** | Option<[**serde_json::Value**](.md)> | The UTC datetime the circuit was uploaded in ISO8601 format. | 
**meta** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Metadata keys and values for the circuit that were specified at creation time. | 
**num_proofs** | Option<[**serde_json::Value**](.md)> | The number of proofs submitted for this circuit. | 
**proving_scheme** | Option<[**serde_json::Value**](.md)> | The proving scheme for this circuit. This is specified during creation in the included sindri.json file. | 
**public** | Option<[**serde_json::Value**](.md)> | Whether the circuit is public. Public circuits can be used by any user. | 
**status** | [**crate::models::JobStatus**](JobStatus.md) |  | 
**finished_processing** | Option<[**serde_json::Value**](.md)> | The job is finished processing and waiting/polling can be terminated. | 
**tags** | Option<[**serde_json::Value**](.md)> | Tags for the circuit. | 
**team** | Option<[**serde_json::Value**](.md)> | The name of the team that owns this circuit. | 
**team_avatar_url** | Option<[**serde_json::Value**](.md)> | URL for the avatar image of the team that owns this circuit. | 
**team_slug** | Option<[**serde_json::Value**](.md)> | The slug of the team that owns this circuit. | 
**compute_time** | Option<[**serde_json::Value**](.md)> | Total compute time in ISO8601 format. | 
**compute_time_sec** | Option<[**serde_json::Value**](.md)> | Total compute time in seconds. | 
**compute_times** | Option<[**serde_json::Value**](.md)> | Detailed compute times for the circuit compilation. | 
**file_size** | Option<[**serde_json::Value**](.md)> | Total size of stored file(s) in bytes. | 
**queue_time** | Option<[**serde_json::Value**](.md)> | Queue time in ISO8601 format. | 
**queue_time_sec** | Option<[**serde_json::Value**](.md)> | Queue time in seconds. | 
**uploaded_file_name** | Option<[**serde_json::Value**](.md)> | The name of the uploaded circuit file. Note: the CLI and SDKs create a generic name when a directory is specified for upload. | 
**has_smart_contract_verifier** | Option<[**serde_json::Value**](.md)> | Boolean indicating whether this circuit has a smart contract verifier available. | 
**has_verification_key** | Option<[**serde_json::Value**](.md)> | Boolean indicating whether this circuit has a verification key available. | 
**verification_key** | Option<[**serde_json::Value**](.md)> | The verification key of this circuit. | 
**warnings** | Option<[**serde_json::Value**](.md)> | A list of runtime warnings with UTC timestamps. | 
**error** | Option<[**serde_json::Value**](.md)> | The error message for a failed circuit upload. | 
**acir_opcodes** | Option<[**serde_json::Value**](.md)> | The number of opcodes in the intermediate representation. | 
**circuit_size** | Option<[**serde_json::Value**](.md)> | The number of constraints with an instantiated proving backend in the circuit. | 
**curve** | Option<[**serde_json::Value**](.md)> | The elliptic curve over which the proving protocol takes place. | 
**nargo_package_name** | Option<[**serde_json::Value**](.md)> | The name of the circuit project specified in the included Nargo.toml file. | 
**noir_version** | Option<[**serde_json::Value**](.md)> | The Noir frontend version tag. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


