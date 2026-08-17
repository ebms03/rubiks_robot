use crate::assembler::*;
use crate::cube::*;

fn cube_to_columns(cube: &Cube) -> [Column; 4] {
    let placeholder = Column {
        topleft: Color::White,
        topright: Color::White,
        botleft: Color::White,
        botright: Color::White,
    };
    let mut columns = [placeholder; 4];

    for col_idx in 0..4 {
        let top = cube.0[col_idx];
        let bot = cube.0[col_idx + 4];
        let top_piece = &ColoredCorner::ALL[top.get_id() as usize];
        let bot_piece = &ColoredCorner::ALL[bot.get_id() as usize];
        let to = top.get_ori() as usize;
        let bo = bot.get_ori() as usize;

        // ori = k  ⟺  piece's c₀ (U/D sticker) is on the position's cₖ face.
        // Cyclic mapping: piece c_i  →  position c_{(i+k) mod 3}.
        // Inverting:  position c_j  shows  piece c_{(j−k) mod 3}.
        //
        //   Top corner:    topright = pos c₁,  topleft  = pos c₂
        //   Bottom corner: botleft  = pos c₁,  botright = pos c₂
        //
        // Rust's % can be negative, so we use (j + 3 − k) % 3.
        columns[col_idx] = Column {
            topright: top_piece.0[(4 - to) % 3],
            topleft: top_piece.0[(5 - to) % 3],
            botleft: bot_piece.0[(4 - bo) % 3],
            botright: bot_piece.0[(5 - bo) % 3],
        };
    }
    columns
}

/// Verify that a cube survives the column round-trip.
fn assert_roundtrip(cube: Cube) {
    let cols = cube_to_columns(&cube);
    let reassembled = assemble_cube(cols).expect("roundtrip should succeed");
    assert_eq!(reassembled, cube);
}

/// Build a cube by applying a sequence of moves to the solved state.
fn cube_from_moves(moves: &[Move]) -> Cube {
    let mut cube = Cube::default();
    for &m in moves {
        cube = cube.applied(m);
    }
    cube
}

// Move constants — keeps multi-move tests readable.
const U_CW: Move = Move::Twist(Twist(Layer::Top, Direction::CW));
const U_CCW: Move = Move::Twist(Twist(Layer::Top, Direction::CCW));
const D_CW: Move = Move::Twist(Twist(Layer::Bottom, Direction::CW));
const D_CCW: Move = Move::Twist(Twist(Layer::Bottom, Direction::CCW));
const F_CW: Move = Move::Twist(Twist(Layer::Front, Direction::CW));
const F_CCW: Move = Move::Twist(Twist(Layer::Front, Direction::CCW));
const B_CW: Move = Move::Twist(Twist(Layer::Back, Direction::CW));
const B_CCW: Move = Move::Twist(Twist(Layer::Back, Direction::CCW));
const R_CW: Move = Move::Twist(Twist(Layer::Right, Direction::CW));
const R_CCW: Move = Move::Twist(Twist(Layer::Right, Direction::CCW));
const L_CW: Move = Move::Twist(Twist(Layer::Left, Direction::CW));
const L_CCW: Move = Move::Twist(Twist(Layer::Left, Direction::CCW));

const ALL_MOVES: [Move; 12] = [
    U_CW, U_CCW, D_CW, D_CCW, F_CW, F_CCW, B_CW, B_CCW, R_CW, R_CCW, L_CW, L_CCW,
];

const ALL_LAYERS: [Layer; 6] = [
    Layer::Top,
    Layer::Bottom,
    Layer::Front,
    Layer::Back,
    Layer::Right,
    Layer::Left,
];

// ══════════════════════════════════════════════════════════════
// 1. Original hand-written tests
// ══════════════════════════════════════════════════════════════

#[test]
fn test_default_cube_is_assembled_correctly() {
    let columns = [
        Column {
            topright: Color::Red,
            botright: Color::Red,
            topleft: Color::Green,
            botleft: Color::Green,
        },
        Column {
            topright: Color::Green,
            botright: Color::Green,
            topleft: Color::Orange,
            botleft: Color::Orange,
        },
        Column {
            topright: Color::Orange,
            botright: Color::Orange,
            topleft: Color::Blue,
            botleft: Color::Blue,
        },
        Column {
            topright: Color::Blue,
            botright: Color::Blue,
            topleft: Color::Red,
            botleft: Color::Red,
        },
    ];
    assert_eq!(assemble_cube(columns).unwrap(), Cube::default());
}

