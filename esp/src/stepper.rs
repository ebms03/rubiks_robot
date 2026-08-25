use embedded_hal::digital::{OutputPin, PinState};
use esp_hal::delay::Delay;

pub trait Stepper {
    fn wake(&mut self);
    fn sleep(&mut self);
    fn set_direction(&mut self, forward: bool);
    fn step(&mut self, delay: &mut Delay);
}

pub struct A4988<STEP, DIR, EN> {
    step: STEP,
    dir: DIR,
    slp: EN,
}

impl<STEP, DIR, EN> A4988<STEP, DIR, EN>
where
    STEP: OutputPin,
    DIR: OutputPin,
    EN: OutputPin,
{
    pub fn new(step: STEP, dir: DIR, slp: EN) -> Self {
        Self { step, dir, slp }
    }
}

impl<STEP, DIR, EN> Stepper for A4988<STEP, DIR, EN>
where
    STEP: OutputPin,
    DIR: OutputPin,
    EN: OutputPin,
{
    fn wake(&mut self) {
        let _ = self.slp.set_high();
    }

    fn sleep(&mut self) {
        let _ = self.slp.set_low();
    }

    fn set_direction(&mut self, forward: bool) {
        let _ = self.dir.set_state(PinState::from(forward));
    }

    fn step(&mut self, delay: &mut Delay) {
        let _ = self.step.set_high().unwrap();
        delay.delay_micros(10);
        let _ = self.step.set_low().unwrap();
    }
}
