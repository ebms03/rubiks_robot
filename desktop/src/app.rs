use std::{
    collections::VecDeque,
    sync::{
        Arc,
        atomic::{self, AtomicBool},
    },
    thread::JoinHandle,
};

use crossbeam_channel::{Receiver, Sender, bounded, unbounded};
use eframe::egui;
use opencv::{core, imgproc, prelude::*};
use protocol::DesktopToArduinoPacket;
use strum::IntoEnumIterator;

use crate::{
    calibration::{CalibrationData, ColorClass, Location},
    camera::{self, CameraConfig},
    robot::{self, RobotCommand},
};

const CALIBRATION_PATH: &str = "calibration.json";
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OptionalCalibratable {
    Color(ColorClass),
    Location(Location),
    None,
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Move(protocol::Move),
    CaptureColors,
}

impl Action {
    const CAPTURE_FACES: &[Self] = &[
        Self::CaptureColors,
        Self::Move(protocol::Move::Y),
        Self::CaptureColors,
        Self::Move(protocol::Move::Y),
        Self::CaptureColors,
        Self::Move(protocol::Move::Y),
        Self::CaptureColors,
        Self::Move(protocol::Move::Y),
    ];
}

pub struct App {
    display_rx: Receiver<core::Mat>,
    texture: Option<egui::TextureHandle>,
    img_size: egui::Vec2,

    robot_command_tx: Sender<RobotCommand>,
    robot_is_busy: Arc<AtomicBool>,

    data: CalibrationData,
    currently_calibrating: OptionalCalibratable,
    latest_frame: core::Mat,

    action_queque: VecDeque<Action>,
    cube_faces: Vec<[ColorClass; 4]>,
}

fn read_calibration_data() -> Option<CalibrationData> {
    let data = std::fs::read(CALIBRATION_PATH).ok()?;
    serde_json::from_slice(&data)
        .inspect_err(|e| log::error!("Failed to load calibration data: {e:?}"))
        .ok()
}
fn save_calibration_data(map: &CalibrationData) {
    let data = serde_json::to_string(map).unwrap();
    match std::fs::write(CALIBRATION_PATH, data) {
        Ok(_) => {}
        Err(e) => log::error!("Failed to save calibration data: {e:?}"),
    }
}

impl App {
    pub fn new(camera_config: CameraConfig) -> Self {
        let robot_is_busy = Arc::new(AtomicBool::new(false));
        let (display_tx, display_rx) = bounded::<core::Mat>(1);
        let (robot_command_tx, robot_command_rx) = unbounded::<RobotCommand>();

        let _cam_worker = camera::camera_worker(camera_config, display_tx);
        let _robot_worker = robot::robot_worker(robot_command_rx, robot_is_busy.clone());

        Self {
            display_rx,
            texture: None,
            img_size: egui::Vec2::ZERO,
            robot_command_tx,
            data: read_calibration_data().unwrap_or_default(),
            currently_calibrating: OptionalCalibratable::None,
            latest_frame: Mat::default(),
            action_queque: Default::default(),
            cube_faces: Default::default(),
            robot_is_busy,
        }
    }

    fn upload_frame(&mut self, ctx: &egui::Context, mut frame: core::Mat) {
        for loc in Location::iter() {
            imgproc::circle(
                &mut frame,
                self.data.get_location(loc),
                10,
                core::Scalar::from_array([0.0, 0.0, 255.0, 0.0]),
                3,
                imgproc::LINE_8,
                0,
            )
            .unwrap();
        }

        let mut rgba = core::Mat::default();
        if imgproc::cvt_color(&frame, &mut rgba, imgproc::COLOR_BGR2RGBA, 4).is_err() {
            return;
        }

        let size = match rgba.size() {
            Ok(s) => s,
            Err(_) => return,
        };
        let bytes: &[u8] = match rgba.data_bytes() {
            Ok(b) => b,
            Err(_) => return,
        };

        let color_image = egui::ColorImage::from_rgba_unmultiplied(
            [size.width as usize, size.height as usize],
            bytes,
        );

        match &mut self.texture {
            Some(tex) => tex.set(color_image, egui::TextureOptions::LINEAR),
            None => {
                self.img_size = egui::vec2(size.width as f32, size.height as f32);
                self.texture =
                    Some(ctx.load_texture("video", color_image, egui::TextureOptions::LINEAR));
            }
        }
        self.latest_frame = frame;
        ctx.request_repaint();
    }
}

