use std::{
    collections::{HashSet, VecDeque},
    sync::OnceLock,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Cube(pub(crate) [Corner; 8]);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Corner(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis {
    X,
    Y,
    Z,
}
impl Axis {
    pub const ALL: [Self; 3] = [Self::X, Self::Y, Self::Z];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Layer {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}

impl Layer {
    pub const ALL: [Self; 6] = [
        Self::Top,
        Self::Bottom,
        Self::Left,
        Self::Right,
        Self::Front,
        Self::Back,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    CW,
    CCW,
}
impl Direction {
    pub const ALL: [Self; 2] = [Self::CW, Self::CCW];
    pub fn inverse(self) -> Self {
        match self {
            Direction::CW => Self::CCW,
            Direction::CCW => Self::CW,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Move {
    Twist(Twist),
    Rotation(Rotation),
}
impl Move {
    pub const ALL: [Self; 18] = [
        Self::Twist(Twist::ALL[0]),
        Self::Twist(Twist::ALL[1]),
        Self::Twist(Twist::ALL[2]),
        Self::Twist(Twist::ALL[3]),
        Self::Twist(Twist::ALL[4]),
        Self::Twist(Twist::ALL[5]),
        Self::Twist(Twist::ALL[6]),
        Self::Twist(Twist::ALL[7]),
        Self::Twist(Twist::ALL[8]),
        Self::Twist(Twist::ALL[9]),
        Self::Twist(Twist::ALL[10]),
        Self::Twist(Twist::ALL[11]),
        Self::Rotation(Rotation::ALL[0]),
        Self::Rotation(Rotation::ALL[1]),
        Self::Rotation(Rotation::ALL[2]),
        Self::Rotation(Rotation::ALL[3]),
        Self::Rotation(Rotation::ALL[4]),
        Self::Rotation(Rotation::ALL[5]),
    ];

    pub const U: Self = Self::Twist(Twist(Layer::Top, Direction::CW));
    pub const U_: Self = Self::Twist(Twist(Layer::Top, Direction::CCW));
    pub const D: Self = Self::Twist(Twist(Layer::Bottom, Direction::CW));
    pub const D_: Self = Self::Twist(Twist(Layer::Bottom, Direction::CCW));
    pub const F: Self = Self::Twist(Twist(Layer::Front, Direction::CW));
    pub const F_: Self = Self::Twist(Twist(Layer::Front, Direction::CCW));
    pub const B: Self = Self::Twist(Twist(Layer::Back, Direction::CW));
    pub const B_: Self = Self::Twist(Twist(Layer::Back, Direction::CCW));
    pub const L: Self = Self::Twist(Twist(Layer::Left, Direction::CW));
    pub const L_: Self = Self::Twist(Twist(Layer::Left, Direction::CCW));
    pub const R: Self = Self::Twist(Twist(Layer::Right, Direction::CW));
    pub const R__: Self = Self::Twist(Twist(Layer::Right, Direction::CCW));

    pub const X: Self = Self::Rotation(Rotation(Axis::X, Direction::CW));
    pub const X_: Self = Self::Rotation(Rotation(Axis::X, Direction::CCW));
    pub const Y: Self = Self::Rotation(Rotation(Axis::Y, Direction::CW));
    pub const Y_: Self = Self::Rotation(Rotation(Axis::Y, Direction::CCW));
    pub const Z: Self = Self::Rotation(Rotation(Axis::Z, Direction::CW));
    pub const Z_: Self = Self::Rotation(Rotation(Axis::Z, Direction::CCW));

    pub fn dir(self) -> Direction {
        match self {
            Move::Twist(twist) => twist.1,
            Move::Rotation(rotation) => rotation.1,
        }
    }

    pub fn inverse(self) -> Move {
        match self {
            Move::Twist(Twist(layer, dir)) => Self::Twist(Twist(layer, dir.inverse())),
            Move::Rotation(Rotation(axis, dir)) => Self::Rotation(Rotation(axis, dir.inverse())),
        }
    }

    pub fn pack(self) -> u8 {
        let dir_bit = ((self.dir() == Direction::CW) as u8) << 6;
        let type_bit;
        let axis_or_layer;
        match self {
            Move::Twist(twist) => {
                type_bit = 1 << 7;
                axis_or_layer = twist.0 as u8;
            }
            Move::Rotation(rotation) => {
                type_bit = 0 << 7;
                axis_or_layer = rotation.0 as u8;
            }
        }
        type_bit | dir_bit | axis_or_layer
    }
    pub fn unpack(packed: u8) -> Option<Move> {
        let type_bit = packed >> 7;
        let dir_bit = (packed >> 6) & 1;
        let axis_or_layer = packed & 0b0011_1111;
        let dir = if dir_bit == 1 {
            Direction::CW
        } else {
            Direction::CCW
        };

        Some(if type_bit == 1 {
            Move::Twist(Twist(*Layer::ALL.get(axis_or_layer as usize)?, dir))
        } else {
            Move::Rotation(Rotation(*Axis::ALL.get(axis_or_layer as usize)?, dir))
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Twist(pub Layer, pub Direction);
impl Twist {
    pub const ALL: [Self; 12] = [
        Twist(Layer::ALL[0], Direction::CW),
        Twist(Layer::ALL[0], Direction::CCW),
        Twist(Layer::ALL[1], Direction::CW),
        Twist(Layer::ALL[1], Direction::CCW),
        Twist(Layer::ALL[2], Direction::CW),
        Twist(Layer::ALL[2], Direction::CCW),
        Twist(Layer::ALL[3], Direction::CW),
        Twist(Layer::ALL[3], Direction::CCW),
        Twist(Layer::ALL[4], Direction::CW),
        Twist(Layer::ALL[4], Direction::CCW),
        Twist(Layer::ALL[5], Direction::CW),
        Twist(Layer::ALL[5], Direction::CCW),
    ];
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rotation(pub Axis, pub Direction);

impl Rotation {
    pub const ALL: [Self; 6] = [
        Rotation(Axis::ALL[0], Direction::ALL[0]),
        Rotation(Axis::ALL[0], Direction::ALL[1]),
        Rotation(Axis::ALL[1], Direction::ALL[0]),
        Rotation(Axis::ALL[1], Direction::ALL[1]),
        Rotation(Axis::ALL[2], Direction::ALL[0]),
        Rotation(Axis::ALL[2], Direction::ALL[1]),
    ];
}

impl Corner {
    pub const fn new_with_id(id: u8) -> Self {
        let mut corner = Self(0);
        corner.set_id(id);
        corner
    }
    pub const fn new_with_id_ori(id: u8, ori: u8) -> Self {
        let mut corner = Self(0);
        corner.set_id(id);
        corner.set_ori(ori);
        corner
    }
    pub const fn get_id(self) -> u8 {
        self.0 >> 4
    }
    pub const fn get_ori(self) -> u8 {
        self.0 & 0b1111
    }
    pub const fn set_id(&mut self, id: u8) {
        assert!(id < 8);
        self.0 &= 0b1111;
        self.0 |= id << 4;
    }
    pub const fn set_ori(&mut self, ori: u8) {
        assert!(ori < 3);
        self.0 &= 0b1111_0000;
        self.0 |= ori;
    }
}

impl Default for Cube {
    fn default() -> Self {
        Self::SOLVED
    }
}

impl Cube {
    pub const SOLVED: Self = Self([
        Corner::new_with_id(0),
        Corner::new_with_id(1),
        Corner::new_with_id(2),
        Corner::new_with_id(3),
        Corner::new_with_id(4),
        Corner::new_with_id(5),
        Corner::new_with_id(6),
        Corner::new_with_id(7),
    ]);
    pub fn applied(self, m: Move) -> Self {
        match m {
            Move::Twist(Twist(layer, dir)) => self._apply_twist(layer, dir),
            Move::Rotation(Rotation(axis, dir)) => self._apply_rotation(axis, dir),
        }
    }

    fn _apply_twist(mut self, layer: Layer, dir: Direction) -> Self {
        let (indices, delta) = Self::face_table(layer);
        let mut corners = indices.map(|i| self.0[i]);
        match dir {
            Direction::CW => corners.rotate_left(1),
            Direction::CCW => corners.rotate_right(1),
        }
        for (k, &i) in indices.iter().enumerate() {
            let mut c = corners[k];
            c.set_ori((c.get_ori() + delta[k]) % 3);
            self.0[i] = c;
        }
        self
    }

    fn _apply_rotation(self, axis: Axis, dir: Direction) -> Self {
        let layers = match axis {
            Axis::X => [Layer::Right, Layer::Left],
            Axis::Y => [Layer::Top, Layer::Bottom],
            Axis::Z => [Layer::Front, Layer::Back],
        };
        self._apply_twist(layers[0], dir)
            ._apply_twist(layers[1], dir.inverse())
    }

    pub fn face_table(layer: Layer) -> ([usize; 4], [u8; 4]) {
        match layer {
            Layer::Top => ([0, 3, 2, 1], [0, 0, 0, 0]),
            Layer::Bottom => ([4, 5, 6, 7], [0, 0, 0, 0]),
            Layer::Front => ([2, 3, 7, 6], [1, 2, 1, 2]),
            Layer::Back => ([1, 5, 4, 0], [2, 1, 2, 1]),
            Layer::Right => ([1, 2, 6, 5], [1, 2, 1, 2]),
            Layer::Left => ([0, 4, 7, 3], [2, 1, 2, 1]),
        }
    }

    pub fn ids(&self) -> [u8; 8] {
        self.0.map(Corner::get_id)
    }
    pub fn orientations(&self) -> [u8; 8] {
        self.0.map(Corner::get_ori)
    }
    pub fn twist_sum(&self) -> u8 {
        self.orientations().iter().fold(0u8, |a, &b| (a + b) % 3)
    }
    pub fn is_twist_valid(&self) -> bool {
        self.twist_sum() == 0
    }

    /// The 24 whole-cube rotations of `SOLVED`. Two cubes are both "solved"
    /// iff one is in this set.
    pub fn solved_class() -> &'static [Cube; 24] {
        static CLASS: OnceLock<[Cube; 24]> = OnceLock::new();
        CLASS.get_or_init(|| {
            let mut all = [Cube::SOLVED; 24];
            let mut seen: HashSet<Cube> = HashSet::with_capacity(64);
            let mut queue: VecDeque<Cube> = VecDeque::with_capacity(64);
            seen.insert(Cube::SOLVED);
            queue.push_back(Cube::SOLVED);
            let mut i = 0;
            while let Some(c) = queue.pop_front() {
                all[i] = c;
                i += 1;
                for &r in Rotation::ALL.iter() {
                    let n = c.applied(Move::Rotation(r));
                    if seen.insert(n) {
                        queue.push_back(n);
                    }
                }
            }
            assert_eq!(i, 24, "rotation group of the cube has order 24");
            all
        })
    }

    pub fn is_solved(&self) -> bool {
        Self::solved_class().contains(self)
    }
    pub const FACTORIALS: [u32; 8] = [5040, 720, 120, 24, 6, 2, 1, 1];
    pub const ORI_SPACE: u32 = 2187;
    pub const STATE_SPACE: u32 = 88_179_840;

    /// Dense index in [0, 88_179_840). Requires `is_twist_valid()`.
    pub fn dense_index(&self) -> u32 {
        // --- Lehmer code ---
        let ids = self.ids();
        let mut perm_idx = 0u32;
        for i in 0..8 {
            let mut smaller = 0u8;
            for j in (i + 1)..8 {
                if ids[j] < ids[i] {
                    smaller += 1;
                }
            }
            perm_idx += (smaller as u32) * Self::FACTORIALS[i];
        }

        // --- 7 orientations in base 3 ---
        let oris = self.orientations();
        let mut ori_idx = 0u32;
        for i in 0..7 {
            ori_idx = ori_idx * 3 + oris[i] as u32;
        }

        perm_idx * Self::ORI_SPACE + ori_idx
    }

    /// Inverse of `dense_index`. No allocation.
    pub fn from_dense_index(idx: u32) -> Self {
        assert!(idx < Self::STATE_SPACE);

        let ori_idx = idx % Self::ORI_SPACE;
        let perm_idx = idx / Self::ORI_SPACE;

        // --- Decode Lehmer → permutation ---
        let mut digits = [0u8; 8];
        let mut p = perm_idx;
        for i in 0..8 {
            digits[i] = (p / Self::FACTORIALS[i]) as u8;
            p %= Self::FACTORIALS[i];
        }
        let mut available = [0u8, 1, 2, 3, 4, 5, 6, 7];
        let mut len = 8;
        let mut ids = [0u8; 8];
        for i in 0..8 {
            let d = digits[i] as usize;
            ids[i] = available[d];
            for j in d..(len - 1) {
                available[j] = available[j + 1];
            }
            len -= 1;
        }

        // --- Decode 7 orientations, derive the 8th ---
        let mut oris = [0u8; 8];
        let mut o = ori_idx;
        for i in (0..7).rev() {
            oris[i] = (o % 3) as u8;
            o /= 3;
        }
        let sum: u8 = oris[..7].iter().fold(0u8, |a, &b| (a + b) % 3);
        oris[7] = (3 - sum) % 3;

        let mut state = [Corner(0); 8];
        for i in 0..8 {
            state[i] = Corner::new_with_id_ori(ids[i], oris[i]);
        }
        Self(state)
    }
}
