use std::collections::{HashSet, VecDeque};

use crate::cube;
use crate::profile;

pub struct Solver;

impl Solver {
    const G0_MOVES: [&str; 12] = ["u", "u'", "d", "d'", "l", "l'", "r", "r'", "f", "f'", "b", "b'"];
    const G1_MOVES: [&str; 10] = ["u2", "d2", "l", "l'", "r", "r'", "f", "f'", "b", "b'"];
    // const G2_MOVES: [&str; 8] = ["u2", "d2", "l", "l'", "r", "r'", "f2", "b2"];
    // const G3_MOVES: [&str; 6] = ["u2", "d2", "l2", "r2", "f2", "b2"];

    fn edge_orientation_to_index(edge_orientations: [u8; 12]) -> u16 {
        let mut index = 0;
        for i in 0..12 {
            index = index * 2 + edge_orientations[i] as u16;
        }
        index
    }

    fn is_solved_g0(cube: cube::Cube) -> bool {
        cube.edge_orientations == [0; 12]
    }

    pub fn solve_g0(cube: cube::Cube) -> Vec<String> {
        if Self::is_solved_g0(cube) {
            return vec![];
        }

        let p = profile::Profile::start("solve_g0");

        let mut queue = VecDeque::from([(cube, vec![])]);
        let mut visited = HashSet::from([Self::edge_orientation_to_index(cube.edge_orientations)]);
        while let Some((current_cube, current_moves)) = queue.pop_front() {
            if Self::is_solved_g0(current_cube) {
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

    fn corner_orientation_to_index(corner_orientations: [u8; 8]) -> u16 {
        let mut index = 0;
        for i in 0..8 {
            index = index * 3 + corner_orientations[i] as u16;
        }
        index
    }

    fn is_solved_g1(cube: cube::Cube) -> bool { // assumes g0 solved
        // corner orientations solved
        if cube.corner_orientations != [0; 8] {
            return false;
        }
        // FUBD slice edge cubies on FUBD slice
        const G1_EDGES: [u8; 4] = [0, 2, 8, 10];
        for i in 0..4 {
            let edge_index = cube.edge_permutations[G1_EDGES[i as usize] as usize];
            if G1_EDGES.contains(&edge_index) {
                return false;
            }
        }
        true
    }

    pub fn solve_g1(cube: cube::Cube) -> Vec<String> {
        if Self::is_solved_g1(cube) {
            return vec![];
        }

        let mut min_move_size = 0;

        let p = profile::Profile::start("solve_g1");

        let mut queue = VecDeque::from([(cube, vec![])]);
        let mut visited = HashSet::from([(Self::corner_orientation_to_index(cube.corner_orientations), cube.edge_permutations)]);
        while let Some((current_cube, current_moves)) = queue.pop_front() {
            if Self::is_solved_g1(current_cube) {
                p.end();
                return current_moves;
            }
            if current_moves.len() > min_move_size {
                min_move_size = current_moves.len();
                println!("min move size: {}", min_move_size);
            }
            // println!("current moves: {:?}", current_moves);
            for m in Self::G1_MOVES {
                let new_cube = current_cube.apply_sequence(m);
                if !visited.contains(&(Self::corner_orientation_to_index(new_cube.corner_orientations), new_cube.edge_permutations)) {
                    visited.insert((Self::corner_orientation_to_index(new_cube.corner_orientations), new_cube.edge_permutations));
                    let mut new_moves = current_moves.clone();
                    new_moves.push(m.to_string());
                    queue.push_back((new_cube, new_moves));
                }
            }
        }
        p.report("no solution found");
        return vec![];
    }
}