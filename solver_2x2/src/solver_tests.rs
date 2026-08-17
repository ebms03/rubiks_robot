use rand::Rng;
use rand::RngExt;
use rand::SeedableRng;
use rand::rngs::SmallRng;

use super::cube::*;
use super::solver::*;

fn apply_seq(start: Cube, seq: &[Move]) -> Cube {
    seq.iter().copied().fold(start, Cube::applied)
}

fn ru_moveset() -> Vec<Move> {
    vec![
        Move::Twist(Twist(Layer::Right, Direction::CW)),
        Move::Twist(Twist(Layer::Right, Direction::CCW)),
        Move::Twist(Twist(Layer::Top, Direction::CW)),
        Move::Twist(Twist(Layer::Top, Direction::CCW)),
    ]
}
fn bd_moveset() -> Vec<Move> {
    vec![
        Move::Twist(Twist(Layer::Back, Direction::CW)),
        Move::Twist(Twist(Layer::Back, Direction::CCW)),
        Move::Twist(Twist(Layer::Bottom, Direction::CW)),
        Move::Twist(Twist(Layer::Bottom, Direction::CCW)),
    ]
}

fn random_scramble(rng: &mut impl Rng, moveset: &[Move], n: usize) -> Vec<Move> {
    let mut seq = Vec::with_capacity(n);
    let mut last: Option<Move> = None;
    for _ in 0..n {
        loop {
            let m = moveset[rng.random_range(0..moveset.len())];
            // don't trivially undo the previous move
            if Some(m) == last.map(|l| l.inverse()) {
                continue;
            }
            seq.push(m);
            last = Some(m);
            break;
        }
    }
    seq
}

#[test]
fn bfs_solves_depth_1_for_every_move() {
    let moves = Twist::ALL.map(Move::Twist).to_vec();
    let mut solver = BfsSolver::new(); // Allocate 264 MB ONCE
    for &m in moves.iter() {
        let start = Cube::SOLVED.applied(m);
        let sol = solver
            .solve(start, &moves)
            .expect("depth 1 must be solvable");
        assert_eq!(sol.len(), 1, "expected 1-move solution for {:?}", m);
        assert!(start.applied(sol[0]).is_solved(), "solution didn't solve");
    }
}

#[test]
fn small_bfs_scrambles() {
    let moves = Move::ALL.to_vec();
    let mut solver = BfsSolver::new(); // Allocate 264 MB ONCE
    let mut rng = SmallRng::seed_from_u64(0xDEAD);

    for i in [2, 3, 4, 5] {
        for _ in 0..50 {
            let scramble = random_scramble(&mut rng, &moves, i);
            let start = apply_seq(Cube::SOLVED, &scramble);
            let bfs = solver.solve(start, &moves).expect("should be solvable");
            assert!(apply_seq(start, &bfs).is_solved());
        }
    }
}

fn ida_matches_bfs_on_small_moveset(moves: &[Move], is_complete: bool) {
    let mut solver = BfsSolver::new(); // Allocate 264 MB ONCE
    let (table, complete) = build_move_table(&moves);
    assert_eq!(complete, is_complete);

    let mut rng = SmallRng::seed_from_u64(0xDEAD);
    for _ in 0..50 {
        let scramble = random_scramble(&mut rng, &moves, 10);
        let start = apply_seq(Cube::SOLVED, &scramble);

        let ida = solve(start, &table).expect("should be solvable");
        let bfs = solver.solve(start, &moves).expect("should be solvable");
        let ida_solved = apply_seq(start, &ida);
        let bfs_solved = apply_seq(start, &bfs);
        // Both must be valid...
        assert!(ida_solved.is_solved());
        assert!(bfs_solved.is_solved());
        // ...and the same length (IDA* is optimal given an admissible heuristic,
        // and the BFS distance table is admissible by construction).
        assert_eq!(
            ida.len(),
            bfs.len(),
            "scramble {:?}: ida={} bfs={}",
            scramble,
            ida.len(),
            bfs.len()
        );
    }
}

#[test]
fn ida_matches_bfs_on_ru_moveset() {
    ida_matches_bfs_on_small_moveset(&ru_moveset(), false);
}

#[test]
fn ida_matches_bfs_on_bd_moveset() {
    ida_matches_bfs_on_small_moveset(&bd_moveset(), false);
}
