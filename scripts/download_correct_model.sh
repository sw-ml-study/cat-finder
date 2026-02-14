#!/bin/bash

# Download working YOLOv8n ONNX model from Ultralytics
echo "Downloading YOLOv8n ONNX model..."

cd models

# Download from correct source (v8.2.0 release has ONNX models)
curl -L "https://github.com/ultralytics/assets/releases/download/v8.2.0/yolov8n.onnx" -o yolov8n_proper.onnx

# Check file size
if [ -f yolov8n_proper.onnx ]; then
    SIZE=$(ls -lh yolov8n_proper.onnx | awk '{print $5}')
    echo "Downloaded yolov8n_proper.onnx - Size: $SIZE"

    # If size is reasonable (should be ~13MB)
    FILE_SIZE_KB=$(du -k yolov8n_proper.onnx | cut -f1)
    if [ $FILE_SIZE_KB -gt 10000 ]; then
        echo "✅ Model downloaded successfully!"
        mv yolov8n_proper.onnx yolov8n.onnx
        echo "Model saved as yolov8n.onnx"
    else
        echo "❌ Downloaded file is too small, possibly an error page"
        rm yolov8n_proper.onnx
    fi
fi