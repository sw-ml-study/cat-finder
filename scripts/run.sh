#!/bin/bash

# Script to run cat-finder with proper library path on macOS
# Usage: ./scripts/run.sh [path] [options]

# Set the directory of this script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

# Set library path for ONNX Runtime on macOS
export DYLD_LIBRARY_PATH="${PROJECT_DIR}/target/release"

# Default values
DEFAULT_PATH="${1:-$PROJECT_DIR/test_images}"
DEFAULT_MODEL="${PROJECT_DIR}/models/yolov8n.onnx"

# Check if the binary exists
if [ ! -f "${PROJECT_DIR}/target/release/cat-finder" ]; then
    echo "Error: cat-finder binary not found!"
    echo "Please run: cargo build --release"
    exit 1
fi

# Check if model exists
if [ ! -f "$DEFAULT_MODEL" ]; then
    echo "Warning: Model file not found at $DEFAULT_MODEL"
    echo "Please run: ./download_models.sh"
fi

# Run cat-finder with all arguments passed to this script
"${PROJECT_DIR}/target/release/cat-finder" "$@" --model "$DEFAULT_MODEL"