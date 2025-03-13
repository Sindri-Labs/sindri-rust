#!/bin/bash
set -e  # Exit on error

# Use the location of the script as the working directory.
cd "$(dirname "$0")"

# Load api key
if [ -f "../.env" ]; then
    export $(cat ../.env | xargs)
    echo ".env file found and loaded"
fi

# Set VCR_PATH environment variable and remove old recording
export VCR_PATH="../.github/assets/recordings/integration.vcr.json" 
rm -f "$VCR_PATH"

# Run test in record mode
cargo test --package sindri --features record --test offline -- end_to_end
