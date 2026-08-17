use solver_2x2::cube::{self, Axis, Direction, Layer, Rotation, Twist};

pub const FILENAME: &str = "solver_table.bin";
pub const CUBE_MOVES: &[cube::Move] = &[
    cube::Move::Rotation(Rotation(Axis::X, Direction::CW)),
    cube::Move::Rotation(Rotation(Axis::Y, Direction::CW)),
    cube::Move::Rotation(Rotation(Axis::Y, Direction::CCW)),
    cube::Move::Twist(Twist(Layer::Bottom, Direction::CCW)),
    cube::Move::Twist(Twist(Layer::Bottom, Direction::CCW)),
];
