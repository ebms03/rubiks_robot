use esp_hal::{Config, delay::Delay, ledc::LowSpeed};
use esp_hal_servo::Servo;

// at least kinda, more than good enough
pub trait ServoInterpolation {
    fn set_angle_interpolate(&mut self, from: f32, to: f32, speed: f32, delay: &mut Delay);
}

impl<'a> ServoInterpolation for Servo<'a, LowSpeed> {
    fn set_angle_interpolate(&mut self, from: f32, to: f32, speed: f32, delay: &mut Delay) {
        let step = 0.5f32;
        let distance = to - from;
        let n_steps = (distance.abs() / step) as u32;
        let time = distance.abs() / speed;
        let time_per_step = (1e6 * time / n_steps as f32) as u32;
        for i in 0..n_steps {
            self.set_angle(from + distance * i as f32 / n_steps as f32);
            delay.delay_micros(time_per_step);
        }
        self.set_angle(to);
    }
}
