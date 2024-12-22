# ProofInfoResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**proof_id** | Option<[**serde_json::Value**](.md)> | A unique identifier generated for the proof. UUID4 format. | 
**circuit_name** | Option<[**serde_json::Value**](.md)> |  | 
**project_name** | Option<[**serde_json::Value**](.md)> | The name of the project associated with this proof. | 
**circuit_id** | Option<[**serde_json::Value**](.md)> | The circuit_id of the circuit associated with this proof. UUID4 format. | 
**circuit_type** | [**crate::models::CircuitType**](CircuitType.md) |  | 
**date_created** | Option<[**serde_json::Value**](.md)> | The UTC datetime the circuit was uploaded in ISO8601 format. | 
**meta** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Metadata keys and values for the proof that were specified at creation time. | 
**perform_verify** | Option<[**serde_json::Value**](.md)> | A boolean indicating whether an internal verification check occurred during the proof creation. | 
**status** | [**crate::models::JobStatus**](JobStatus.md) |  | 
**finished_processing** | Option<[**serde_json::Value**](.md)> | The job is finished processing and waiting/polling can be terminated. | 
**verified** | Option<[**serde_json::Value**](.md)> | The status of proof verification. | 
**team** | Option<[**serde_json::Value**](.md)> | The name of the team that owns this proof. | 
**team_avatar_url** | Option<[**serde_json::Value**](.md)> | URL for the avatar image of the team that owns this proof. | 
**team_slug** | Option<[**serde_json::Value**](.md)> | The slug of the team that owns this proof. | 
**circuit_team** | Option<[**serde_json::Value**](.md)> | The name of the team that owns the circuit associated with this proof. | 
**circuit_team_avatar_url** | Option<[**serde_json::Value**](.md)> | URL for the avatar image of the team that owns the circuit associated with this proof. | 
**circuit_team_slug** | Option<[**serde_json::Value**](.md)> | The slug of the team that owns the circuit associated with this proof. | 
**compute_time** | Option<[**serde_json::Value**](.md)> | Total compute time in ISO8601 format. | [optional]
**compute_time_sec** | Option<[**serde_json::Value**](.md)> | Total compute time in seconds. | [optional]
**compute_times** | Option<[**serde_json::Value**](.md)> | Detailed compute times for the proof generation. | [optional]
**file_size** | Option<[**serde_json::Value**](.md)> | Total size of stored file(s) in bytes. | [optional]
**proof** | Option<[**serde_json::Value**](.md)> | The succinct argument(s) of knowledge. | [optional]
**public** | Option<[**serde_json::Value**](.md)> | The public outputs of the circuit. | [optional]
**queue_time** | Option<[**serde_json::Value**](.md)> | Queue time in ISO8601 format. | [optional]
**queue_time_sec** | Option<[**serde_json::Value**](.md)> | Queue time in seconds. | [optional]
**smart_contract_calldata** | Option<[**serde_json::Value**](.md)> | The proof and public formatted as calldata for the smart contract verifier. | [optional]
**has_smart_contract_calldata** | Option<[**serde_json::Value**](.md)> | Boolean indicating whether this proof has smart contract calldata available. | [optional][default to false]
**has_verification_key** | Option<[**serde_json::Value**](.md)> | Boolean indicating whether this proof's circuit has a verification key available. | [optional][default to false]
**verification_key** | Option<[**serde_json::Value**](.md)> | The verification key of this circuit. | [optional]
**warnings** | Option<[**serde_json::Value**](.md)> | A list of runtime warnings with UTC timestamps. | [optional]
**error** | Option<[**serde_json::Value**](.md)> | The error message for a failed proof. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