#[test]
fn test_default_cube_with_front_rotation_is_assembled_correctly() {
    let columns = [
        Column {
            topright: Color::Red,
            botright: Color::Red,
            topleft: Color::Green,
            botleft: Color::Green,
        },
        Column {
            topright: Color::Green,
            botright: Color::Green,
            topleft: Color::Orange,
            botleft: Color::Orange,
        },
        Column {
            topright: Color::White,
            botright: Color::White,
            topleft: Color::Blue,
            botleft: Color::Blue,
        },
        Column {
            topright: Color::Blue,
            botright: Color::Blue,
            topleft: Color::Yellow,
            botleft: Color::Yellow,
        },
    ];
    assert_eq!(
        assemble_cube(columns).unwrap(),
        Cube::default().applied(F_CW)
    );
}

// ══════════════════════════════════════════════════════════════
// 2. Validate cube_to_columns against hand-written columns
//    (if this is wrong, every round-trip test is meaningless)
// ══════════════════════════════════════════════════════════════

#[test]
fn cube_to_columns_matches_default_handwritten() {
    let cols = cube_to_columns(&Cube::default());
    let expected = [
        Column {
            topright: Color::Red,
            botright: Color::Red,
            topleft: Color::Green,
            botleft: Color::Green,
        },
        Column {
            topright: Color::Green,
            botright: Color::Green,
            topleft: Color::Orange,
            botleft: Color::Orange,
        },
        Column {
            topright: Color::Orange,
            botright: Color::Orange,
            topleft: Color::Blue,
            botleft: Color::Blue,
        },
        Column {
            topright: Color::Blue,
            botright: Color::Blue,
            topleft: Color::Red,
            botleft: Color::Red,
        },
    ];
    assert_eq!(cols, expected);
}

#[test]
fn cube_to_columns_matches_front_cw_handwritten() {
    let cube = Cube::default().applied(F_CW);
    let cols = cube_to_columns(&cube);
    let expected = [
        Column {
            topright: Color::Red,
            botright: Color::Red,
            topleft: Color::Green,
            botleft: Color::Green,
        },
        Column {
            topright: Color::Green,
            botright: Color::Green,
            topleft: Color::Orange,
            botleft: Color::Orange,
        },
        Column {
            topright: Color::White,
            botright: Color::White,
            topleft: Color::Blue,
            botleft: Color::Blue,
        },
        Column {
            topright: Color::Blue,
            botright: Color::Blue,
            topleft: Color::Yellow,
            botleft: Color::Yellow,
        },
    ];
    assert_eq!(cols, expected);
}

// ══════════════════════════════════════════════════════════════
// 3. Round-trip: every single face rotation
// ══════════════════════════════════════════════════════════════

#[test]
fn roundtrip_every_single_rotation() {
    for &m in &ALL_MOVES {
        assert_roundtrip(Cube::default().applied(m));
    }
}

// ══════════════════════════════════════════════════════════════
// 4. Round-trip: 180° turns (CW twice)
// ══════════════════════════════════════════════════════════════

#[test]
fn roundtrip_half_turns() {
    for &layer in &ALL_LAYERS {
        let m = Move::Twist(Twist(layer, Direction::CW));
        assert_roundtrip(cube_from_moves(&[m, m]));
    }
}

// ══════════════════════════════════════════════════════════════
// 5. Inverse property: CW + CCW = identity (both orders)
// ══════════════════════════════════════════════════════════════

#[test]
fn cw_then_ccw_is_identity() {
    for &layer in &ALL_LAYERS {
        let cw = Move::Twist(Twist(layer, Direction::CW));
        let ccw = Move::Twist(Twist(layer, Direction::CCW));
        assert_eq!(cube_from_moves(&[cw, ccw]), Cube::default());
        assert_eq!(cube_from_moves(&[ccw, cw]), Cube::default());
    }
}

// ══════════════════════════════════════════════════════════════
// 6. Four CW = identity
// ══════════════════════════════════════════════════════════════

#[test]
fn four_cw_rotations_is_identity() {
    for &layer in &ALL_LAYERS {
        let m = Move::Twist(Twist(layer, Direction::CW));
        assert_eq!(cube_from_moves(&[m, m, m, m]), Cube::default());
    }
}

// ══════════════════════════════════════════════════════════════
// 7. Round-trip: multi-move scrambles (cross-layer corner placement)
// ══════════════════════════════════════════════════════════════

#[test]
fn roundtrip_sexy_move() {
    // R U R' U' — the "sexy move", touches R and U faces
    assert_roundtrip(cube_from_moves(&[R_CW, U_CW, R_CCW, U_CCW]));
}

#[test]
fn roundtrip_sexy_move_repeated() {
    // (R U R' U')² — two repetitions, still scrambled
    assert_roundtrip(cube_from_moves(&[
        R_CW, U_CW, R_CCW, U_CCW, R_CW, U_CW, R_CCW, U_CCW,
    ]));
}

