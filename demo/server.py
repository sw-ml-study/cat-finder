#!/usr/bin/env python3
"""
Cat Finder Web Demo
Real-time visualization of cat detection on sample images.
"""

import os
import subprocess
import json
import time
from pathlib import Path
from flask import Flask, Response, send_from_directory, render_template_string

app = Flask(__name__)

PROJECT_DIR = Path(__file__).parent.parent
SAMPLES_DIR = PROJECT_DIR / "samples"
CAT_FINDER = PROJECT_DIR / "target" / "release" / "cat-finder"
MODEL_PATH = PROJECT_DIR / "models" / "yolov8n.onnx"

HTML_TEMPLATE = """
<!DOCTYPE html>
<html>
<head>
    <meta http-equiv="Cache-Control" content="no-cache, no-store, must-revalidate">
    <meta http-equiv="Pragma" content="no-cache">
    <meta http-equiv="Expires" content="0">
    <title>Cat Finder Demo</title>
    <style>
        * { box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: #1a1a2e;
            color: #eee;
            margin: 0;
            padding: 20px;
        }
        h1 {
            text-align: center;
            color: #fff;
            margin-bottom: 10px;
        }
        .status {
            text-align: center;
            color: #888;
            margin-bottom: 20px;
            font-size: 14px;
        }
        .grid {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
            gap: 15px;
            max-width: 1200px;
            margin: 0 auto;
        }
        .card {
            background: #16213e;
            border-radius: 12px;
            overflow: hidden;
            position: relative;
            transition: transform 0.2s, box-shadow 0.3s;
        }
        .card:hover {
            transform: translateY(-3px);
        }
        .card.processing {
            box-shadow: 0 0 20px rgba(255, 193, 7, 0.5);
        }
        .card.cat {
            box-shadow: 0 0 20px rgba(76, 175, 80, 0.6);
        }
        .card.nocat {
            box-shadow: 0 0 20px rgba(244, 67, 54, 0.4);
        }
        .thumb {
            width: 100%;
            height: 150px;
            object-fit: cover;
            display: block;
        }
        .info {
            padding: 10px;
            font-size: 12px;
            color: #aaa;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }
        .badge {
            position: absolute;
            top: 10px;
            right: 10px;
            width: 40px;
            height: 40px;
            border-radius: 50%;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 24px;
            font-weight: bold;
            opacity: 0;
            transform: scale(0);
            transition: all 0.3s ease;
        }
        .badge.show {
            opacity: 1;
            transform: scale(1);
        }
        .badge.cat {
            background: #4caf50;
            color: white;
        }
        .badge.nocat {
            background: #f44336;
            color: white;
        }
        .spinner {
            position: absolute;
            top: 10px;
            right: 10px;
            width: 30px;
            height: 30px;
            border: 3px solid rgba(255,255,255,0.2);
            border-top-color: #ffc107;
            border-radius: 50%;
            animation: spin 0.8s linear infinite;
            opacity: 0;
        }
        .spinner.show {
            opacity: 1;
        }
        @keyframes spin {
            to { transform: rotate(360deg); }
        }
        .stats {
            text-align: center;
            margin-top: 20px;
            font-size: 18px;
        }
        .stats span {
            margin: 0 15px;
        }
        .cats-count { color: #4caf50; }
        .nocats-count { color: #f44336; }
        #start-btn {
            display: block;
            margin: 20px auto;
            padding: 12px 30px;
            font-size: 16px;
            background: #4caf50;
            color: white;
            border: none;
            border-radius: 8px;
            cursor: pointer;
            transition: background 0.2s;
        }
        #start-btn:hover {
            background: #45a049;
        }
        #start-btn:disabled {
            background: #666;
            cursor: not-allowed;
        }
    </style>
</head>
<body>
    <h1>Cat Finder Demo</h1>
    <p class="status" id="status">Click Start to begin detection</p>
    <button id="start-btn" onclick="startDemo()">Start Detection</button>
    <div class="stats">
        <span class="cats-count">Cats: <strong id="cat-count">0</strong></span>
        <span class="nocats-count">Not Cats: <strong id="nocat-count">0</strong></span>
    </div>
    <div class="grid" id="grid"></div>

    <script>
        const images = IMAGES_JSON;
        let catCount = 0;
        let nocatCount = 0;

        // Build grid with cache-busting timestamps
        const grid = document.getElementById('grid');
        const ts = Date.now();
        images.forEach(img => {
            const card = document.createElement('div');
            card.className = 'card';
            card.id = 'card-' + img.id;
            card.innerHTML = `
                <img class="thumb" src="/image/${img.filename}?ts=${ts}" alt="${img.filename}">
                <div class="spinner" id="spinner-${img.id}"></div>
                <div class="badge" id="badge-${img.id}"></div>
                <div class="info">${img.filename}</div>
            `;
            grid.appendChild(card);
        });

        function startDemo() {
            document.getElementById('start-btn').disabled = true;
            document.getElementById('status').textContent = 'Running cat detection...';
            catCount = 0;
            nocatCount = 0;
            document.getElementById('cat-count').textContent = '0';
            document.getElementById('nocat-count').textContent = '0';

            // Reset all cards
            images.forEach(img => {
                const card = document.getElementById('card-' + img.id);
                const badge = document.getElementById('badge-' + img.id);
                const spinner = document.getElementById('spinner-' + img.id);
                card.className = 'card';
                badge.className = 'badge';
                badge.textContent = '';
                spinner.className = 'spinner';
            });

            const evtSource = new EventSource('/detect');

            evtSource.onmessage = function(event) {
                const data = JSON.parse(event.data);

                if (data.type === 'processing') {
                    const card = document.getElementById('card-' + data.id);
                    const spinner = document.getElementById('spinner-' + data.id);
                    if (card) card.classList.add('processing');
                    if (spinner) spinner.classList.add('show');
                }
                else if (data.type === 'result') {
                    const card = document.getElementById('card-' + data.id);
                    const badge = document.getElementById('badge-' + data.id);
                    const spinner = document.getElementById('spinner-' + data.id);

                    if (spinner) spinner.classList.remove('show');
                    if (card) {
                        card.classList.remove('processing');
                        card.classList.add(data.has_cat ? 'cat' : 'nocat');
                    }
                    if (badge) {
                        badge.className = 'badge show ' + (data.has_cat ? 'cat' : 'nocat');
                        badge.textContent = data.has_cat ? '✓' : '✗';
                    }

                    if (data.has_cat) {
                        catCount++;
                        document.getElementById('cat-count').textContent = catCount;
                    } else {
                        nocatCount++;
                        document.getElementById('nocat-count').textContent = nocatCount;
                    }
                }
                else if (data.type === 'done') {
                    document.getElementById('status').textContent =
                        `Complete! Found ${catCount} cats in ${catCount + nocatCount} images.`;
                    document.getElementById('start-btn').disabled = false;
                    evtSource.close();
                }
            };

            evtSource.onerror = function() {
                document.getElementById('status').textContent = 'Connection error';
                document.getElementById('start-btn').disabled = false;
                evtSource.close();
            };
        }
    </script>
</body>
</html>
"""

