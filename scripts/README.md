# Internal Development Scripts

The `scripts/` directory automates rust SDK updates and testing.

## Updates

The `update-sdk.sh` script will grab the newest API specification and regenerate the base Sindri API client via openapi-generator.
All of the code in `openapi/` is updated with the regeneration, after which manual patches from `openapi/patches/` are applied to the generated client.

#### Usage Notes
* Do not remove `scripts/openapitools.json`. This provides an openapi-generator version lock.
* If any patch fails to apply, consult `openapi/patches/README.md` for more information.
* As of PR #17, a github access token is no longer required.
* This script updates the internal base `sindri-openapi` client, but not the external `sindri` package.
  * If a new framework has been added, you must add the circuit info response type to `sindri/src/types.rs`.
* While CI will perform unit tests that provide preliminary checks, a full live test (via `test-sdk.sh`) should follow the update.

## Tests

The `test-sdk.sh` script requires an argument specifying one of three modes:
* `no-vcr`: Runs `sindri` tests without VCR recording/replaying (sending requests to the Sindri API).
* `record`: Runs `sindri` tests with VCR recording.  This will send requests to the Sindri API and record both requests and responses in a directory specified by the environment variable `VCR_PATH`.
* `replay`: Runs `sindri` tests with VCR replaying.  The `sindri` client will use the recording in `VCR_PATH` to replay the requests and responses.  If any new type of request is made and not found in the recording, the test will fail.

#### Usage notes
* These tests require the environment variable `SINDRI_API_KEY` set in your `.env` file.
* The `VCR_PATH` environment variable is optional.  If not set, the default path will be used (`tests/recordings/replay.vcr.json`)
* While `replay` is an available mode, it is not recommended to evaluate branches or PRs based on the results of replay tests. There are many reasons why a recording will fail one of the integration tests. 
  * For instance, nondeterministic request formation will cause a failure to find a matching request. 
  * Another reason `replay` may fail is that the first matching request (e.g. details for a circuit that has been deleted) may not be the correct match given the previous context within the test case.

---

The `update-ci-fixtures.sh` script is meant to replace a recording in `.github/assets/recordings/`.  CI does not make live API requests. If CI fails after an update, it is generally caused by the `sindri` client sending more information (headers, etc) than a recording contains.  Removing the previous recording and re-running the script will update the fixture. (Make sure the CI failure does not represent true backwards incompatibility first.)

#### Usage notes
* A valid api key is not required to run the CI tests (in replay mode) but it is required to replace the fixtures. Make sure that `SINDRI_API_KEY` is set in your `.env` file before running the script.
