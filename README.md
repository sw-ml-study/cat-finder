# Cat Finder

A Rust CLI tool that scans directories for photos containing cats using YOLO object detection.

## Quick Start

```bash
# Clone and set up in one command
git clone https://github.com/wrightmikea/cat-finder.git
cd cat-finder
./setup.sh

# Scan for cats
./scripts/run.sh ~/Pictures
```

## Installation

### One-Command Setup

The setup script handles everything—checks for Rust, downloads the model, and builds:

```bash
./setup.sh
```

### Manual Installation

If you prefer to set up manually:

1. Install Rust via [rustup.rs](https://rustup.rs) (version 1.85+)
2. Download the YOLO model:
   ```bash
   ./scripts/download_models.sh
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```

### Requirements

- **Rust 1.85+** (2024 edition)
- **~80MB disk space** (model + ONNX Runtime + binary)
- **No cloud APIs, Ollama, or LLMs required** — runs entirely locally

## Usage

```bash
cat-finder [OPTIONS] [PATH]
```

### Arguments
- `PATH` - Directory to scan for images (default: current directory)

### Options
- `-v, --verbose` - Show detailed progress
- `-t, --timestamp` - Show file timestamps
- `--confidence <FLOAT>` - Detection confidence threshold (0.0-1.0, default: 0.25)
- `--model <PATH>` - Path to ONNX model (default: models/yolov8n.onnx)

## Examples

```bash
# Scan current directory
cat-finder

# Scan with verbose output
cat-finder --verbose ~/Dropbox/Photos

# Lower confidence threshold for more detections
cat-finder --confidence 0.1 ~/Pictures

# Use with timestamps
cat-finder -t ~/Photos
```

## Model Information

The tool uses YOLOv8n (nano) by default, which can detect 80 object classes including cats. The model achieves good accuracy while being fast enough for scanning large photo collections. All inference runs locally via ONNX Runtime—no cloud APIs or external services required.

### Detection Classes
The model can detect: person, bicycle, car, motorcycle, airplane, bus, train, truck, boat, traffic light, fire hydrant, stop sign, parking meter, bench, bird, **cat**, dog, horse, sheep, cow, elephant, bear, zebra, giraffe, and many more.

## Troubleshooting

### macOS: Library not loaded error
Use the DYLD_LIBRARY_PATH environment variable:
```bash
DYLD_LIBRARY_PATH=target/release ./target/release/cat-finder
```

Or use the provided script:
```bash
./scripts/run.sh [path]
```

### Model file not found
Run the download script:
```bash
./scripts/download_models.sh
```

## Testing

Run the test script to verify the installation:

```bash
./scripts/test.sh
```

**Expected results:**
- 19 sample images in `samples/`
- 10 images contain cats (should be detected)
- 9 images do not contain cats (`two.jpg`, `small.jpg`, `tiny.jpg`, `notcat_*.jpg`)

A successful test shows: `Images with cats: 10`

## Development

Built with:
- Rust 2024 edition
- ONNX Runtime for local inference
- YOLOv8n for object detection
- clap for CLI parsing
- image for image processing

See [docs/usage.md](docs/usage.md) for detailed installation and usage instructions.

## License

MIT License — Copyright (c) 2025-2026 Michael A. Wright

See [LICENSE](LICENSE) for details.