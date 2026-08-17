use flat_enum::{FlatTarget, flat, into_flat};

#[derive(FlatTarget, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    X,
    Y,
    Y_,
    D,
    D_,
}
#[derive(FlatTarget, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Reset,
    Relax,
}

#[into_flat(_FlattenedPacket)]
pub enum Packet {
    #[flatten]
    Move(Move),
    #[flatten]
    Operation(Operation),
}

#[flat(Packet)]
#[repr(u8)]
#[derive(num_enum::TryFromPrimitive)]
pub enum _FlattenedPacket {}
