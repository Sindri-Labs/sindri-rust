# ProofHistogramResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bin_size** | **i32** | The bin size in seconds. | 
**data** | [**Vec<serde_json::Value>**](serde_json::Value.md) | A list of dictionaries in the format:</br>                 ```                  {                      'bin': '2021-01-01T00:00:00Z',                      'ready': 2,                      'failed': 1,                      'in_progress': 3,                      'queued': 4,                  }                  ```</br>                  where 'bin' is an ISO8601 timestamp indicating the start                  of the bin, and proof status keys (e.g. 'ready') indicating                  the number of proofs with that status in the bin. | 
**end_time** | **String** | The end of the histogram in timezone-aware ISO8601 format. | 
**start_time** | **String** | The start of the histogram in timezone-aware ISO8601 format. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


