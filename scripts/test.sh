#!/bin/bash

# Test script for cat-finder
# Uses the samples directory for quick testing
#
# Expected results:
#   - 10 total images in samples/
#   - 7 images contain cats (should be detected)
#   - 3 images do not contain cats (two.jpg, small.jpg, tiny.jpg)

# Set the directory of this script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

# Set library path for ONNX Runtime on macOS
export DYLD_LIBRARY_PATH="${PROJECT_DIR}/target/release"

# Default test directory
TEST_DIR="${PROJECT_DIR}/samples"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo "Cat Finder - Test Script"
echo "========================"
echo ""

# Check if samples directory exists
if [ ! -d "$TEST_DIR" ]; then
    echo -e "${RED}Error: samples directory not found!${NC}"
    echo "Please add test images to: $TEST_DIR"
    echo "  - samples/one.jpg (should contain a cat)"
    echo "  - samples/two.jpg (should not contain a cat)"
    exit 1
fi

# Check if binary exists
if [ ! -f "${PROJECT_DIR}/target/release/cat-finder" ]; then
    echo -e "${RED}Error: cat-finder binary not found!${NC}"
    echo "Building release version..."
    cd "$PROJECT_DIR"
    cargo build --release
fi

# Count images in samples
IMAGE_COUNT=$(find "$TEST_DIR" -type f \( -iname "*.jpg" -o -iname "*.jpeg" -o -iname "*.png" \) | wc -l)
echo "Found $IMAGE_COUNT images in samples directory"
echo ""

# Run with verbose output and very low confidence
echo -e "${GREEN}Testing with verbose output (confidence 0.01):${NC}"
"${PROJECT_DIR}/target/release/cat-finder" "$TEST_DIR" --verbose --confidence 0.01

echo ""
echo -e "${GREEN}Testing without verbose (just results):${NC}"
"${PROJECT_DIR}/target/release/cat-finder" "$TEST_DIR" --confidence 0.1

# Test with custom confidence threshold
echo ""
echo -e "${GREEN}Testing with higher confidence (0.5):${NC}"
"${PROJECT_DIR}/target/release/cat-finder" "$TEST_DIR" --confidence 0.5 --verbose