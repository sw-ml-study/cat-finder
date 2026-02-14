#!/bin/bash

# Script to download YOLOv8 model for cat detection
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MODELS_DIR="$SCRIPT_DIR/models"

echo "Cat Finder - Model Download Script"
echo "===================================="
echo ""

# Create models directory if it doesn't exist
if [ ! -d "$MODELS_DIR" ]; then
    echo "Creating models directory..."
    mkdir -p "$MODELS_DIR"
fi

cd "$MODELS_DIR"

# YOLOv8 nano model (smallest and fastest)
# Direct download from Ultralytics
MODEL_URL="https://github.com/ultralytics/assets/releases/download/v0.0.0/yolov8n.onnx"
MODEL_FILE="yolov8n.onnx"
MODEL_SIZE="13MB"

echo "Downloading YOLOv8 nano model ($MODEL_SIZE)..."
echo "This model can detect 80 object classes including cats."
echo ""

# Download with progress bar
if command -v curl >/dev/null 2>&1; then
    curl -L --progress-bar "$MODEL_URL" -o "$MODEL_FILE" || {
        echo "Failed to download model from primary source."
        echo "Trying alternative source..."
        # Alternative: Download from ONNX Model Zoo
        curl -L --progress-bar "https://media.githubusercontent.com/media/onnx/models/main/validated/vision/object_detection_segmentation/yolov8/model/yolov8n.onnx" -o "$MODEL_FILE"
    }
elif command -v wget >/dev/null 2>&1; then
    wget --show-progress "$MODEL_URL" -O "$MODEL_FILE" || {
        echo "Failed to download model from primary source."
        echo "Trying alternative source..."
        wget --show-progress "https://media.githubusercontent.com/media/onnx/models/main/validated/vision/object_detection_segmentation/yolov8/model/yolov8n.onnx" -O "$MODEL_FILE"
    }
else
    echo "Error: Neither curl nor wget is installed."
    echo "Please install curl or wget and run this script again."
    exit 1
fi

# Verify download
if [ -f "$MODEL_FILE" ]; then
    FILE_SIZE=$(du -h "$MODEL_FILE" | cut -f1)
    echo ""
    echo "✅ Model downloaded successfully!"
    echo "   File: $MODELS_DIR/$MODEL_FILE"
    echo "   Size: $FILE_SIZE"
    echo ""
    echo "The model can detect the following objects:"
    echo "  - Animals: cat, dog, horse, sheep, cow, elephant, bear, zebra, giraffe"
    echo "  - People: person"
    echo "  - Vehicles: bicycle, car, motorcycle, airplane, bus, train, truck, boat"
    echo "  - And many more (80 classes total)"
    echo ""
    echo "You can now run: cargo build --release"
else
    echo "❌ Error: Failed to download model"
    exit 1
fi