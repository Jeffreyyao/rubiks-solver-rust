use std::collections::{HashSet, VecDeque};

use crate::cube;
use crate::profile;

pub struct Solver;

const FACTORIALS: [u64; 12] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800, 39916800];

pub fn comb(n: u64, r: u64) -> u64 {
    if n < r {
        return 0;
    }
    FACTORIALS[n as usize] / (FACTORIALS[r as usize] * FACTORIALS[(n - r) as usize])
}

impl Solver {
    fn solve_bfs(
        cube: cube::Cube,
        is_solved: fn(cube::Cube) -> bool,
        fn_get_index: fn(cube::Cube) -> u64,
        moves: &[&str],
        name: String,
    ) -> Vec<String> {
        if is_solved(cube) {
            return vec![];
        }
        let p = profile::Profile::start(&name);
        let mut queue = VecDeque::from([(cube, vec![])]);
        let mut visited_indices = HashSet::from([fn_get_index(cube)]);
        while let Some((current_cube, current_moves)) = queue.pop_front() {
            if is_solved(current_cube) {
                p.end();
                return current_moves;
            }
            for m in moves {
                let new_cube = current_cube.apply_sequence(m);
                let new_index = fn_get_index(new_cube);
                if !visited_indices.contains(&new_index) {
                    visited_indices.insert(new_index);
                    let mut new_moves = current_moves.clone();
                    new_moves.push(m.to_string());
                    queue.push_back((new_cube, new_moves));
                }
            }
        }
        p.report("no solution found");
        return vec![];
    }

    pub const G0_MOVES: [&str; 12] = ["u", "u'", "d", "d'", "l", "l'", "r", "r'", "f", "f'", "b", "b'"];
    pub const G1_MOVES: [&str; 10] = ["u2", "d2", "l", "l'", "r", "r'", "f", "f'", "b", "b'"];
    pub const G2_MOVES: [&str; 8] = ["u2", "d2", "l", "l'", "r", "r'", "f2", "b2"];
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
        for i in (0..c.len()).rev() {
            index += comb(c[i] as u64, (i + 1) as u64) as u32;
        }
        index
    }

    fn is_solved_g0(cube: cube::Cube) -> bool {
        cube.edge_orientations == [0; 12]
    }

    fn get_g0_index(cube: cube::Cube) -> u64 {
        Self::orientations_to_index(&cube.edge_orientations, 2) as u64
    }

    pub fn solve_g0(cube: cube::Cube) -> Vec<String> {
        Self::solve_bfs(cube, Self::is_solved_g0, Self::get_g0_index, &Self::G0_MOVES, "solve_g0".to_string())
    }

    pub fn get_g1_index(cube: cube::Cube) -> u64 {
        let corner_orientation_index = Self::orientations_to_index(&cube.corner_orientations, 3);
        let mut lr_mid_slice_permutation = [0; 4];
        for i in 0..4 {
            lr_mid_slice_permutation[i] = cube.edge_permutations[cube::Cube::G2_SLICE_EDGES[i as usize] as usize];
        }
        let lr_mid_slice_combination_index = Self::combinations_to_index(&lr_mid_slice_permutation);
        corner_orientation_index as u64 * 495 + lr_mid_slice_combination_index as u64 // 495: comb(12, 4)
    }

    fn is_solved_g1(cube: cube::Cube) -> bool { // assumes g0 solved
        let g1_index = Self::get_g1_index(cube);
        g1_index == 267 // combination index of [0, 2, 8, 10]
    }

    pub fn solve_g1(cube: cube::Cube) -> Vec<String> {
        return Self::solve_bfs(cube, Self::is_solved_g1, Self::get_g1_index, &Self::G1_MOVES, "solve_g1".to_string());
    }

    fn get_g2_index(cube: cube::Cube) -> u64 {
        let mut ud_mid_slice_permutation = [0; 4];
        for i in 0..4 {
            ud_mid_slice_permutation[i] = cube.edge_permutations[cube::Cube::G3_SLICE_EDGES[i as usize] as usize];
        }
        let ud_mid_slice_combination_index = Self::combinations_to_index(&ud_mid_slice_permutation);
        let mut tetrad_permutation = [0; 4];
        for i in 0..4 {
            tetrad_permutation[i] = cube.corner_permutations[cube::Cube::G3_TETRAD_CORNERS[i as usize] as usize];
        }
        let tetrad_combination_index = Self::combinations_to_index(&tetrad_permutation);
        return tetrad_combination_index as u64 * comb(8, 4) + ud_mid_slice_combination_index as u64;
    }

    fn is_solved_g2(cube: cube::Cube) -> bool {
        return Self::get_g2_index(cube) == 69 * comb(8, 4) + 46;
    }

    pub fn solve_g2(cube: cube::Cube) -> Vec<String> {
        return Self::solve_bfs(cube, Self::is_solved_g2, Self::get_g2_index, &Self::G2_MOVES, "solve_g2".to_string());
    }
}