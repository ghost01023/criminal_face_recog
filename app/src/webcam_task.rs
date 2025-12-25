use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::{CameraIndex, RequestedFormat, RequestedFormatType};
use nokhwa::Camera;
use std::path::PathBuf;

pub async fn capture_frame() -> String {
    // 1. Setup path in your local temp folder
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("temp");
    if !path.exists() {
        let _ = std::fs::create_dir(&path);
    }
    path.push("current_scan.jpg");
    let path_str = path.to_string_lossy().to_string();

    // 2. Access the webcam
    let index = CameraIndex::Index(0);
    let requested =
        RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

    // We open the camera, grab one frame, and shut it down to release the device
    if let Ok(mut camera) = Camera::new(index, requested) {
        if camera.open_stream().is_ok() {
            if let Ok(frame) = camera.frame() {
                let decoded = frame.decode_image::<RgbFormat>().unwrap();
                // Save the frame as JPG
                let _ = decoded.save(&path);
            }
        }
    }

    path_str
}
