# ProjectInfoResponseLatestCircuit

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**circuit_id** | Option<[**serde_json::Value**](.md)> | A unique identifier generated for the circuit. UUID4 format. | 
**circuit_type** | [**models::CircuitType**](CircuitType.md) | The development framework used to write the circuit. This is specified during creation in the included sindri.json file. | 
**date_created** | Option<[**serde_json::Value**](.md)> | The UTC datetime the circuit was uploaded in ISO8601 format. | 
**proving_scheme** | Option<[**serde_json::Value**](.md)> | The proving scheme for this circuit. This is specified during creation in the included sindri.json file. | 
**status** | [**models::JobStatus**](JobStatus.md) | The status of the circuit job. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


