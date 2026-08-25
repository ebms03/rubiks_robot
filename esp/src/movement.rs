use esp_hal::{delay::Delay, ledc::LowSpeed};
use esp_hal_servo::Servo;

use crate::{servo::ServoInterpolation, stepper::Stepper};

pub const PUSHER_HOLD: f32 = 150.0;
pub const PUSHER_RELAX: f32 = 90.0;
pub const FLIPPER_FLIP: f32 = 140.0;
pub const FLIPPER_BLOCK: f32 = 20.0;
pub const FLIPPER_RELAX: f32 = 40.0;

const TWISTER_START_INTERVAL_MICROS: u32 = 2000; // slow starting interval
const TWISTER_CRUISE_INTERVAL_MICROS: u32 = 1000; // your TWISTER_STEP_INTERVAL_MICROS
const TWISTER_ACCEL_STEPS: u32 = 200;

pub const TWISTER_DIR_Y: bool = false;
pub const TWISTER_DIR_Y_: bool = true;
pub const TWISTER_DIR_D: bool = !TWISTER_DIR_Y;
pub const TWISTER_DIR_D_: bool = !TWISTER_DIR_Y_;
pub const TWISTER_90_DEG_STEPS: u32 = 50 * 8;
pub const PUSHER_SPEED: f32 = 400.0;
pub const FLIPPER_SPEED: f32 = 400.0;

pub fn move_y(twister: &mut impl Stepper, delay: &mut Delay) {
    twister_twist(twister, delay, TWISTER_DIR_Y);
}
pub fn move_y_(twister: &mut impl Stepper, delay: &mut Delay) {
    twister_twist(twister, delay, TWISTER_DIR_Y_);
}
pub fn move_d(twister: &mut impl Stepper, pusher: &mut Servo<'_, LowSpeed>, delay: &mut Delay) {
    pusher_push(pusher, delay);
    delay.delay_millis(500);
    twister_twist(twister, delay, TWISTER_DIR_D);
    pusher_relax(pusher, delay);
}
pub fn move_d_(twister: &mut impl Stepper, pusher: &mut Servo<'_, LowSpeed>, delay: &mut Delay) {
    pusher_push(pusher, delay);
    delay.delay_millis(500);
    twister_twist(twister, delay, TWISTER_DIR_D_);
    pusher_relax(pusher, delay);
}

pub fn move_z(
    twister: &mut impl Stepper,
    flipper: &mut Servo<'_, LowSpeed>,
    pusher: &mut Servo<'_, LowSpeed>,
    delay: &mut Delay,
) {
    flipper_flip_and_block(flipper, delay);
    delay.delay_millis(500);
    pusher_push(pusher, delay);
    delay.delay_millis(200);
    flipper_relax(flipper, delay);
    pusher_relax(pusher, delay);
}

pub fn move_relax(
    twister: &mut impl Stepper,
    flipper: &mut Servo<'_, LowSpeed>,
    pusher: &mut Servo<'_, LowSpeed>,
    delay: &mut Delay,
) {
    flipper_relax(flipper, delay);
    pusher_relax(pusher, delay);
}

pub fn twister_twist(twister: &mut impl Stepper, delay: &mut Delay, dir: bool) {
    twister.set_direction(dir);
    delay.delay_micros(10);
    run_twister_90(twister, delay);
    delay.delay_millis(100);
    // for _ in 0..TWISTER_90_DEG_STEPS {
    //     twister.step(delay);
    //     delay.delay_micros(TWISTER_STEP_INTERVAL_MICROS);
    // }
}

pub fn pusher_push(pusher: &mut Servo<'_, LowSpeed>, delay: &mut Delay) {
    pusher.set_angle_interpolate(PUSHER_RELAX, PUSHER_HOLD, PUSHER_SPEED, delay);
}

pub fn pusher_relax(pusher: &mut Servo<'_, LowSpeed>, delay: &mut Delay) {
    pusher.set_angle(PUSHER_RELAX);
}

pub fn flipper_flip_and_block(flipper: &mut Servo<'_, LowSpeed>, delay: &mut Delay) {
    flipper_flip(flipper, delay);
    delay.delay_millis(100);
    flipper.set_angle(FLIPPER_BLOCK);
}

pub fn flipper_flip(flipper: &mut Servo<'_, LowSpeed>, delay: &mut Delay) {
    flipper.set_angle_interpolate(FLIPPER_RELAX, FLIPPER_FLIP, FLIPPER_SPEED, delay);
}

pub fn flipper_relax(flipper: &mut Servo<'_, LowSpeed>, delay: &mut Delay) {
    flipper.set_angle(FLIPPER_RELAX);
}

fn run_twister_90(twister: &mut impl Stepper, delay: &mut Delay) {
    // Decide between trapezoidal and triangular profile
    let (accel, cruise, decel) =
        if TWISTER_90_DEG_STEPS >= TWISTER_ACCEL_STEPS + TWISTER_ACCEL_STEPS {
            // Enough steps for a full trapezoid
            (
                TWISTER_ACCEL_STEPS,
                TWISTER_90_DEG_STEPS - TWISTER_ACCEL_STEPS - TWISTER_ACCEL_STEPS,
                TWISTER_ACCEL_STEPS,
            )
        } else {
            // Short move: triangular (accel to midpoint, then decel)
            let half = TWISTER_90_DEG_STEPS / 2;
            (half, 0, TWISTER_90_DEG_STEPS - half)
        };

    for step in 0..TWISTER_90_DEG_STEPS {
        let interval = if step < accel {
            // --- Acceleration: shrink interval from START → CRUISE ---
            lerp_u32(
                TWISTER_START_INTERVAL_MICROS,
                TWISTER_CRUISE_INTERVAL_MICROS,
                step,
                accel,
            )
        } else if step < accel + cruise {
            // --- Cruise at full speed ---
            TWISTER_CRUISE_INTERVAL_MICROS
        } else {
            // --- Deceleration: grow interval from CRUISE → START ---
            let d = step - accel - cruise;
            lerp_u32(
                TWISTER_CRUISE_INTERVAL_MICROS,
                TWISTER_START_INTERVAL_MICROS,
                d,
                decel,
            )
        };

        twister.step(delay);
        delay.delay_micros(interval);
    }
}

/// Linear interpolation with integer math.
/// Returns `start` when `i == 0`, approaches `end` as `i → total`.
fn lerp_u32(start: u32, end: u32, i: u32, total: u32) -> u32 {
    if total == 0 {
        return end;
    }
    if end >= start {
        start + (end - start) * i / total
    } else {
        start - (start - end) * i / total
    }
}
