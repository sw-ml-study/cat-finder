#!/bin/bash

# Demo script: search a directory for cat images
# Usage: ./scripts/demo.sh [directory]
# Default: ~/Downloads

# Set the directory of this script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

# Set library path for ONNX Runtime on macOS
export DYLD_LIBRARY_PATH="${PROJECT_DIR}/target/release"

# Default model path
DEFAULT_MODEL="${PROJECT_DIR}/models/yolov8n.onnx"

# Target directory (default: ~/Downloads)
TARGET_DIR="${1:-$HOME/Downloads}"

# Check if the binary exists
if [ ! -f "${PROJECT_DIR}/target/release/cat-finder" ]; then
    echo "Error: cat-finder binary not found!"
    echo "Please run: cargo build --release"
    exit 1
fi

# Check if model exists
if [ ! -f "$DEFAULT_MODEL" ]; then
    echo "Error: Model file not found at $DEFAULT_MODEL"
    echo "Please run: ./scripts/download_models.sh"
    exit 1
fi

# Check if target directory exists
if [ ! -d "$TARGET_DIR" ]; then
    echo "Error: Directory not found: $TARGET_DIR"
    exit 1
fi

echo "Searching for cat images in: $TARGET_DIR"
echo ""

# Run cat-finder with verbose output
"${PROJECT_DIR}/target/release/cat-finder" "$TARGET_DIR" --verbose --model "$DEFAULT_MODEL"
