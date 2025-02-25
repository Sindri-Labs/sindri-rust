#! /bin/bash

# Use the location of the script as the working directory.
cd "$(dirname "$0")"

# Download the most recent open api spec
wget https://sindri.app/api/openapi.json

# Generate the client
npx @openapitools/openapi-generator-cli@2.16.3 generate -i openapi.json -g rust -o ../openapi --additional-properties=supportMiddleware=true

# Move up to the project root.
cd ..

# Format the client.
rustfmt $(find ./openapi/ -name "*.rs")

# Patch over the generated client with some manual changes
# git apply openapi/openapi.patch

# Move back into scripts and remove the spec files.
cd ./scripts/
rm openapi.json
