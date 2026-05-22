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

pub fn perm(n: u64, r: u64) -> u64 {
    if n < r {
        return 0;
    }
    FACTORIALS[n as usize] / FACTORIALS[(n - r) as usize]
}

impl Solver {
    pub const G0_MOVES: [&str; 12] = ["u", "u'", "d", "d'", "l", "l'", "r", "r'", "f", "f'", "b", "b'"];
    pub const G1_MOVES: [&str; 10] = ["u2", "d2", "l", "l'", "r", "r'", "f", "f'", "b", "b'"];
    pub const G2_MOVES: [&str; 8] = ["u2", "d2", "l", "l'", "r", "r'", "f2", "b2"];
    pub const G3_MOVES: [&str; 6] = ["u2", "d2", "l2", "r2", "f2", "b2"];

    fn solve_bfs(
        cube: cube::Cube,
        is_solved: fn(cube::Cube) -> bool,
        fn_get_index: fn(cube::Cube) -> u64,
        moves: &[&str],
        name: String,
    ) -> Vec<String> {
        let p = profile::Profile::start(&name);
        if is_solved(cube) {
            p.report("already solved");
            return vec![];
        }
        let mut queue = VecDeque::from([(cube, vec![])]);
        let mut visited_indices = HashSet::from([fn_get_index(cube)]);
        while let Some((current_cube, current_moves)) = queue.pop_front() {
            if is_solved(current_cube) {
                p.end();
                return current_moves;
            }
            for m in moves {
                if current_cube.last_move.is_some() && current_cube.last_move.unwrap().face == cube::Cube::char_to_face(m.chars().next().unwrap()).unwrap() {
                    continue; // skip next move of the same face
                }
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

    pub fn permutations_to_index(permutations: &[u8], n: u8) -> u32 { // n: size of all allowed elements
        let k = permutations.len() as u8;
        let mut index: u32 = 0;
        let mut used = vec![false; 12];

        for i in 0..k as usize {
            let p = permutations[i];
            let mut count = 0u32;
            for j in 0..p {
                if !used[j as usize] {
                    count += 1;
                }
            }
            let remaining_after_this = n - i as u8 - 1;
            let still_to_choose = k - i as u8 - 1;
            let multiplier = if still_to_choose == 0 {
                1
            } else {
                perm(remaining_after_this as u64, still_to_choose as u64) as u32
            };
            index += count * multiplier as u32;
            used[p as usize] = true;
        }

        index
    }

    // helper function to get the combination index of a cube given on positions of 4
    fn get_cube_combination_index_at_position<const N: usize>(combinations: &[u8], position: &[u8; N]) -> u64 {
        let mut comb = [0u8; N];
        for i in 0..N {
            comb[i] = combinations[position[i] as usize];
        }
        Self::combinations_to_index(&comb) as u64
    }

    // helper function to get the permutation index of a cube given on positions of 4
    fn get_cube_permutation_index_at_position(permutations: &[u8], position: [u8; 4]) -> u64 {
        let mut perm = [0; 4];
        for i in 0..4 {
            perm[i] = permutations[position[i] as usize];
        }
        Self::permutations_to_index(&perm, 8) as u64
    }

    fn is_solved_g0(cube: cube::Cube) -> bool { // edge orientations are all 0
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
            lr_mid_slice_permutation[i] = cube.edge_permutations[cube::Cube::LR_MID_SLICE_EDGES[i as usize] as usize];
        }
        let lr_mid_slice_combination_index = Self::combinations_to_index(&lr_mid_slice_permutation);
        corner_orientation_index as u64 * 495 + lr_mid_slice_combination_index as u64 // 495: comb(12, 4)
    }

    fn is_solved_g1(cube: cube::Cube) -> bool { // corner orientations are all 0; LR mid slice combination is [0, 2, 8, 10]
        let g1_index = Self::get_g1_index(cube);
        g1_index == 267 // combination index of [0, 2, 8, 10]
    }

    pub fn solve_g1(cube: cube::Cube) -> Vec<String> {
        return Self::solve_bfs(cube, Self::is_solved_g1, Self::get_g1_index, &Self::G1_MOVES, "solve_g1".to_string());
    }

    pub fn get_g2_index(cube: cube::Cube) -> u64 {
        let ud_mid_slice_combination_index = Self::get_cube_combination_index_at_position(&cube.edge_permutations, &cube::Cube::UD_MID_SLICE_EDGES);
        let ht1_i = Self::get_cube_combination_index_at_position(&cube.corner_permutations, &cube::Cube::HALF_TETRAD_1_CORNERS);
        let ht2_i = Self::get_cube_combination_index_at_position(&cube.corner_permutations, &cube::Cube::HALF_TETRAD_2_CORNERS);
        let ht3_i = Self::get_cube_combination_index_at_position(&cube.corner_permutations, &cube::Cube::HALF_TETRAD_3_CORNERS);
        let ht_size = comb(8, 2);
        return ht1_i + ht_size*(ht2_i + ht_size*(ht3_i + ht_size*ud_mid_slice_combination_index));
    }

    fn is_solved_g2(cube: cube::Cube) -> bool { // first tetrad is [0, 2, 5, 7]; UD mid slice combination is [4, 5, 6, 7]
        Self::get_g2_index(cube) == 1518553
    }

    pub fn solve_g2(cube: cube::Cube) -> Vec<String> {
        return Self::solve_bfs(cube, Self::is_solved_g2, Self::get_g2_index, &Self::G2_MOVES, "solve_g2".to_string());
    }

    pub fn get_g3_index(cube: cube::Cube) -> u64 {
        let e1_index = Self::get_cube_permutation_index_at_position(&cube.edge_permutations, cube::Cube::LR_MID_SLICE_EDGES);
        let e2_index = Self::get_cube_permutation_index_at_position(&cube.edge_permutations, cube::Cube::UD_MID_SLICE_EDGES);
        let e3_index = Self::get_cube_permutation_index_at_position(&cube.edge_permutations, cube::Cube::FB_MID_SLICE_EDGES);
        let c1_index = Self::get_cube_permutation_index_at_position(&cube.corner_permutations, cube::Cube::TETRAD_1_CORNERS);
        let c2_index = Self::get_cube_permutation_index_at_position(&cube.corner_permutations, cube::Cube::TETRAD_2_CORNERS);
        let perm_size = perm(4,4);
        return e1_index + perm_size*(e2_index + perm_size*(e3_index + perm_size*(c1_index + perm_size*c2_index)));
    }

    fn is_solved_g3(cube: cube::Cube) -> bool {
        for i in 0..8 {
            if cube.corner_permutations[i] != i as u8 { return false }
        }
        for i in 0..12 {
            if cube.edge_permutations[i] != i as u8 { return false }
        }
        true
    }

    pub fn solve_g3(cube: cube::Cube) -> Vec<String> {
        return Self::solve_bfs(cube, Self::is_solved_g3, Self::get_g3_index, &Self::G3_MOVES, "solve_g3".to_string());
    }
}