impl eframe::App for App {
    fn on_exit(&mut self) {
        match self.robot_command_tx.send(RobotCommand::Shutdown) {
            Ok(_) => (),
            Err(e) => log::error!("Error during shutdown: {e:?}"),
        }
    }
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.robot_is_busy.load(atomic::Ordering::SeqCst)
            && let Some(action) = self.action_queque.pop_front()
        {
            match action {
                Action::Move(m) => {
                    self.robot_is_busy.store(true, atomic::Ordering::SeqCst);
                    match self
                        .robot_command_tx
                        .send(protocol::DesktopToArduinoPacket::Move(m).into())
                    {
                        Ok(_) => {}
                        Err(e) => log::error!("{e:?}"),
                    }
                }
                Action::CaptureColors => {
                    let locs = [
                        self.data.get_location(Location::Topleft),
                        self.data.get_location(Location::Botleft),
                        self.data.get_location(Location::Topright),
                        self.data.get_location(Location::Botright),
                    ];
                    self.cube_faces.push(locs.map(|l| {
                        let color = *self.latest_frame.at_2d(l.y, l.x).unwrap();
                        self.data.lookup_closest_color(color)
                    }));
                }
            }
        }
    }
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx();

        if let Some(f) = self.display_rx.try_iter().last() {
            self.upload_frame(ctx, f);
        } else if self.texture.is_some() {
            ctx.request_repaint();
        }

        egui::Panel::left("calibration").show(ui, |ui| {
            ui.heading("Calibration");
            ui.separator();

            for class in ColorClass::iter() {
                ui.horizontal(|ui| {
                    let [b, g, r] = self.data.get_color(class).0;
                    let (rect, _) =
                        ui.allocate_exact_size(egui::vec2(16.0, 16.0), egui::Sense::hover());
                    ui.painter()
                        .rect_filled(rect, 0.0, egui::Color32::from_rgb(r, g, b));
                    ui.label(format!("{class:?}"));

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if self.currently_calibrating == OptionalCalibratable::Color(class) {
                            if ui.button("[Cancel]").clicked() {
                                self.currently_calibrating = OptionalCalibratable::None;
                            }
                        } else {
                            if ui.button("Calibrate").clicked() {
                                self.currently_calibrating = OptionalCalibratable::Color(class);
                            }
                        };
                    });
                });
            }
            ui.separator();
            for loc in Location::iter() {
                ui.horizontal(|ui| {
                    let [x, y] = self.data.get_location(loc).to_vec2().0;
                    ui.monospace(format!("{loc:?}\n{x:>4},{y:>4}"));

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if self.currently_calibrating == OptionalCalibratable::Location(loc) {
                            if ui.button("[Cancel]").clicked() {
                                self.currently_calibrating = OptionalCalibratable::None;
                            }
                        } else {
                            if ui.button("Calibrate").clicked() {
                                self.currently_calibrating = OptionalCalibratable::Location(loc);
                            }
                        };
                    });
                });
            }
            ui.separator();
            if ui.button("Relax").clicked() {
                self.action_queque.clear();
                self.robot_is_busy.store(false, atomic::Ordering::SeqCst);
                match self
                    .robot_command_tx
                    .send(DesktopToArduinoPacket::Operation(protocol::Operation::Relax).into())
                {
                    Ok(_) => {}
                    Err(e) => log::error!("{e:?}"),
                }
            }
        });

        egui::CentralPanel::default().show(ui, |ui| {
            if let Some(tex) = &self.texture {
                let avail = ui.available_size();
                let scale = (avail.x / self.img_size.x).min(avail.y / self.img_size.y);
                let display_size = self.img_size * scale;

                let img = egui::Image::from_texture(tex)
                    .fit_to_exact_size(display_size)
                    .sense(egui::Sense::click());

                let resp = ui.add(img);

                if resp.clicked() {
                    if let Some(pos) = resp.interact_pointer_pos() {
                        let img_px = (pos - resp.rect.min) / scale;
                        let click_pos = core::Point::new(img_px.x as i32, img_px.y as i32);

                        match self.currently_calibrating {
                            OptionalCalibratable::Color(class) => {
                                let color = *self
                                    .latest_frame
                                    .at_2d::<core::Vec3b>(click_pos.y, click_pos.x)
                                    .unwrap();
                                self.data.set_color(class, color);
                                save_calibration_data(&self.data);
                                self.currently_calibrating = OptionalCalibratable::None;
                            }
                            OptionalCalibratable::Location(loc) => {
                                self.data.set_location(loc, click_pos);
                                save_calibration_data(&self.data);
                                self.currently_calibrating = OptionalCalibratable::None;
                            }
                            OptionalCalibratable::None => {}
                        }
                    }
                }
            } else {
                ui.label("Waiting for video…");
            }
        });
        ui.separator();
        if ui.button("CAPTURE COLORS").clicked() && self.action_queque.is_empty() {
            self.cube_faces.clear();
            self.action_queque
                .extend(Action::CAPTURE_FACES.iter().copied());
        }
    }
}
