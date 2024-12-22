# ProjectInfoResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**date_created** | **String** | The UTC datetime the project was created in ISO8601 format. | 
**is_public** | **bool** | Whether the project is public. | 
**latest_circuit** | Option<[**models::ProjectLatestCircuitResponse**](ProjectLatestCircuitResponse.md)> |  | [optional]
**name** | **String** | The name of the project. | 
**num_proofs** | Option<**i32**> | The number of proofs created for this project. | 
**project_id** | **String** | A unique identifier generated for the project. UUID4 format. | 
**tags** | **Vec<String>** | Tags for the project. | 
**team** | **String** | The name of the team that owns this project. | 
**team_avatar_url** | **String** | URL for the avatar image of the team that owns this project. | 
**team_slug** | **String** | The slug of the team that owns this project. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


