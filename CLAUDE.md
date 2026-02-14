# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Cat Finder is a Rust CLI tool that scans directories for photos containing cats using YOLOv8 object detection via ONNX Runtime. All inference runs locally—no cloud APIs or LLMs required.

## Build and Run Commands

```bash
# Initial setup (installs Rust if needed, downloads model, builds)
./setup.sh

# Build only
cargo build --release

# Run on macOS (handles DYLD_LIBRARY_PATH for ONNX Runtime)
./scripts/run.sh ~/Pictures [--verbose] [--confidence 0.25]

# Run on Linux
./target/release/cat-finder ~/Pictures

# Run tests (uses samples/ directory)
./scripts/test.sh

# Download YOLO model if missing
./scripts/download_models.sh
```

## Architecture

### Binaries

- **cat-finder** (`src/main.rs`): Main tool that walks directories, runs YOLOv8 inference on images, and reports files containing cats
- **find-duplicates** (`src/bin/find-duplicates.rs`): Utility to find duplicate images by size and SHA-256 checksum

### Detection Pipeline

1. `YoloCatDetector::new()` initializes ONNX Runtime session with the YOLOv8n model
2. `detect_cats()` processes each image: loads → `preprocess_image()` resizes to 640x640 NCHW format → runs inference
3. YOLOv8 output is `[1, 84, 8400]` where 84 = 4 bbox coords + 80 COCO class scores
4. Filters for class ID 15 (cat) with confidence above threshold (default 0.25)

### Key Constants

- `CAT_CLASS_ID = 15` in COCO class ordering
- Default input size: 640x640
- Supported image formats: jpg, jpeg, png, gif, bmp, webp, tiff

## macOS Notes

ONNX Runtime shared libraries require `DYLD_LIBRARY_PATH=target/release`. The `scripts/run.sh` wrapper handles this automatically. The `.cargo/config.toml` also sets rpath flags for the build.
