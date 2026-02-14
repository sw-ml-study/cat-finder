use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use clap::Parser;
use image::DynamicImage;
use ndarray::{Array, ArrayBase, IxDyn, OwnedRepr};
use ort::{Environment, Session, SessionBuilder, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(name = "cat-finder")]
#[command(about = "Scans directories for photos containing cats using YOLOv8", long_about = None)]
struct Args {
    /// Path to start searching for photos (default: current directory)
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Show verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Show timestamp (F for file-based, M for metadata-based)
    #[arg(short = 't', long)]
    timestamp: bool,

    /// Confidence threshold for detection (0.0-1.0)
    #[arg(long, default_value = "0.25")]
    confidence: f32,

    /// Path to YOLO ONNX model file
    #[arg(long, default_value = "models/yolov8n.onnx")]
    model: PathBuf,
}

// YOLO COCO class names (for reference, not used in simplified detection)
#[allow(dead_code)]
const YOLO_CLASSES: [&str; 80] = [
    "person", "bicycle", "car", "motorcycle", "airplane", "bus", "train", "truck", "boat",
    "traffic light", "fire hydrant", "stop sign", "parking meter", "bench", "bird", "cat",
    "dog", "horse", "sheep", "cow", "elephant", "bear", "zebra", "giraffe", "backpack",
    "umbrella", "handbag", "tie", "suitcase", "frisbee", "skis", "snowboard", "sports ball",
    "kite", "baseball bat", "baseball glove", "skateboard", "surfboard", "tennis racket",
    "bottle", "wine glass", "cup", "fork", "knife", "spoon", "bowl", "banana", "apple",
    "sandwich", "orange", "broccoli", "carrot", "hot dog", "pizza", "donut", "cake", "chair",
    "couch", "potted plant", "bed", "dining table", "toilet", "tv", "laptop", "mouse", "remote",
    "keyboard", "cell phone", "microwave", "oven", "toaster", "sink", "refrigerator", "book",
    "clock", "vase", "scissors", "teddy bear", "hair drier", "toothbrush"
];

const CAT_CLASS_ID: usize = 15;  // Index of "cat" in YOLO classes

struct YoloCatDetector {
    session: Session,
    confidence_threshold: f32,
}

impl YoloCatDetector {
    fn new(model_path: &Path, confidence: f32) -> Result<Self> {
        // Initialize ONNX Runtime environment
        let environment = Arc::new(
            Environment::builder()
                .with_name("cat_detector")
                .build()
                .context("Failed to create ONNX Runtime environment")?
        );

        // Load ONNX model
        let session = SessionBuilder::new(&environment)?
            .with_model_from_file(model_path)
            .context("Failed to load ONNX model")?;

        // Print model info for debugging
        eprintln!("Model inputs: {:?}", session.inputs.iter().map(|i| &i.name).collect::<Vec<_>>());
        eprintln!("Model outputs: {:?}", session.outputs.iter().map(|o| &o.name).collect::<Vec<_>>());

        Ok(Self {
            session,
            confidence_threshold: confidence,
        })
    }

    fn detect_cats(&self, image_path: &Path) -> Result<bool> {
        // Load and preprocess image
        let img = image::open(image_path)
            .with_context(|| format!("Failed to open image: {}", image_path.display()))?;

        let input_tensor = self.preprocess_image(img);

        // Create ORT tensor - YOLOv8 only needs the image input
        let shape = input_tensor.shape().to_vec();
        let flattened: Vec<f32> = input_tensor.iter().copied().collect();
        let cow_array = ndarray::CowArray::from(flattened.as_slice()).into_shape(shape).unwrap();

        let input_tensor_values = Value::from_array(
            self.session.allocator(),
            &cow_array
        ).context("Failed to create input tensor")?;

        // Run inference - YOLOv8 takes single input
        let outputs = self.session
            .run(vec![input_tensor_values])
            .with_context(|| {
                format!("Failed to run inference. Input shape: {:?}", input_tensor.shape())
            })?;

        // YOLOv8 output format: [1, 84, 8400]
        // Where 84 = 4 bbox coords + 80 class scores
        // 8400 = number of predictions

        let output = outputs[0]
            .try_extract::<f32>()
            .context("Failed to extract output tensor")?;

        let output_view = output.view();
        let shape = output_view.shape();

        eprintln!("YOLOv8 output shape: {:?}", shape);

        // Expected shape: [1, 84, 8400]
        if shape.len() == 3 && shape[1] == 84 {
            let num_predictions = shape[2];

            // Process each prediction
            for i in 0..num_predictions {
                // Get the prediction data for this anchor
                let mut class_scores = vec![];
                for class_id in 0..80 {
                    class_scores.push((class_id, output_view[[0, 4 + class_id, i]]));
                }

                // Find the class with highest score
                if let Some((class_id, score)) = class_scores.iter()
                    .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                {
                    // Check if it's a cat with sufficient confidence
                    if *class_id == CAT_CLASS_ID && *score > self.confidence_threshold {
                        eprintln!("CAT DETECTED! Confidence: {:.3}", score);
                        return Ok(true);
                    }

                    // Debug: show high confidence detections
                    if *score > 0.3 && i < 10 {
                        eprintln!("Detection {}: class_id={}, confidence={:.3}", i, class_id, score);
                    }
                }
            }

            Ok(false)
        } else {
            eprintln!("Unexpected output shape: {:?}", shape);
            Ok(false)
        }
    }