def get_image_list():
    """Get list of sample images."""
    images = []
    for i, f in enumerate(sorted(SAMPLES_DIR.rglob("*"))):
        if f.suffix.lower() in ['.jpg', '.jpeg', '.png', '.gif', '.webp']:
            images.append({
                'id': i,
                'filename': f.name,
                'path': str(f.relative_to(SAMPLES_DIR))
            })
    return images

@app.route('/')
def index():
    images = get_image_list()
    html = HTML_TEMPLATE.replace('IMAGES_JSON', json.dumps(images))
    return html

@app.route('/image/<path:filename>')
def serve_image(filename):
    # Handle nested paths (strip query params)
    filename = filename.split('?')[0]
    for img_path in SAMPLES_DIR.rglob(filename):
        if img_path.is_file():
            response = send_from_directory(img_path.parent, img_path.name)
            response.headers['Cache-Control'] = 'no-cache, no-store, must-revalidate'
            response.headers['Pragma'] = 'no-cache'
            response.headers['Expires'] = '0'
            return response
    return "Not found", 404

@app.route('/detect')
def detect():
    """SSE endpoint that runs cat-finder and streams results."""
    def generate():
        images = get_image_list()
        image_map = {img['path']: img for img in images}

        # Set up environment for ONNX Runtime
        env = os.environ.copy()
        env['DYLD_LIBRARY_PATH'] = str(PROJECT_DIR / "target" / "release")

        for img in images:
            img_id = img['id']
            img_path = SAMPLES_DIR / img['path']

            # Send processing event
            yield f"data: {json.dumps({'type': 'processing', 'id': img_id})}\n\n"

            # Small delay for visual effect
            time.sleep(0.3)

            # Run cat-finder on single image
            try:
                result = subprocess.run(
                    [str(CAT_FINDER), str(img_path), '--model', str(MODEL_PATH)],
                    capture_output=True,
                    text=True,
                    env=env,
                    timeout=30
                )
                has_cat = img_path.name in result.stdout or str(img_path) in result.stdout
            except Exception as e:
                has_cat = False

            # Send result
            yield f"data: {json.dumps({'type': 'result', 'id': img_id, 'has_cat': has_cat})}\n\n"

            # Small delay between images
            time.sleep(0.2)

        yield f"data: {json.dumps({'type': 'done'})}\n\n"

    return Response(generate(), mimetype='text/event-stream')

if __name__ == '__main__':
    print(f"Starting Cat Finder Demo...")
    print(f"Samples directory: {SAMPLES_DIR}")
    print(f"Cat finder binary: {CAT_FINDER}")
    print(f"\nOpen http://localhost:5001 in your browser\n")
    app.run(host='0.0.0.0', port=5001, debug=False, threaded=True)
