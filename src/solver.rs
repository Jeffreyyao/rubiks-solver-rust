use std::collections::{HashSet, VecDeque};

use crate::cube;
use crate::profile;

pub struct Solver;

impl Solver {
    const G0_MOVES: [&str; 12] = ["u", "u'", "d", "d'", "l", "l'", "r", "r'", "f", "f'", "b", "b'"];
    // const G1_MOVES: [&str; 10] = ["u2", "d2", "l", "l'", "r", "r'", "f", "f'", "b", "b'"];
    // const G2_MOVES: [&str; 8] = ["u2", "d2", "l", "l'", "r", "r'", "f2", "b2"];
    // const G3_MOVES: [&str; 6] = ["u2", "d2", "l2", "r2", "f2", "b2"];

    fn edge_orientation_to_index(edge_orientations: [i8; 12]) -> u16 {
        let mut index = 0;
        for i in 0..12 {
            index = index * 2 + edge_orientations[i] as u16;
        }
        index
    }

    pub fn solve_g0(cube: cube::Cube) -> Vec<String> {
        if cube.edge_orientations == [0; 12] {
            return vec![];
        }

        let p = profile::Profile::start("solve_g0");

        let mut queue = VecDeque::from([(cube, vec![])]);
        let mut visited = HashSet::from([Self::edge_orientation_to_index(cube.edge_orientations)]);
        while let Some((current_cube, current_moves)) = queue.pop_front() {
            if current_cube.edge_orientations == [0; 12] {
                p.end();
                return current_moves;
            }
            for m in Self::G0_MOVES {
                let new_cube = current_cube.apply_sequence(m);
                if !visited.contains(&Self::edge_orientation_to_index(new_cube.edge_orientations)) {
                    visited.insert(Self::edge_orientation_to_index(new_cube.edge_orientations));
                    let mut new_moves = current_moves.clone();
                    new_moves.push(m.to_string());
                    queue.push_back((new_cube, new_moves));
                }
            }
        }
        p.report("no solution found");
        return vec![];
    }

    pub fn solve_g1(cube: cube::Cube) -> Vec<String> {
        let p = profile::Profile::start("solve_g1");
        return vec![];
    }
}