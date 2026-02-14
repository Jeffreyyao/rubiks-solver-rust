use std::collections::{HashSet, VecDeque};

use crate::cube;
use crate::profile;

pub struct Solver;

impl Solver {
    pub const G0_MOVES: [&str; 12] = ["u", "u'", "d", "d'", "l", "l'", "r", "r'", "f", "f'", "b", "b'"];
    pub const G1_MOVES: [&str; 10] = ["u2", "d2", "l", "l'", "r", "r'", "f", "f'", "b", "b'"];
    // pub const G2_MOVES: [&str; 8] = ["u2", "d2", "l", "l'", "r", "r'", "f2", "b2"];
    // pub const G3_MOVES: [&str; 6] = ["u2", "d2", "l2", "r2", "f2", "b2"];

    pub fn orientations_to_index(orientations: &[u8], modulus: u8) -> u32 {
        let mut index = 0;
        for i in 0..orientations.len() {
            index = index * modulus as u32 + orientations[i] as u32;
        }
        index
    }

    pub fn combinations_to_index(combinations: &[u8]) -> u32 {
        let mut c: Vec<u8> = combinations.to_vec();
        c.sort();
        let mut index = 0;
        for i in 0..c.len() {
            index += c[i] as u32 * (100_u32.pow(i as u32));
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
        let mut visited = HashSet::from([Self::orientations_to_index(&cube.edge_orientations, 2)]);
        while let Some((current_cube, current_moves)) = queue.pop_front() {
            if Self::is_solved_g0(current_cube) {
                p.end();
                return current_moves;
            }
            for m in Self::G0_MOVES {
                let new_cube = current_cube.apply_sequence(m);
                if !visited.contains(&Self::orientations_to_index(&new_cube.edge_orientations, 2)) {
                    visited.insert(Self::orientations_to_index(&new_cube.edge_orientations, 2));
                    let mut new_moves = current_moves.clone();
                    new_moves.push(m.to_string());
                    queue.push_back((new_cube, new_moves));
                }
            }
        }
        p.report("no solution found");
        return vec![];
    }

    pub fn get_g1_index(cube: cube::Cube) -> u64 {
        let corner_orientation_index = Self::orientations_to_index(&cube.corner_orientations, 3);
        let mut lr_mid_slice_permutation = [0; 4];
        for i in 0..4 {
            lr_mid_slice_permutation[i] = cube.edge_permutations[cube::Cube::LR_MID_SLICE_EDGES[i as usize] as usize];
        }
        let lr_mid_slice_combination_index = Self::combinations_to_index(&lr_mid_slice_permutation);
        lr_mid_slice_combination_index as u64 * (3_u64.pow(7)) + corner_orientation_index as u64
    }

    fn is_solved_g1(cube: cube::Cube) -> bool { // assumes g0 solved
        let g1_index = Self::get_g1_index(cube);
        g1_index == 10080200 * (3_u64.pow(7))
    }

    pub fn solve_g1(cube: cube::Cube) -> Vec<String> {
        if Self::is_solved_g1(cube) {
            return vec![];
        }

        let mut min_move_size = 0;

        let p = profile::Profile::start("solve_g1");

        let mut queue = VecDeque::from([(cube, vec![])]);
        let g1_index = Self::get_g1_index(cube);
        let mut visited_g1_indices = HashSet::from([g1_index]);
        while let Some((current_cube, current_moves)) = queue.pop_front() {
            if Self::is_solved_g1(current_cube) {
                p.end();
                return current_moves;
            }
            if current_moves.len() > min_move_size {
                min_move_size = current_moves.len();
                println!("move size: {}, visited_g1_indices size: {}", min_move_size, visited_g1_indices.len());
            }
            for m in Self::G1_MOVES {
                let new_cube = current_cube.apply_sequence(m);
                let new_g1_index = Self::get_g1_index(new_cube);
                if !visited_g1_indices.contains(&new_g1_index) {
                    visited_g1_indices.insert(new_g1_index);
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