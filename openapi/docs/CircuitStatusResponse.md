# CircuitStatusResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**circuit_id** | Option<[**serde_json::Value**](.md)> | A unique identifier generated for the circuit. UUID4 format. | 
**status** | [**crate::models::JobStatus**](JobStatus.md) |  | 
**finished_processing** | Option<[**serde_json::Value**](.md)> | The job is finished processing and waiting/polling can be terminated. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


