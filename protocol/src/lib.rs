#![no_std]

mod arduino_to_desktop;
mod desktop_to_arduino;

pub use arduino_to_desktop::Packet as ArduinoToDesktopPacket;
pub use desktop_to_arduino::{Move, Operation, Packet as DesktopToArduinoPacket};

use flat_enum::IntoFlat;

pub fn encode_desktop_to_arduino_packet(p: DesktopToArduinoPacket) -> u8 {
    p.into_flat() as u8
}
pub fn decode_desktop_to_arduino_packet(byte: u8) -> Option<DesktopToArduinoPacket> {
    desktop_to_arduino::_FlattenedPacket::try_from(byte)
        .ok()
        .map(DesktopToArduinoPacket::from_flat)
}

pub fn encode_arduino_to_desktop_packet(p: ArduinoToDesktopPacket) -> u8 {
    p.into_flat() as u8
}
pub fn decode_arduino_to_desktop_packet(byte: u8) -> Option<ArduinoToDesktopPacket> {
    arduino_to_desktop::_FlattenedPacket::try_from(byte)
        .ok()
        .map(ArduinoToDesktopPacket::from_flat)
}
