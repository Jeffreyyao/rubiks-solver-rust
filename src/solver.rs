use std::collections::{HashSet, VecDeque};

use crate::cube::{self, U, UP, U2, D, DP, D2, L, LP, L2, R, RP, R2, F, FP, F2, B, BP, B2};
use crate::profile;

pub struct Solver;

const FACTORIALS: [u64; 13] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800, 39916800, 479001600];

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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum IndexType {
    Combination,
    Permutation,
}

impl Solver {
    pub const G0_MOVES: [cube::Mov; 12] = [U, UP, D, DP, L, LP, R, RP, F, FP, B, BP];
    pub const G1_MOVES: [cube::Mov; 10] = [U2, D2, L, LP, R, RP, F, FP, B, BP];
    pub const G2_MOVES: [cube::Mov; 8] = [U2, D2, L, LP, R, RP, F2, B2];
    pub const G3_MOVES: [cube::Mov; 6] = [U2, D2, L2, R2, F2, B2];

    fn solve_bfs(
        cube: cube::Cube,
        is_solved: fn(cube::Cube) -> bool,
        fn_get_index: fn(cube::Cube) -> u64,
        moves: &[cube::Mov],
        name: String,
    ) -> (bool, cube::Moves) {
        let p = profile::Profile::start(&name);
        if is_solved(cube) {
            p.report("already solved");
            return (true, cube::Moves(vec![]));
        }
        let mut queue = VecDeque::from([(cube, cube::Moves(vec![]))]);
        let mut visited_indices = HashSet::from([fn_get_index(cube)]);
        while let Some((current_cube, current_moves)) = queue.pop_front() {
            if is_solved(current_cube) {
                p.end();
                return (true, current_moves);
            }
            for m in moves {
                if current_cube.prev_move.is_some() && current_cube.prev_move.unwrap().face == m.face {
                    continue; // skip next move of the same face
                }
                let new_cube = current_cube.apply_move(*m);
                let new_index = fn_get_index(new_cube);
                if !visited_indices.contains(&new_index) {
                    visited_indices.insert(new_index);
                    let mut new_moves = current_moves.clone();
                    new_moves.push(*m);
                    queue.push_back((new_cube, new_moves));
                }
            }
        }
        p.report("no solution found");
        return (false, cube::Moves(vec![]));
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

    // helper function to get the combination index of cubie positions in permutation
    pub fn get_cubies_position_index<const N: usize>(permutations: &[u8], cubies: &[u8; N], index_type: IndexType) -> u64 {
        let mut perm = [0u8; N];
        let mut i = 0;
        for position in 0..permutations.len() {
            for cubie in cubies {
                if permutations[position] == *cubie {
                    perm[i] = position as u8;
                    i += 1;
                    break;
                }
            }
        }
        if index_type == IndexType::Combination {
            return Self::combinations_to_index(&perm) as u64;
        } else if index_type == IndexType::Permutation {
            return Self::permutations_to_index(&perm, 8) as u64;
        }
        0
    }

    // helper function to get the permutation index on given positions in permutation
    fn get_cube_permutation_index_at_position(permutations: &[u8], positions: [u8; 4], n: u8) -> u64 {
        let mut perm = [0; 4];
        for i in 0..4 {
            perm[i] = permutations[positions[i] as usize];
        }
        Self::permutations_to_index(&perm, n) as u64
    }

    fn is_solved_g0(cube: cube::Cube) -> bool { // edge orientations are all 0
        cube.edge_orientations == [0; 12]
    }

    fn get_g0_index(cube: cube::Cube) -> u64 {
        Self::orientations_to_index(&cube.edge_orientations, 2) as u64
    }

    pub fn solve_g0(cube: cube::Cube) -> (bool, cube::Moves) {
        Self::solve_bfs(cube, Self::is_solved_g0, Self::get_g0_index, &Self::G0_MOVES, "solve_g0".to_string())
    }

    pub fn get_g1_index(cube: cube::Cube) -> u64 {
        let corner_orientation_index = Self::orientations_to_index(&cube.corner_orientations, 3);
        let lr_mid_slice_combination_index = Self::get_cubies_position_index(&cube.edge_permutations, &cube::Cube::LR_MID_SLICE_EDGES, IndexType::Combination);
        corner_orientation_index as u64 * 495 + lr_mid_slice_combination_index // 495: comb(12, 4)
    }

    fn is_solved_g1(cube: cube::Cube) -> bool { // corner orientations are all 0; LR mid slice combination match
        let g1_index = Self::get_g1_index(cube);
        g1_index == 267 // combination index of [0, 2, 8, 10]
    }

    pub fn solve_g1(cube: cube::Cube) -> (bool, cube::Moves) {
        Self::solve_bfs(cube, Self::is_solved_g1, Self::get_g1_index, &Self::G1_MOVES, "solve_g1".to_string())
    }

    pub fn get_g2_index(cube: cube::Cube) -> u64 {
        let ud_mid_slice_combination_index = Self::get_cubies_position_index(&cube.edge_permutations, &cube::Cube::UD_MID_SLICE_EDGES, IndexType::Combination);
        let ht1_i = Self::get_cubies_position_index(&cube.corner_permutations, &cube::Cube::HALF_TETRAD_1_CORNERS, IndexType::Combination);
        let ht2_i = Self::get_cubies_position_index(&cube.corner_permutations, &cube::Cube::HALF_TETRAD_2_CORNERS, IndexType::Combination);
        let ht3_i = Self::get_cubies_position_index(&cube.corner_permutations, &cube::Cube::HALF_TETRAD_3_CORNERS, IndexType::Combination);
        let ht_size = comb(8, 2);
        return ht1_i + ht_size*(ht2_i + ht_size*(ht3_i + ht_size*ud_mid_slice_combination_index));
    }

    fn is_solved_g2(cube: cube::Cube) -> bool { // first, second & third half-tetrad combination match, ud mid slice combination match
        Self::get_g2_index(cube) == 1518553
    }

    pub fn solve_g2(cube: cube::Cube) -> (bool, cube::Moves) {
        Self::solve_bfs(cube, Self::is_solved_g2, Self::get_g2_index, &Self::G2_MOVES, "solve_g2".to_string())
    }

    pub fn get_g3_index(cube: cube::Cube) -> u64 {
        let e1_index = Self::get_cube_permutation_index_at_position(&cube.edge_permutations, cube::Cube::LR_MID_SLICE_EDGES, 12);
        let e2_index = Self::get_cube_permutation_index_at_position(&cube.edge_permutations, cube::Cube::UD_MID_SLICE_EDGES, 12);
        let e3_index = Self::get_cube_permutation_index_at_position(&cube.edge_permutations, cube::Cube::FB_MID_SLICE_EDGES, 12);
        let c1_index = Self::get_cube_permutation_index_at_position(&cube.corner_permutations, cube::Cube::TETRAD_1_CORNERS, 8);
        let c2_index = Self::get_cube_permutation_index_at_position(&cube.corner_permutations, cube::Cube::TETRAD_2_CORNERS, 8);
        let eperm_size = perm(12,4);
        let cperm_size = perm(8,4);
        return c1_index + cperm_size*(c2_index + cperm_size*(e1_index + eperm_size*(e2_index + eperm_size*e3_index)));
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

    pub fn solve_g3(cube: cube::Cube) -> (bool, cube::Moves) {
        Self::solve_bfs(cube, Self::is_solved_g3, Self::get_g3_index, &Self::G3_MOVES, "solve_g3".to_string())
    }

    pub fn solve(cube: cube::Cube, name: String, print_moves: bool) -> (bool, cube::Moves) {
        let p = profile::Profile::start(&name);
        let mut moves = cube::Moves(vec![]);

        let (g0_success, moves_g0) = Self::solve_g0(cube.clone());
        let cube_g0 = cube.apply_moves(moves_g0.clone());
        moves.extend(moves_g0.clone());
        if print_moves {
            println!("G0 Moves: {}", moves_g0.to_string());
            println!("{}", cube_g0);
        }
        if !g0_success {
            p.report("no solution found for G0");
            return (false, moves);
        }

        let (g1_success, moves_g1) = Self::solve_g1(cube_g0.clone());
        let cube_g1 = cube_g0.apply_moves(moves_g1.clone());
        moves.extend(moves_g1.clone());
        if print_moves {
            println!("G1 Moves: {}", moves_g1.to_string());
            println!("{}", cube_g1);
        }
        if !g1_success {
            p.report("no solution found for G1");
            return (false, moves);
        }

        let (g2_success, moves_g2) = Self::solve_g2(cube_g1.clone());
        let cube_g2 = cube_g1.apply_moves(moves_g2.clone());
        moves.extend(moves_g2.clone());
        if print_moves {
            println!("G2 Moves: {}", moves_g2.to_string());
            println!("{}", cube_g2);
        }
        if !g2_success {
            p.report("no solution found for G2");
            return (false, moves);
        }

        let (g3_success, moves_g3) = Self::solve_g3(cube_g2.clone());
        let cube_g3 = cube_g2.apply_moves(moves_g3.clone());
        moves.extend(moves_g3.clone());
        if print_moves {
            println!("G3 Moves: {}", moves_g3.to_string());
            println!("{}", cube_g3);
        }
        if !g3_success {
            p.report("no solution found for G3");
            return (false, moves);
        }

        p.end();
        if print_moves { println!("Full solution ({} moves): {}", moves.0.len(), moves.to_string()); }
        return (true, moves);
    }
}
