# ProjectLatestCircuitResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**circuit_id** | Option<[**serde_json::Value**](.md)> | A unique identifier generated for the circuit. UUID4 format. | 
**circuit_type** | [**crate::models::CircuitType**](CircuitType.md) |  | 
**date_created** | Option<[**serde_json::Value**](.md)> | The UTC datetime the circuit was uploaded in ISO8601 format. | 
**proving_scheme** | Option<[**serde_json::Value**](.md)> | The proving scheme for this circuit. This is specified during creation in the included sindri.json file. | 
**status** | [**crate::models::JobStatus**](JobStatus.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


