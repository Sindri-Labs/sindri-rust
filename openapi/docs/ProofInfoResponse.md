# ProofInfoResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**proof_id** | **String** | A unique identifier generated for the proof. UUID4 format. | 
**circuit_name** | **String** |  | 
**project_name** | **String** | The name of the project associated with this proof. | 
**circuit_id** | **String** | The circuit_id of the circuit associated with this proof. UUID4 format. | 
**circuit_type** | [**models::CircuitType**](CircuitType.md) | The development framework used to write the circuit. This is specified during creation in the included sindri.json file. | 
**date_created** | **String** | The UTC datetime the circuit was uploaded in ISO8601 format. | 
**meta** | **std::collections::HashMap<String, String>** | Metadata keys and values for the proof that were specified at creation time. | 
**perform_verify** | **bool** | A boolean indicating whether an internal verification check occurred during the proof creation. | 
**status** | [**models::JobStatus**](JobStatus.md) | The status of the proof job. | 
**finished_processing** | **bool** | The job is finished processing and waiting/polling can be terminated. | 
**verified** | Option<**bool**> | The status of proof verification. | 
**team** | **String** | The name of the team that owns this proof. | 
**team_avatar_url** | **String** | URL for the avatar image of the team that owns this proof. | 
**team_slug** | **String** | The slug of the team that owns this proof. | 
**circuit_team** | **String** | The name of the team that owns the circuit associated with this proof. | 
**circuit_team_avatar_url** | **String** | URL for the avatar image of the team that owns the circuit associated with this proof. | 
**circuit_team_slug** | **String** | The slug of the team that owns the circuit associated with this proof. | 
**compute_time** | Option<**String**> | Total compute time in ISO8601 format. | [optional]
**compute_time_sec** | Option<**f64**> | Total compute time in seconds. | [optional]
**compute_times** | Option<[**serde_json::Value**](.md)> | Detailed compute times for the proof generation. | [optional]
**file_size** | Option<**i32**> | Total size of stored file(s) in bytes. | [optional]
**proof** | Option<[**serde_json::Value**](.md)> | The succinct argument(s) of knowledge. | [optional]
**public** | Option<[**serde_json::Value**](.md)> | The public outputs of the circuit. | [optional]
**queue_time** | Option<**String**> | Queue time in ISO8601 format. | [optional]
**queue_time_sec** | Option<**f64**> | Queue time in seconds. | [optional]
**smart_contract_calldata** | Option<**String**> | The proof and public formatted as calldata for the smart contract verifier. | [optional]
**has_smart_contract_calldata** | Option<**bool**> | Boolean indicating whether this proof has smart contract calldata available. | [optional][default to false]
**has_verification_key** | Option<**bool**> | Boolean indicating whether this proof's circuit has a verification key available. | [optional][default to false]
**verification_key** | Option<[**serde_json::Value**](.md)> | The verification key of this circuit. | [optional]
**warnings** | Option<**Vec<String>**> | A list of runtime warnings with UTC timestamps. | [optional]
**error** | Option<**String**> | The error message for a failed proof. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


