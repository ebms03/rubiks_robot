use protocol::DesktopToEspPacket;
use solver_2x2::cube;

pub const FILENAME: &str = "solver_table.bin";
pub const CUBE_MOVES: &[cube::Move] = &[
    cube::Move::Z,
    cube::Move::Y,
    cube::Move::Y_,
    cube::Move::D,
    cube::Move::D_,
];
pub fn map_to_packet(m: cube::Move) -> Option<DesktopToEspPacket> {
    match m {
        cube::Move::Z => Some(DesktopToEspPacket::Z),
        cube::Move::Y => Some(DesktopToEspPacket::Y),
        cube::Move::Y_ => Some(DesktopToEspPacket::Y_),
        cube::Move::D => Some(DesktopToEspPacket::D),
        cube::Move::D_ => Some(DesktopToEspPacket::D_),
        _ => None,
    }
}
