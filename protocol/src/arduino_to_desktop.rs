use flat_enum::{FlatTarget, flat, into_flat};

#[derive(FlatTarget, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AMove {
    X,
    Y,
    Y_,
    D,
    D_,
}

#[into_flat(_FlattenedPacket)]
pub enum Packet {
    ActionCompleted,
    ActionFailed,
    #[flatten]
    Move(AMove),
}

#[flat(Packet)]
#[repr(u8)]
#[derive(num_enum::TryFromPrimitive)]
pub enum _FlattenedPacket {}
