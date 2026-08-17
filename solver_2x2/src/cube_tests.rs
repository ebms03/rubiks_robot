use super::cube::*;
use std::collections::HashSet;

// ---------- construction ----------

#[test]
fn default_is_solved() {
    assert!(Cube::default().is_solved());
    assert!(Cube::SOLVED.is_solved());
}

#[test]
fn solved_class_has_24_elements() {
    let class = Cube::solved_class();
    assert_eq!(class.len(), 24);
    // all distinct
    let set: HashSet<Cube> = class.iter().copied().collect();
    assert_eq!(set.len(), 24);
    // all "solved"
    for c in class {
        assert!(c.is_solved());
    }
}

#[test]
fn solved_class_contains_all_axis_rotations() {
    let s = Cube::SOLVED;
    for &axis in Axis::ALL.iter() {
        for &dir in Direction::ALL.iter() {
            assert!(s.applied(Move::Rotation(Rotation(axis, dir))).is_solved());
        }
    }
}

// ---------- corner bit packing ----------

#[test]
fn corner_id_ori_roundtrip() {
    for id in 0..8u8 {
        for ori in 0..3u8 {
            let c = Corner::new_with_id_ori(id, ori);
            assert_eq!(c.get_id(), id);
            assert_eq!(c.get_ori(), ori);
        }
    }
}

#[test]
#[should_panic]
fn corner_id_out_of_range_panics() {
    Corner::new_with_id(8);
}

#[test]
#[should_panic]
fn corner_ori_out_of_range_panics() {
    let mut c = Corner::new_with_id(0);
    c.set_ori(3);
}

#[test]
fn corner_set_preserves_other_field() {
    let mut c = Corner::new_with_id_ori(5, 2);
    c.set_id(3);
    assert_eq!((c.get_id(), c.get_ori()), (3, 2));
    c.set_ori(0);
    assert_eq!((c.get_id(), c.get_ori()), (3, 0));
}

// ---------- twist invariants ----------

#[test]
fn twist_preserves_twist_sum_mod_3() {
    let mut c = Cube::default();
    for _ in 0..50 {
        let m = Move::Twist(Twist(Layer::ALL[0], Direction::CW));
        c = c.applied(m);
        assert!(c.is_twist_valid(), "twist sum must remain 0 mod 3");
    }
}

#[test]
fn four_twists_same_face_is_identity() {
    for &layer in Layer::ALL.iter() {
        for &dir in Direction::ALL.iter() {
            let s = Cube::default();
            let t = Twist(layer, dir);
            let after4 = s
                .applied(Move::Twist(t))
                .applied(Move::Twist(t))
                .applied(Move::Twist(t))
                .applied(Move::Twist(t));
            assert_eq!(after4, s, "{:?}^4 should be identity", t);
        }
    }
}

#[test]
fn twist_inverse_is_inverse() {
    for &layer in Layer::ALL.iter() {
        for &dir in Direction::ALL.iter() {
            let s = Cube::default();
            let m = Move::Twist(Twist(layer, dir));
            let inv = Move::Twist(Twist(layer, dir.inverse()));
            assert_eq!(s.applied(m).applied(inv), s);
        }
    }
}

#[test]
fn rotation_inverse_is_inverse() {
    for &axis in Axis::ALL.iter() {
        for &dir in Direction::ALL.iter() {
            let s = Cube::default();
            let m = Move::Rotation(Rotation(axis, dir));
            let inv = Move::Rotation(Rotation(axis, dir.inverse()));
            assert_eq!(s.applied(m).applied(inv), s);
        }
    }
}

// ---------- rotation group structure ----------

#[test]
fn four_rotations_same_axis_is_identity() {
    for &axis in Axis::ALL.iter() {
        for &dir in Direction::ALL.iter() {
            let s = Cube::default();
            let r = Move::Rotation(Rotation(axis, dir));
            let after4 = s.applied(r).applied(r).applied(r).applied(r);
            assert_eq!(after4, s, "{:?}^4 should be identity", r);
        }
    }
}

