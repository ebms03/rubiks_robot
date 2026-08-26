#![no_std]
#![no_main]
use embedded_hal::digital::OutputPin;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::ledc::timer::LSClockSource;
use esp_hal::uart::{self, Uart};
use esp_hal_servo::{Servo, ServoConfig};

esp_bootloader_esp_idf::esp_app_desc!();
use esp_backtrace as _;
use esp_hal::{
    Config,
    ledc::{Ledc, LowSpeed, channel, timer},
};

use crate::movement::*;
use crate::stepper::*;

mod movement;
mod servo;
mod stepper;

#[esp_hal::main]
fn main() -> ! {
    let peripherals = esp_hal::init(Config::default());

    let mut uart = Uart::new(
        peripherals.UART0,
        uart::Config::default().with_baudrate(115_200),
    )
    .unwrap()
    .with_rx(peripherals.GPIO44)
    .with_tx(peripherals.GPIO43);

    let mut ledc = Ledc::new(peripherals.LEDC);
    let servo_config = ServoConfig::sg90(timer::config::Duty::Duty14Bit);
    let servo_timer = servo_config
        .configure_timer::<LowSpeed>(&mut ledc, timer::Number::Timer0, LSClockSource::APBClk)
        .unwrap();

    let twister_step = peripherals.GPIO7;
    let twister_dir = peripherals.GPIO6;
    let twister_slp = peripherals.GPIO15;

    let pusher_pin = peripherals.GPIO5;
    let flipper_pin = peripherals.GPIO4;

    let mut delay = Delay::new();
    let mut twister = A4988::new(
        Output::new(twister_step, Level::Low, OutputConfig::default()),
        Output::new(twister_dir, Level::Low, OutputConfig::default()),
        Output::new(twister_slp, Level::Low, OutputConfig::default()),
    );

    let mut pusher = Servo::new(
        "pusher",
        servo_config.clone(),
        &mut ledc,
        &servo_timer,
        channel::Number::Channel0,
        pusher_pin,
    )
    .unwrap();

    let mut flipper = Servo::new(
        "flipper",
        servo_config.clone(),
        &mut ledc,
        &servo_timer,
        channel::Number::Channel1,
        flipper_pin,
    )
    .unwrap();

    move_relax(&mut twister, &mut flipper, &mut pusher, &mut delay);
    twister.wake();

    loop {
        if uart.read_ready() {
            esp_println::println!("1");
            let mut byte = [0];
            uart.read_buffered(&mut byte).unwrap();
            let success =
                handle_command(byte[0], &mut twister, &mut flipper, &mut pusher, &mut delay);
            let packet = match success {
                CommandResult::Success => Some(protocol::EspToDesktopPacket::Success),
                CommandResult::Failed => Some(protocol::EspToDesktopPacket::Failed),
            };
            if let Some(packet) = packet {
                uart.write(&[protocol::encode_esp_to_desktop_packet(packet)])
                    .unwrap();
                uart.flush().unwrap();
            }
        }
    }
}

enum CommandResult {
    Success,
    Failed,
}

fn handle_command<S, D, E>(
    mut byte: u8,
    twister: &mut A4988<S, D, E>,
    flipper: &mut Servo<'_, LowSpeed>,
    pusher: &mut Servo<'_, LowSpeed>,
    delay: &mut Delay,
) -> CommandResult
where
    S: OutputPin,
    D: OutputPin,
    E: OutputPin,
{
    // send commands with 0-9 keys when testing
    if byte >= 48 {
        byte -= 48;
    }
    esp_println::println!("{byte:?}");
    let Some(p) = protocol::decode_desktop_to_esp_packet(byte) else {
        return CommandResult::Failed;
    };
    esp_println::println!("{p:?}");
    match p {
        protocol::DesktopToEspPacket::Z => move_z(twister, flipper, pusher, delay),
        protocol::DesktopToEspPacket::Y => move_y(twister, delay),
        protocol::DesktopToEspPacket::Y_ => move_y_(twister, delay),
        protocol::DesktopToEspPacket::D => move_d(twister, pusher, delay),
        protocol::DesktopToEspPacket::D_ => move_d_(twister, pusher, delay),
    }
    return CommandResult::Success;
}
