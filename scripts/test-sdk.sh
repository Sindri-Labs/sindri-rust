#!/bin/bash
set -e  # Exit on any error

if [ -f "../.env" ]; then
    export $(cat ../.env | xargs)
    echo ".env file found and loaded"
fi

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
        cd ../sindri && cargo test
        ;;
    "record")
        rm -rf ../sindri/tests/recordings/*
        echo "Running tests in record mode..."
        cd ../sindri && cargo test --features record
        ;;
    "replay")
        echo "Running tests in replay mode..."
        cd ../sindri && cargo test --features replay
        ;;
    *)
        echo "Error: Invalid mode '$mode'"
        echo "Valid modes are: no-vcr, record, replay"
        exit 1
        ;;
esac
