#!/bin/bash
set -e  # Exit on any error

# Check if mode argument is provided
if [ $# -ne 1 ]; then
    echo "Usage: $0 <mode>"
    echo "Modes: no-vcr, record, replay"
    exit 1
fi

mode=$1

# Validate the mode
case $mode in
    "no-vcr")
        echo "Running tests in no-vcr mode..."
        cd ../sindri-rs && cargo test
        ;;
    "record")
        echo "Running tests in record mode..."
        cd ../sindri-rs && cargo test --features record
        ;;
    "replay")
        echo "Running tests in replay mode..."
        cd ../sindri-rs && cargo test --features replay
        ;;
    *)
        echo "Error: Invalid mode '$mode'"
        echo "Valid modes are: no-vcr, record, replay"
        exit 1
        ;;
esac
