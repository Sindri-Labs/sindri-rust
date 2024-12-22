# CircuitProveInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**meta** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | An arbitrary mapping of metadata keys to string values. This can be used to track additional information about the proof such as an ID from an external system. | [optional][default to {}]
**proof_input** | Option<[**serde_json::Value**](.md)> | An object mapping proof input variable names to their values. Can be a raw JSON object or a string serialized as JSON or TOML. | 
**perform_verify** | Option<[**serde_json::Value**](.md)> | A boolean indicating whether to perform an internal verification check during the proof creation. | [optional][default to false]
**prover_implementation** | Option<[**serde_json::Value**](.md)> | Internal prover implementation setting. | [optional][default to ]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