#[test]
fn rotation_order_is_4_or_2() {
    // any single rotation has order dividing 4
    for &axis in Axis::ALL.iter() {
        for &dir in Direction::ALL.iter() {
            let s = Cube::default();
            let r = Move::Rotation(Rotation(axis, dir));
            let mut c = s;
            let mut order = 0;
            for k in 1..=4 {
                c = c.applied(r);
                if c == s {
                    order = k;
                    break;
                }
            }
            assert!(order == 2 || order == 4, "order was {}", order);
            assert!(order != 0, "rotation never returned to identity in 4 steps");
        }
    }
}

#[test]
fn rotation_generates_at_most_24_states() {
    let s = Cube::default();
    let mut seen = HashSet::new();
    let mut frontier = vec![s];
    seen.insert(s);
    while let Some(c) = frontier.pop() {
        for &r in Rotation::ALL.iter() {
            let n = c.applied(Move::Rotation(r));
            if seen.insert(n) {
                frontier.push(n);
            }
        }
    }
    assert_eq!(seen.len(), 24);
}

// ---------- known cube identities ----------

#[test]
fn sexy_move_has_order_6() {
    // (R U R' U')^6 = identity on the 2x2x2
    let r = Move::Twist(Twist(Layer::Right, Direction::CW));
    let rp = Move::Twist(Twist(Layer::Right, Direction::CCW));
    let u = Move::Twist(Twist(Layer::Top, Direction::CW));
    let up = Move::Twist(Twist(Layer::Top, Direction::CCW));
    let s = Cube::default();
    let mut c = s;
    for _ in 0..6 {
        c = c.applied(r).applied(u).applied(rp).applied(up);
    }
    assert_eq!(c, s);
}

#[test]
fn u_and_d_commute() {
    // U and D don't share corners, so they commute on 2x2x2
    let u = Move::Twist(Twist(Layer::Top, Direction::CW));
    let d = Move::Twist(Twist(Layer::Bottom, Direction::CW));
    let s = Cube::default();
    let ud = s.applied(u).applied(d);
    let du = s.applied(d).applied(u);
    assert_eq!(ud, du);
}

#[test]
fn opposite_face_twists_commute() {
    for axis_pairs in [
        (Layer::Top, Layer::Bottom),
        (Layer::Left, Layer::Right),
        (Layer::Front, Layer::Back),
    ] {
        let (a, b) = axis_pairs;
        let ma = Move::Twist(Twist(a, Direction::CW));
        let mb = Move::Twist(Twist(b, Direction::CW));
        let s = Cube::default();
        assert_eq!(s.applied(ma).applied(mb), s.applied(mb).applied(ma));
    }
}

// ---------- is_solved ----------

#[test]
fn single_twist_is_not_solved() {
    for &layer in Layer::ALL.iter() {
        for &dir in Direction::ALL.iter() {
            let s = Cube::default();
            let c = s.applied(Move::Twist(Twist(layer, dir)));
            assert!(
                !c.is_solved(),
                "{:?} {:?} should leave unsolved",
                layer,
                dir
            );
        }
    }
}

#[test]
fn solved_in_any_orientation() {
    // apply random rotations, must still be solved
    let s = Cube::default();
    let mut c = s;
    let seq = [
        Move::Rotation(Rotation(Axis::X, Direction::CW)),
        Move::Rotation(Rotation(Axis::Y, Direction::CCW)),
        Move::Rotation(Rotation(Axis::Z, Direction::CW)),
        Move::Rotation(Rotation(Axis::X, Direction::CCW)),
    ];
    for m in seq {
        c = c.applied(m);
    }
    assert!(c.is_solved(), "rotated SOLVED must still be SOLVED");
}

#[test]
fn is_solved_false_for_two_twists() {
    let s = Cube::default();
    let c = s
        .applied(Move::Twist(Twist(Layer::Right, Direction::CW)))
        .applied(Move::Twist(Twist(Layer::Top, Direction::CW)));
    assert!(!c.is_solved());
}

