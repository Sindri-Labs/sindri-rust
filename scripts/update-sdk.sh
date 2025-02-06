#! /bin/bash

# Use the project root as the working directory.
cd "$(dirname "$0")/../"

# Download the open api spec that has been downgraded to v3.0.3
curl -L \
  -H "Accept: application/vnd.github+json" \
  -H "Authorization: Bearer $GITHUB_TOKEN" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  -O \
  -L https://api.github.com/repos/Sindri-Labs/sindri-labs.github.io/contents/docs/reference/api/openapi.json

# Decode the base64 encoded content 
cat openapi.json | jq -r '.content' | base64 -d | jq '.' > openapi_decoded.json

# Generate the client
npx @openapitools/openapi-generator-cli generate -i openapi_decoded.json -g rust -o ./openapi --additional-properties=supportMiddleware=true

# Format the client.
cargo fmt -- ./openapi/

# Patch over the generated client with some manual changes
git apply openapi/openapi.patch

# Remove the spec files
rm openapi.json
rm openapi_decoded.json
rm openapitools.json
