#![no_std]

#[derive(num_enum::TryFromPrimitive, Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum EspToDesktopPacket {
    Success,
    Failed,
}
#[derive(num_enum::TryFromPrimitive, Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum DesktopToEspPacket {
    Z,
    Y,
    Y_,
    D,
    D_,
    Relax,
    Shutdown,
}

pub fn encode_desktop_to_esp_packet(p: DesktopToEspPacket) -> u8 {
    p as u8
}
pub fn decode_desktop_to_esp_packet(byte: u8) -> Option<DesktopToEspPacket> {
    DesktopToEspPacket::try_from(byte).ok()
}

pub fn encode_esp_to_desktop_packet(p: EspToDesktopPacket) -> u8 {
    p as u8
}
pub fn decode_esp_to_desktop_packet(byte: u8) -> Option<EspToDesktopPacket> {
    EspToDesktopPacket::try_from(byte).ok()
}