// ---------- lehmer code ----------

#[test]
fn dense_index_roundtrip() {
    // Use a deterministic scramble that hits non-trivial perm + ori.
    let mut c = Cube::default();
    let seq = [
        Move::Twist(Twist(Layer::Right, Direction::CW)),
        Move::Twist(Twist(Layer::Top, Direction::CCW)),
        Move::Twist(Twist(Layer::Front, Direction::CW)),
        Move::Twist(Twist(Layer::Left, Direction::CCW)),
    ];
    for m in seq {
        c = c.applied(m);
    }
    assert!(c.is_twist_valid());

    let idx = c.dense_index();
    assert!(idx < Cube::STATE_SPACE);
    assert_eq!(Cube::from_dense_index(idx), c);
}

#[test]
fn dense_index_solved_is_zero() {
    assert_eq!(Cube::SOLVED.dense_index(), 0);
}

#[test]
fn dense_index_bijection_on_solved_class() {
    let class = Cube::solved_class();
    let indices: std::collections::HashSet<u32> = class.iter().map(|c| c.dense_index()).collect();
    assert_eq!(
        indices.len(),
        24,
        "24 solved-class states must map to 24 distinct indices"
    );
}

// ---------- face table sanity ----------

#[test]
fn face_table_has_four_distinct_indices() {
    for &layer in Layer::ALL.iter() {
        let (idx, _) = Cube::face_table(layer);
        let s: HashSet<usize> = idx.iter().copied().collect();
        assert_eq!(s.len(), 4, "face {:?} indices must be distinct", layer);
        for i in idx.iter() {
            assert!(*i < 8);
        }
    }
}

#[test]
fn face_table_orientation_deltas_in_range() {
    for &layer in Layer::ALL.iter() {
        let (_, deltas) = Cube::face_table(layer);
        for &d in deltas.iter() {
            assert!(d < 3);
        }
    }
}

// ---------- equivalence of solved checks ----------

#[test]
fn solved_class_matches_uniform_orientation_property() {
    // Every member of solved_class must have its corner multiset == {0,1,...,7}
    // and twist sum == 0 (rotation is a valid move, so twist_sum must hold).
    for c in Cube::solved_class() {
        assert!(c.is_twist_valid());
        let mut ids: Vec<u8> = c.ids().to_vec();
        ids.sort();
        assert_eq!(ids, [0, 1, 2, 3, 4, 5, 6, 7]);
    }
}

// ---------- moveset-dependent solve semantics ----------

#[test]
fn d_conjugates_to_b_under_x() {
    let s = Cube::default();
    let x_cw = Move::Rotation(Rotation(Axis::X, Direction::CW));
    let x_ccw = Move::Rotation(Rotation(Axis::X, Direction::CCW));
    let d_cw = Move::Twist(Twist(Layer::Bottom, Direction::CW));

    // X D X'  ==  F_CW
    let f_emulated = s.applied(x_cw).applied(d_cw).applied(x_ccw);
    let f_real = s.applied(Move::Twist(Twist(Layer::Back, Direction::CW)));
    assert_eq!(f_emulated, f_real);
}

#[test]
fn d_conjugates_to_u_under_x_squared() {
    let s = Cube::default();
    let x_cw = Move::Rotation(Rotation(Axis::X, Direction::CW));
    let d_cw = Move::Twist(Twist(Layer::Bottom, Direction::CW));

    // X² D X⁻²  ==  U_CW   (X² has order 2, so X⁻² == X²)
    let u_emulated = s
        .applied(x_cw)
        .applied(x_cw)
        .applied(d_cw)
        .applied(x_cw)
        .applied(x_cw);
    let u_real = s.applied(Move::Twist(Twist(Layer::Top, Direction::CW)));
    assert_eq!(u_emulated, u_real);
}

#[test]
fn move_pack_roundtrips() {
    for m in Move::ALL {
        assert_eq!(
            m,
            Move::unpack(m.pack()).expect("valid move must be unpackable")
        );
    }
}