    fn preprocess_image(&self, img: DynamicImage) -> Array<f32, IxDyn> {
        // Resize to 640x640 (YOLOv8 input size)
        let img = img.resize_exact(640, 640, image::imageops::FilterType::Triangle);
        let img = img.to_rgb8();

        // Convert to NCHW format and normalize
        let mut input = Array::zeros(IxDyn(&[1, 3, 640, 640]));

        for (x, y, pixel) in img.enumerate_pixels() {
            input[[0, 0, y as usize, x as usize]] = f32::from(pixel[0]) / 255.0;
            input[[0, 1, y as usize, x as usize]] = f32::from(pixel[1]) / 255.0;
            input[[0, 2, y as usize, x as usize]] = f32::from(pixel[2]) / 255.0;
        }

        input
    }

    fn has_cat_detection(&self, output: &ArrayBase<OwnedRepr<f32>, IxDyn>) -> bool {
        // Tiny YOLOv3 with NMS outputs: [num_detections, 6]
        // Each detection: [x1, y1, x2, y2, confidence, class_id]

        // Try to get a 2D view if possible
        if let Some(data) = output.as_slice() {
            // The model outputs detections in batches of 6 values
            for chunk in data.chunks(6) {
                if chunk.len() >= 6 {
                    let class_id = chunk[5] as usize;
                    let confidence = chunk[4];

                    // Check if this is a cat detection with sufficient confidence
                    if class_id == CAT_CLASS_ID && confidence > self.confidence_threshold {
                        return true;
                    }
                }
            }
        } else {
            // Fallback: try to interpret as 2D array
            let shape = output.shape();
            if shape.len() == 2 && shape[1] >= 6 {
                for i in 0..shape[0] {
                    let class_id = output[[i, 5]] as usize;
                    let confidence = output[[i, 4]];

                    if class_id == CAT_CLASS_ID && confidence > self.confidence_threshold {
                        return true;
                    }
                }
            }
        }

        false
    }
}

fn get_image_timestamp(path: &Path) -> Option<(DateTime<Local>, char)> {
    // Get file modification time
    fs::metadata(path)
        .ok()
        .and_then(|metadata| metadata.modified().ok())
        .map(|modified| (DateTime::from(modified), 'F'))
}

fn is_image_file(path: &Path) -> bool {
    path.extension().map_or(false, |ext| {
        let ext = ext.to_string_lossy().to_lowercase();
        matches!(
            ext.as_str(),
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "tiff" | "tif"
        )
    })
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Check if model file exists
    if !args.model.exists() {
        eprintln!("Error: Model file not found at {}", args.model.display());
        eprintln!("Please run ./download_models.sh to download the YOLOv8 model.");
        std::process::exit(1);
    }

    if args.verbose {
        eprintln!("Loading YOLOv8 model from {}...", args.model.display());
    }

    // Initialize detector
    let detector = YoloCatDetector::new(&args.model, args.confidence)?;

    if args.verbose {
        eprintln!("Model loaded successfully!");
        eprintln!("Scanning directory: {}", args.path.display());
        eprintln!("Confidence threshold: {}", args.confidence);
    }

    let mut found_count = 0;
    let mut total_count = 0;
    let mut error_count = 0;

    for entry in WalkDir::new(&args.path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if !path.is_file() || !is_image_file(path) {
            continue;
        }

        total_count += 1;

        if args.verbose {
            eprint!("Analyzing: {} ... ", path.display());
        }

        match detector.detect_cats(path) {
            Ok(has_cats) => {
                if args.verbose {
                    eprintln!("{}", if has_cats { "CAT FOUND!" } else { "no cats" });
                }

                if has_cats {
                    found_count += 1;

                    if args.timestamp {
                        if let Some((timestamp, source)) = get_image_timestamp(path) {
                            println!(
                                "{} [{}:{}]",
                                path.display(),
                                source,
                                timestamp.format("%Y-%m-%d %H:%M:%S")
                            );
                        } else {
                            println!("{}", path.display());
                        }
                    } else {
                        println!("{}", path.display());
                    }
                }
            }
            Err(e) => {
                error_count += 1;
                if args.verbose {
                    eprintln!("ERROR: {e:?}");
                }
            }
        }
    }

    if args.verbose {
        eprintln!();
        eprintln!("Summary:");
        eprintln!("  Total images scanned: {total_count}");
        eprintln!("  Images with cats: {found_count}");
        if error_count > 0 {
            eprintln!("  Errors: {error_count}");
        }
    }

    Ok(())
}
