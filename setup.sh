#!/bin/bash
#
# Cat Finder Setup Script
# Handles prerequisites check, model download, and build
#

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_step() {
    echo -e "\n${GREEN}==>${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}Warning:${NC} $1"
}

print_error() {
    echo -e "${RED}Error:${NC} $1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

# Check for Rust
check_rust() {
    print_step "Checking for Rust toolchain..."

    if command -v rustc &> /dev/null; then
        RUST_VERSION=$(rustc --version | cut -d' ' -f2)
        print_success "Rust $RUST_VERSION found"
        return 0
    else
        print_warning "Rust not found"
        echo ""
        echo "Rust is required to build this project."
        echo "Install it with:"
        echo ""
        echo "    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        echo ""
        read -p "Would you like to install Rust now? [y/N] " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
            source "$HOME/.cargo/env"
            print_success "Rust installed"
        else
            print_error "Rust is required. Please install it and run this script again."
            exit 1
        fi
    fi
}

# Download the YOLO model
download_model() {
    print_step "Checking for YOLOv8n model..."

    MODEL_PATH="models/yolov8n.onnx"
    MODEL_URL="https://github.com/ultralytics/assets/releases/download/v8.3.0/yolov8n.onnx"

    mkdir -p models

    if [ -f "$MODEL_PATH" ]; then
        print_success "Model already exists at $MODEL_PATH"
        return 0
    fi

    echo "Downloading YOLOv8n model (~12MB)..."

    if command -v curl &> /dev/null; then
        curl -L -o "$MODEL_PATH" "$MODEL_URL"
    elif command -v wget &> /dev/null; then
        wget -O "$MODEL_PATH" "$MODEL_URL"
    else
        print_error "Neither curl nor wget found. Please install one and try again."
        exit 1
    fi

    if [ -f "$MODEL_PATH" ]; then
        print_success "Model downloaded to $MODEL_PATH"
    else
        print_error "Failed to download model"
        exit 1
    fi
}

# Build the project
build_project() {
    print_step "Building project (this may take a few minutes on first run)..."

    cargo build --release

    if [ -f "target/release/cat-finder" ]; then
        print_success "Build complete"
    else
        print_error "Build failed"
        exit 1
    fi
}

# Print usage instructions
print_usage() {
    echo ""
    echo -e "${GREEN}Setup complete!${NC}"
    echo ""
    echo "To scan a directory for cat photos:"
    echo ""

    if [[ "$OSTYPE" == "darwin"* ]]; then
        echo "    ./scripts/run.sh ~/Pictures"
        echo ""
        echo "Or with options:"
        echo ""
        echo "    ./scripts/run.sh ~/Pictures --verbose"
        echo "    ./scripts/run.sh ~/Pictures --confidence 0.5"
    else
        echo "    ./target/release/cat-finder ~/Pictures"
        echo ""
        echo "Or with options:"
        echo ""
        echo "    ./target/release/cat-finder ~/Pictures --verbose"
        echo "    ./target/release/cat-finder ~/Pictures --confidence 0.5"
    fi

    echo ""
    echo "See docs/usage.md for full documentation."
    echo ""
}

# Main
main() {
    echo "=============================="
    echo "  Cat Finder Setup"
    echo "=============================="

    check_rust
    download_model
    build_project
    print_usage
}

main
