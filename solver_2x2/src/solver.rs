use std::{
    collections::VecDeque,
    io::{Read, Write},
};

use crate::cube::*;
pub struct BfsSolver {
    incoming: Vec<u16>,
    depth: Vec<u8>,
}

impl BfsSolver {
    pub fn new() -> Self {
        let n = Cube::STATE_SPACE as usize;
        Self {
            incoming: vec![u16::MAX; n],
            depth: vec![u8::MAX; n],
        }
    }

    pub fn solve(&mut self, start: Cube, moves: &[Move]) -> Option<Vec<Move>> {
        // Reset only the entries we touch, or just reset the whole thing.
        // For small tests, resetting the whole array is fast enough (88 MB memset).
        self.incoming.fill(u16::MAX);
        self.depth.fill(u8::MAX);

        let mut queue = VecDeque::new();

        for &solved in Cube::solved_class() {
            let i = solved.dense_index() as usize;
            if self.depth[i] == u8::MAX {
                self.depth[i] = 0;
                self.incoming[i] = 0;
                queue.push_back(solved);
            }
        }

        while let Some(c) = queue.pop_front() {
            if c == start {
                break;
            }
            let d = self.depth[c.dense_index() as usize];
            for (mi, &m) in moves.iter().enumerate() {
                let n = c.applied(m);
                let ni = n.dense_index() as usize;
                if self.depth[ni] == u8::MAX {
                    self.depth[ni] = d + 1;
                    self.incoming[ni] = (mi + 1) as u16;
                    queue.push_back(n);
                }
            }
        }

        let si = start.dense_index() as usize;
        if self.depth[si] == u8::MAX {
            return None;
        }

        let mut path = Vec::new();
        let mut cur = start;
        while self.incoming[cur.dense_index() as usize] != 0 {
            let mi = self.incoming[cur.dense_index() as usize] as usize - 1;
            let m = moves[mi];
            path.push(m.inverse());
            cur = cur.applied(m.inverse());
        }
        Some(path)
    }
}

pub fn solve(mut cube: Cube, move_table: &Table) -> Option<Vec<Move>> {
    let mut out = Vec::new();
    while !cube.is_solved() {
        let m = Move::unpack(move_table.0[cube.dense_index() as usize])?;
        cube = cube.applied(m);
        out.push(m);
    }
    Some(out)
}

pub struct Table(pub(crate) Vec<u8>);

/// Plain BFS over canonical states, writing into a flat `Vec<u8>`.
pub fn build_move_table(moveset: &[Move]) -> (Table, bool) {
    const UNVISITED: u8 = u8::MAX; // cant be unpacked to valid move
    let mut table = vec![UNVISITED; Cube::STATE_SPACE as usize];
    let mut queue = VecDeque::new();
    for &goal in Cube::solved_class() {
        let i = goal.dense_index();
        if table[i as usize] == UNVISITED {
            table[i as usize] = 0;
            queue.push_back(goal);
        }
    }
    while let Some(c) = queue.pop_front() {
        for &m in moveset {
            let n = c.applied(m);
            let ni = n.dense_index() as usize;
            if table[ni] == UNVISITED {
                table[ni] = m.inverse().pack();
                queue.push_back(n);
            }
        }
    }
    let complete = table.iter().all(|i| *i != UNVISITED);
    (Table(table), complete)
}

impl Table {
    pub fn load(mut r: impl Read) -> std::io::Result<Table> {
        let mut bytes = vec![];
        r.read_to_end(&mut bytes)?;
        return Ok(Table(bytes));
    }

    pub fn save(self: &Table, mut w: impl Write) -> std::io::Result<()> {
        w.write_all(&self.0)
    }
}
