use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::{CameraIndex, RequestedFormat, RequestedFormatType};
use nokhwa::Camera;
use std::path::PathBuf;

pub async fn capture_frame() -> String {
    // 1. Setup path in projectroot/webframe_capture
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("webframe_capture");

    // Create directory if it doesn't exist
    if !path.exists() {
        let _ = std::fs::create_dir_all(&path);
    }

    path.push("current_scan.jpg");
    let path_str = path.to_string_lossy().to_string();

    // 2. Access the webcam (Index 0)
    let index = CameraIndex::Index(0);
    let requested =
        RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

    // Using a block to ensure camera is dropped (closed) as soon as we're done
    let result: Result<(), String> = (|| {
        let mut camera = Camera::new(index, requested).map_err(|e| e.to_string())?;
        camera.open_stream().map_err(|e| e.to_string())?;
        let frame = camera.frame().map_err(|e| e.to_string())?;
        let decoded = frame
            .decode_image::<RgbFormat>()
            .map_err(|e| e.to_string())?;

        decoded.save(&path).map_err(|e| e.to_string())?;
        Ok(())
    })();

    if let Err(e) = result {
        eprintln!("Webcam Capture Error: {}", e);
        return String::new(); // Return empty string on failure
    }

    path_str
}
