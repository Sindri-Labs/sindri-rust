# GnarkCircuitInfoResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**circuit_id** | **String** | A unique identifier generated for the circuit. UUID4 format. | 
**circuit_name** | **String** |  | 
**project_name** | **String** | The name of the project. This can be used in place of circuit_id for proving. This is specified during creation in the included sindri.json file. If the project is renamed, this will be the new name of the project, not the original name that was included in the sindri.json file. | 
**circuit_type** | **String** | The development framework used to write the circuit. This is specified during creation in the included sindri.json file. | 
**date_created** | **String** | The UTC datetime the circuit was uploaded in ISO8601 format. | 
**meta** | **std::collections::HashMap<String, String>** | Metadata keys and values for the circuit that were specified at creation time. | 
**num_proofs** | Option<**i32**> | The number of proofs submitted for this circuit. | 
**proving_scheme** | **String** | The proving scheme for this circuit. This is specified during creation in the included sindri.json file. | 
**public** | **bool** | Whether the circuit is public. Public circuits can be used by any user. | 
**status** | [**models::JobStatus**](JobStatus.md) | The status of the circuit job. | 
**finished_processing** | **bool** | The job is finished processing and waiting/polling can be terminated. | 
**tags** | **Vec<String>** | Tags for the circuit. | 
**team** | **String** | The name of the team that owns this circuit. | 
**team_avatar_url** | **String** | URL for the avatar image of the team that owns this circuit. | 
**team_slug** | **String** | The slug of the team that owns this circuit. | 
**compute_time** | Option<**String**> | Total compute time in ISO8601 format. | 
**compute_time_sec** | Option<**f64**> | Total compute time in seconds. | 
**compute_times** | Option<[**serde_json::Value**](.md)> | Detailed compute times for the circuit compilation. | 
**file_size** | Option<**i32**> | Total size of stored file(s) in bytes. | 
**queue_time** | Option<**String**> | Queue time in ISO8601 format. | 
**queue_time_sec** | Option<**f64**> | Queue time in seconds. | 
**uploaded_file_name** | **String** | The name of the uploaded circuit file. Note: the CLI and SDKs create a generic name when a directory is specified for upload. | 
**has_smart_contract_verifier** | **bool** | Boolean indicating whether this circuit has a smart contract verifier available. | 
**has_verification_key** | **bool** | Boolean indicating whether this circuit has a verification key available. | 
**verification_key** | Option<[**serde_json::Value**](.md)> | The verification key of this circuit. | 
**warnings** | Option<**Vec<String>**> | A list of runtime warnings with UTC timestamps. | 
**error** | Option<**String**> | The error message for a failed circuit upload. | 
**curve** | **String** | The elliptic curve over which the proving protocol takes place. | 
**gnark_version** | **String** | The Gnark frontend version tag. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