#[test]
fn sexy_move_six_times_is_identity() {
    // (R U R' U')⁶ = identity — a well-known Rubik's cube property
    let sexy = [R_CW, U_CW, R_CCW, U_CCW];
    let moves: Vec<Move> = (0..6).flat_map(|_| sexy.iter().copied()).collect();
    assert_eq!(cube_from_moves(&moves), Cube::default());
}

#[test]
fn roundtrip_sune() {
    // R U R' U R U² R' — the "sune" OLL algorithm
    assert_roundtrip(cube_from_moves(&[
        R_CW, U_CW, R_CCW, U_CW, R_CW, U_CW, U_CW, R_CCW,
    ]));
}

#[test]
fn roundtrip_all_faces_mixed() {
    // Touch every face, mixing CW and CCW
    assert_roundtrip(cube_from_moves(&[
        U_CW, D_CCW, F_CW, B_CCW, R_CW, L_CCW, U_CCW, D_CW, F_CCW, B_CW, R_CCW, L_CW,
    ]));
}

#[test]
fn roundtrip_deep_scramble() {
    // Long enough to thoroughly mix top/bottom corners
    assert_roundtrip(cube_from_moves(&[
        R_CW, U_CW, F_CW, R_CCW, U_CCW, F_CCW, L_CW, D_CW, B_CW, L_CCW, D_CCW, B_CCW, R_CW, U_CW,
        R_CCW, U_CCW,
    ]));
}

// ══════════════════════════════════════════════════════════════
// 8. Double inverse: cube_to_columns ∘ assemble_cube = identity
//    (column representation is unique)
// ══════════════════════════════════════════════════════════════

#[test]
fn double_inverse_default() {
    let cols = cube_to_columns(&Cube::default());
    let cube = assemble_cube(cols).unwrap();
    let cols2 = cube_to_columns(&cube);
    assert_eq!(cols, cols2);
}

#[test]
fn double_inverse_scrambled() {
    let cube = cube_from_moves(&[R_CW, U_CW, F_CW, R_CCW, U_CCW, F_CCW]);
    let cols = cube_to_columns(&cube);
    let cube2 = assemble_cube(cols).unwrap();
    let cols2 = cube_to_columns(&cube2);
    assert_eq!(cols, cols2);
}

// ══════════════════════════════════════════════════════════════
// 9. Invalid inputs: should return None
// ══════════════════════════════════════════════════════════════

#[test]
fn invalid_color_pair_returns_none() {
    // White + Yellow are opposite faces — never adjacent on a corner.
    let columns = [
        Column {
            topright: Color::White,
            topleft: Color::Yellow,
            botright: Color::Red,
            botleft: Color::Green,
        },
        Column {
            topright: Color::Green,
            topleft: Color::Orange,
            botright: Color::Green,
            botleft: Color::Orange,
        },
        Column {
            topright: Color::Orange,
            topleft: Color::Blue,
            botright: Color::Orange,
            botleft: Color::Blue,
        },
        Column {
            topright: Color::Blue,
            topleft: Color::Red,
            botright: Color::Blue,
            botleft: Color::Red,
        },
    ];
    assert!(assemble_cube(columns).is_none());
}

#[test]
fn duplicate_corner_returns_none() {
    // All four columns identical → corner 0 found 4×, others never found.
    let col = Column {
        topright: Color::Red,
        topleft: Color::Green,
        botright: Color::Red,
        botleft: Color::Green,
    };
    assert!(assemble_cube([col, col, col, col]).is_none());
}

#[test]
fn invalid_twist_one_corner_returns_none() {
    // Twist a single corner by +1 — total twist = 1 ≢ 0 (mod 3).
    let mut cube = Cube::default();
    cube.0[0].set_ori(1);
    let cols = cube_to_columns(&cube);
    assert!(assemble_cube(cols).is_none());
}

#[test]
fn invalid_twist_two_corners_returns_none() {
    // Twist two corners by +1 each — total twist = 2 ≢ 0 (mod 3).
    let mut cube = Cube::default();
    cube.0[0].set_ori(1);
    cube.0[1].set_ori(1);
    let cols = cube_to_columns(&cube);
    assert!(assemble_cube(cols).is_none());
}

#[test]
fn valid_twist_three_corners_is_accepted() {
    // Three corners twisted by +1 — total twist = 3 ≡ 0 (mod 3).
    // This is a reachable state.
    let mut cube = Cube::default();
    cube.0[0].set_ori(1);
    cube.0[1].set_ori(1);
    cube.0[2].set_ori(1);
    let cols = cube_to_columns(&cube);
    assert!(assemble_cube(cols).is_some());
}
