# ProjectInfoResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**date_created** | Option<[**serde_json::Value**](.md)> | The UTC datetime the project was created in ISO8601 format. | 
**is_public** | Option<[**serde_json::Value**](.md)> | Whether the project is public. | 
**latest_circuit** | Option<[**crate::models::ProjectLatestCircuitResponse**](ProjectLatestCircuitResponse.md)> |  | [optional]
**name** | Option<[**serde_json::Value**](.md)> | The name of the project. | 
**num_proofs** | Option<[**serde_json::Value**](.md)> | The number of proofs created for this project. | 
**project_id** | Option<[**serde_json::Value**](.md)> | A unique identifier generated for the project. UUID4 format. | 
**tags** | Option<[**serde_json::Value**](.md)> | Tags for the project. | 
**team** | Option<[**serde_json::Value**](.md)> | The name of the team that owns this project. | 
**team_avatar_url** | Option<[**serde_json::Value**](.md)> | URL for the avatar image of the team that owns this project. | 
**team_slug** | Option<[**serde_json::Value**](.md)> | The slug of the team that owns this project. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


