# ApiKeyResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_key** | Option<[**serde_json::Value**](.md)> | The API key. Will be `null` unless the key was created during the request. Keys are not stored in plaintext and can not be recovered after creation time. | 
**date_created** | Option<[**serde_json::Value**](.md)> | The date that the API key was created. | 
**date_expires** | Option<[**serde_json::Value**](.md)> | The date that the API key will automatically expire. | 
**date_last_used** | Option<[**serde_json::Value**](.md)> | The last time that the API key was used to authenticate with the API. | 
**id** | Option<[**serde_json::Value**](.md)> | The database ID for the API key. Used when deleting keys. | 
**name** | Option<[**serde_json::Value**](.md)> | The human-readable name for the API key used for managing keys. | 
**prefix** | Option<[**serde_json::Value**](.md)> | The non-secret key prefix. | 
**suffix** | Option<[**serde_json::Value**](.md)> | The non-secret key suffix. Helpful for identifying keys if a name wasn't specified at creation time. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


