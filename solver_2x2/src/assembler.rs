use crate::cube::Cube;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Color {
    White,
    Yellow,
    Red,
    Orange,
    Green,
    Blue,
}

impl Color {
    pub fn is_ud(self) -> bool {
        matches!(self, Self::White | Self::Yellow)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColoredCorner(pub [Color; 3]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Column {
    pub topleft: Color,
    pub topright: Color,
    pub botleft: Color,
    pub botright: Color,
}

impl ColoredCorner {
    pub const ALL: [Self; 8] = [
        Self([Color::White, Color::Red, Color::Green]),
        Self([Color::White, Color::Green, Color::Orange]),
        Self([Color::White, Color::Orange, Color::Blue]),
        Self([Color::White, Color::Blue, Color::Red]),
        //
        Self([Color::Yellow, Color::Green, Color::Red]),
        Self([Color::Yellow, Color::Orange, Color::Green]),
        Self([Color::Yellow, Color::Blue, Color::Orange]),
        Self([Color::Yellow, Color::Red, Color::Blue]),
    ];
    pub fn find_from_2_colors(a: Color, b: Color) -> Option<usize> {
        Self::ALL.iter().enumerate().find_map(|(i, corner)| {
            (corner.0[0] == a && corner.0[1] == b
                || corner.0[1] == a && corner.0[2] == b
                || corner.0[2] == a && corner.0[0] == b)
                .then_some(i)
        })
    }
}

/// indices follow top layer of cube in lib.rs
/// colored_vertical[i] are in positions cube index i and i+4
pub fn assemble_cube(columns: [Column; 4]) -> Option<Cube> {
    let mut cube = Cube::default();
    let mut found = [false; 8];
    for (col_idx, &col) in columns.iter().enumerate() {
        let top_id = ColoredCorner::find_from_2_colors(col.topright, col.topleft)?;
        let bot_id = ColoredCorner::find_from_2_colors(col.botleft, col.botright)?;
        found[top_id] = true;
        found[bot_id] = true;
        cube.0[col_idx + 0].set_id(top_id as _);
        cube.0[col_idx + 4].set_id(bot_id as _);

        if col.topleft.is_ud() {
            cube.0[col_idx + 0].set_ori(2);
        } else if col.topright.is_ud() {
            cube.0[col_idx + 0].set_ori(1);
        } else {
            cube.0[col_idx + 0].set_ori(0); // noop, but is this more clear?
        }

        if col.botleft.is_ud() {
            cube.0[col_idx + 4].set_ori(1);
        } else if col.botright.is_ud() {
            cube.0[col_idx + 4].set_ori(2);
        } else {
            cube.0[col_idx + 4].set_ori(0); // noop, but is this more clear?
        }
    }
    if !found.iter().all(|i| *i) {
        return None;
    }

    cube.is_twist_valid().then_some(cube)
}
