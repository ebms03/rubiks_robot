use clap::Parser;

mod app;
mod calibration;
mod camera;
mod robot;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    #[arg(long, num_args = 0..=1, default_missing_value = "0")]
    webcam: Option<i32>,

    #[arg(long, num_args = 0..=1, default_missing_value = camera::DEFAULT_STREAM)]
    url: Option<String>,
}

impl Args {
    fn camera_config(&self) -> camera::CameraConfig {
        if let Some(url) = &self.url {
            camera::CameraConfig::Url(url.to_string())
        } else if let Some(idx) = self.webcam {
            camera::CameraConfig::Webcam(idx)
        } else {
            camera::CameraConfig::Webcam(0)
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().filter_or("RUST_LOG", "info")) 
        .filter_module("zbus", log::LevelFilter::Off)
        .filter_module("wgpu_hal", log::LevelFilter::Off)
        .filter_module("wgpu_core", log::LevelFilter::Off)
        .filter_module("egui_wgpu", log::LevelFilter::Off)
        .filter_module("tracing", log::LevelFilter::Off)
        .filter_module("sctk_adwaita", log::LevelFilter::Off)
        .init();

    let camera_config = Args::parse().camera_config();

    eframe::run_native(
        "title",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(app::App::new(camera_config)))),
    )?;

    Ok(())
}
