#!/usr/bin/env python3
"""
Download and convert YOLOv8 model to ONNX for cat detection
"""

import subprocess
import sys
import os

def main():
    # Install ultralytics if not installed
    try:
        import ultralytics
    except ImportError:
        print("Installing ultralytics package...")
        subprocess.check_call([sys.executable, "-m", "pip", "install", "ultralytics"])
        import ultralytics

    from ultralytics import YOLO

    print("Downloading YOLOv8n model (nano - fastest)...")

    # Load YOLOv8 nano model (will auto-download if not present)
    model = YOLO("yolov8n.pt")

    print("Converting to ONNX format...")

    # Export to ONNX
    # Use static shape for better compatibility
    path = model.export(format="onnx", imgsz=640, simplify=True)

    # Move to models directory (handle script being in scripts/)
    script_dir = os.path.dirname(os.path.abspath(__file__))
    project_dir = os.path.dirname(script_dir)
    models_dir = os.path.join(project_dir, "models")
    os.makedirs(models_dir, exist_ok=True)

    if os.path.exists("yolov8n.onnx"):
        import shutil
        target_path = os.path.join(models_dir, "yolov8n.onnx")
        shutil.move("yolov8n.onnx", target_path)
        print(f"✅ Model saved to {target_path}")
        print(f"   Size: {os.path.getsize(target_path) / 1024 / 1024:.1f} MB")
    else:
        print("❌ Failed to export model")
        return 1

    return 0

if __name__ == "__main__":
    sys.exit(main())