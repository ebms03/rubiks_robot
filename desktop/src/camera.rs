use crossbeam_channel::Sender;
use opencv::{core, prelude::*, videoio};
use std::thread;


pub const DEFAULT_STREAM: &'static str = "https://10.0.0.7:8080/video";
#[derive(Debug)]
pub enum CameraConfig {
    Webcam(i32),
    Url(String),
}

pub fn camera_worker(cfg: CameraConfig, frame_tx: Sender<core::Mat>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut cam = match cfg {
            CameraConfig::Webcam(idx) => videoio::VideoCapture::new(idx, videoio::CAP_ANY).unwrap(),
            CameraConfig::Url(url) => {
                videoio::VideoCapture::from_file(&url, videoio::CAP_ANY).unwrap()
            }
        };
        if !videoio::VideoCapture::is_opened(&cam).unwrap() {
            log::error!("Video capture not opened");
            return;
        }

        let mut frame = core::Mat::default();
        while cam.read(&mut frame).is_ok() {
            if frame.size().map(|s| s.width > 0).unwrap_or(false) {
                if frame_tx.send(frame.clone()).is_err() {
                    break;
                }
            }
        }
    })
}
