# Usage Guide

## What This Tool Does

Cat Finder scans directories for photographs containing cats using YOLOv8 object detection. It runs entirely locally on your machine—no cloud services, API keys, or external servers required.

## Prerequisites

### Required

| Requirement | Details |
|-------------|---------|
| **Rust toolchain** | Version 1.85+ (2024 edition). Install via [rustup.rs](https://rustup.rs) |
| **ONNX model file** | YOLOv8n (~12MB), downloaded via provided script |

### Not Required

| Dependency | Status |
|------------|--------|
| **Ollama** | Not needed |
| **LLM models** | Not needed (uses computer vision, not language models) |
| **Cloud API keys** | Not needed |
| **GPU** | Optional (CPU inference works fine) |
| **Python** | Not needed for running (only for alternative model download scripts) |

### System Requirements

- **macOS**: Tested and supported (requires helper script or DYLD_LIBRARY_PATH)
- **Linux**: Supported
- **Windows**: Should work but untested; may need path adjustments

### Disk Space

| Component | Size |
|-----------|------|
| ONNX Runtime (auto-downloaded during build) | ~50 MB |
| YOLOv8n model | ~12 MB |
| Compiled binary | ~15 MB |
| **Total** | ~80 MB |

## Installation

### Step 1: Install Rust

If you don't have Rust installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

Verify installation:

```bash
rustc --version   # Should show 1.85.0 or later
cargo --version
```

### Step 2: Clone the Repository

```bash
git clone https://github.com/wrightmikea/cat-finder.git
cd cat-finder
```

### Step 3: Download the Model

```bash
./scripts/download_models.sh
```

This downloads `yolov8n.onnx` (~12MB) to the `models/` directory.

**Alternative methods:**

```bash
# Direct download
curl -L -o models/yolov8n.onnx \
  https://github.com/ultralytics/assets/releases/download/v8.3.0/yolov8n.onnx

# Or using wget
wget -O models/yolov8n.onnx \
  https://github.com/ultralytics/assets/releases/download/v8.3.0/yolov8n.onnx
```

### Step 4: Build

```bash
cargo build --release
```

First build takes 2-5 minutes as it:
1. Downloads dependencies from crates.io
2. Downloads ONNX Runtime binaries (~50MB)
3. Compiles the project

Subsequent builds are fast (~5 seconds).

## Running

### macOS

Use the provided script (handles library path automatically):

```bash
./scripts/run.sh ~/Pictures
```

Or set the environment variable manually:

```bash
DYLD_LIBRARY_PATH=target/release ./target/release/cat-finder ~/Pictures
```

### Linux

```bash
./target/release/cat-finder ~/Pictures
```

### Windows (untested)

```powershell
.\target\release\cat-finder.exe C:\Users\You\Pictures
```

## Command-Line Options

```
cat-finder [OPTIONS] [PATH]
```

| Option | Description | Default |
|--------|-------------|---------|
| `PATH` | Directory to scan | Current directory |
| `-v, --verbose` | Show detailed progress | Off |
| `-t, --timestamp` | Show file modification times | Off |
| `--confidence <FLOAT>` | Detection threshold (0.0-1.0) | 0.25 |
| `--model <PATH>` | Path to ONNX model | `models/yolov8n.onnx` |

## Examples

```bash
# Scan current directory
./scripts/run.sh .

# Scan with verbose output
./scripts/run.sh ~/Photos --verbose

# High confidence (fewer false positives)
./scripts/run.sh ~/Pictures --confidence 0.5

# Low confidence (find more cats, more false positives)
./scripts/run.sh ~/Pictures --confidence 0.1

# Show timestamps
./scripts/run.sh ~/Photos -t

# Combine options
./scripts/run.sh ~/Dropbox/Camera --verbose --timestamp --confidence 0.3
```

## Output

The tool prints paths to images containing cats:

```
/Users/you/Photos/vacation/beach_cat.jpg
/Users/you/Photos/2023/fluffy.png
Found 2 images containing cats
```

With `--verbose`:

```
Scanning: /Users/you/Photos
Processing: /Users/you/Photos/vacation/beach_cat.jpg
  Found cat with confidence 0.87
Processing: /Users/you/Photos/vacation/sunset.jpg
  No cats detected
...
```

## Troubleshooting

### "Library not loaded" error (macOS)

Use the helper script or set DYLD_LIBRARY_PATH:

```bash
# Option 1: Use script
./scripts/run.sh ~/Pictures

# Option 2: Set env var
DYLD_LIBRARY_PATH=target/release ./target/release/cat-finder ~/Pictures
```

### "Model file not found"

Download the model:

```bash
./scripts/download_models.sh
```

Or verify it exists:

```bash
ls -la models/yolov8n.onnx
```

### Build fails with Rust version error

Update Rust:

```bash
rustup update stable
```

### Slow first build

Normal. The first build downloads ONNX Runtime (~50MB) and compiles all dependencies. Subsequent builds are fast.

### No cats found in images that have cats

Try lowering the confidence threshold:

```bash
./scripts/run.sh ~/Pictures --confidence 0.1
```

### Too many false positives

Raise the confidence threshold:

```bash
./scripts/run.sh ~/Pictures --confidence 0.5
```

## How It Works

1. **Walks directory tree** - Recursively finds image files (jpg, png, gif, bmp, webp, tiff)
2. **Preprocesses images** - Resizes to 640x640, normalizes pixel values
3. **Runs inference** - Passes through YOLOv8n model via ONNX Runtime
4. **Parses detections** - Extracts bounding boxes and class scores
5. **Filters for cats** - Keeps detections where class=15 (cat in COCO) and confidence >= threshold
6. **Reports results** - Prints paths of images containing cats

All processing happens locally on your CPU. No data leaves your machine.
