#!/bin/bash
set -e  # Exit on error

# Load api key
if [ -f "../.env" ]; then
    export $(cat ../.env | xargs)
    echo ".env file found and loaded"
fi

# Set VCR_PATH environment variable
export VCR_PATH="../.github/assets/recordings/integration.vcr.json" 

# Run test in record mode
cargo test --package sindri --features record --test offline -- end_to_end
