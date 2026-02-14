#!/bin/bash

# Simple script to find cats in photos
# Usage: ./scripts/find.sh [path] [options]

# Set the directory of this script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

# Set library path for ONNX Runtime on macOS
export DYLD_LIBRARY_PATH="${PROJECT_DIR}/target/release:${DYLD_LIBRARY_PATH}"

# Check if the binary exists, build if not
if [ ! -f "${PROJECT_DIR}/target/release/cat-finder" ]; then
    echo "Building cat-finder..."
    cd "$PROJECT_DIR"
    cargo build --release
fi

# Run with all arguments
exec "${PROJECT_DIR}/target/release/cat-finder" "$@"