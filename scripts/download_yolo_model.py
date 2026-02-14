#!/usr/bin/env python3
"""
Download YOLOv8 nano ONNX model for cat detection
"""

import os
import urllib.request
import sys

def download_model():
    # Create models directory if it doesn't exist
    os.makedirs('models', exist_ok=True)

    model_path = 'models/yolov8n.onnx'

    # YOLOv8n ONNX model from official source (13MB)
    # This URL is from the ultralytics model zoo
    urls = [
        "https://github.com/ultralytics/assets/releases/download/v8.1.0/yolov8n.onnx",
        "https://github.com/ultralytics/assets/releases/download/v0.0.0/yolov8n.onnx",
    ]

    for url in urls:
        try:
            print(f"Attempting to download from: {url}")
            urllib.request.urlretrieve(url, model_path)

            # Check file size
            size = os.path.getsize(model_path)
            size_mb = size / (1024 * 1024)

            if size_mb > 10:  # Should be around 13MB
                print(f"✅ Successfully downloaded YOLOv8n model ({size_mb:.1f} MB)")
                return True
            else:
                print(f"❌ Downloaded file too small ({size_mb:.1f} MB), trying next URL...")
                os.remove(model_path)
        except Exception as e:
            print(f"Failed to download from {url}: {e}")
            continue

    print("\n❌ Failed to download model from all sources")
    print("\nAlternative: Download manually from:")
    print("https://github.com/ultralytics/assets/releases")
    print("Look for yolov8n.onnx (should be around 13MB)")
    return False

if __name__ == "__main__":
    success = download_model()
    sys.exit(0 if success else 1)