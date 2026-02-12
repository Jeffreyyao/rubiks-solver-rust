use std::collections::{HashSet, VecDeque};

use crate::cube;

pub struct Solver;

impl Solver {
    const G0_MOVES: [&str; 12] = ["u", "u'", "d", "d'", "l", "l'", "r", "r'", "f", "f'", "b", "b'"];
    // const G1_MOVES: [&str; 14] = ["u2", "d2", "l", "l'", "l2", "r", "r'", "r2", "f", "f'", "f2", "b", "b'", "b2"];
    // const G2_MOVES: [&str; 10] = ["u2", "d2", "l", "l'", "l2", "r", "r'", "r2", "f2", "b2"];
    // const G3_MOVES: [&str; 6] = ["u2", "d2", "l2", "r2", "f2", "b2"];

    pub fn solve_g0(cube: cube::Cube) -> Vec<String> {
        if cube.edge_orientations == [0; 12] {
            return vec![];
        }
        let mut queue = VecDeque::from([(cube, vec![])]);
        let mut visited = HashSet::from([cube]);
        while let Some((current_cube, current_moves)) = queue.pop_front() {
            println!("moves size: {}, current moves: {:?}, queue size: {}", current_moves.len(), current_moves, queue.len());
            if current_cube.edge_orientations == [0; 12] {
                return current_moves;
            }
            for m in Self::G0_MOVES {
                if current_moves.len() > 0 {
                    let last_move = &current_moves[current_moves.len() - 1];
                    if last_move.chars().nth(0) == m.chars().nth(0) && last_move.len() != m.len() {
                        // skip if same face but different direction eg f and f'
                        continue;
                    }
                }
                let new_cube = current_cube.apply_sequence(m);
                if !visited.contains(&new_cube) {
                    visited.insert(new_cube);
                    let mut new_moves = current_moves.clone();
                    new_moves.push(m.to_string());
                    queue.push_back((new_cube, new_moves));
                }
            }
        }
        println!("solve_g0 no solution found");
        return vec![];
    }
}