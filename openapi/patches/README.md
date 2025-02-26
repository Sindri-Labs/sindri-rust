# Manual Patches

Internal documentation. 

## Patch List

Most of these patches (marked with a `*`) are temporary solutions until the openapi-generator fully supports 3.1.0 specifications.
Removing `scripts/openapitools.json` and rerunning `./scripts/update-sdk.sh` will regenerate the client with the latest rust openapi-generator version, potentially removing the need for some of these patches.

| Filename       | Purpose                  | Patched Files         |
|----------------|--------------------------|-----------------------|
| `circuit_creation.patch` | *Creates custom multipart form data | `src/apis/circuits_api.rs` |
| `define_any_ltgt.patch` | *Defines `AnyOfLessThanGreaterThan` type (as `serde_json::Value`) | `src/models/mod.rs` |
| `export_some_internals.patch` | Exports some internal types for use in `sindri-rs` | `src/apis/mod.rs` |
| `no_circuit_response_nest.patch` | Circuit info is not nested by `circuit_type`, this patch removes the nesting | `src/models/circuit_info_response.rs` |
| `rm_id_options_restore_download.patch` | *Identifiers should not be optional for path params | `src/apis/internal_api.rs` |
| `rm_proof_id_option.patch` | *Identifier should not be optional for path params | `src/apis/proofs_api.rs` |
| `two_input_modes.patch` | Allows automatic inference of proof input type from string or JSON | `src/models/proof_input.rs` |
