use std::collections::{HashSet, VecDeque};

use crate::cube::{self, U, UP, U2, D, DP, D2, L, LP, L2, R, RP, R2, F, FP, F2, B, BP, B2};
use crate::profile;
use crate::prune_table::PruneTable;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum SolveMode {
    Bfs,
    PruneGen,
    Prune,
}

const FACTORIALS: [u64; 13] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800, 39916800, 479001600];

pub fn comb(n: u64, r: u64) -> u64 {
    if n < r {
        return 0;
    } 
    FACTORIALS[n as usize] / (FACTORIALS[r as usize] * FACTORIALS[(n - r) as usize])
}

pub struct ThistlethwaiteSolver;

impl ThistlethwaiteSolver {
    pub const G0_MOVES: [cube::Mov; 12] = [U, UP, D, DP, L, LP, R, RP, F, FP, B, BP];
    pub const G1_MOVES: [cube::Mov; 10] = [U2, D2, L, LP, R, RP, F, FP, B, BP];
    pub const G2_MOVES: [cube::Mov; 8] = [U2, D2, L, LP, R, RP, F2, B2];
    pub const G3_MOVES: [cube::Mov; 6] = [U2, D2, L2, R2, F2, B2];

    pub fn solve_group(
        name: String,
        mode: SolveMode,
        cube: cube::Cube,
        is_solved: Option<fn(cube::Cube) -> bool>,
        fn_get_index: fn(cube::Cube) -> u64,
        moves: &[cube::Mov],
        mut prune_table: Option<&mut PruneTable>,
    ) -> (bool, cube::Moves) {
        let p = profile::Profile::start(&name);
        if is_solved.is_some() && is_solved.unwrap()(cube) {
            p.report("already solved");
            return (true, cube::Moves(vec![]));
        }

        let mut queue = VecDeque::from([(cube, cube::Moves(vec![]))]);
        let mut visited_indices = HashSet::from([]);
        let mut pruned_cnt = 0;

        while let Some((current_cube, current_moves)) = queue.pop_front() {
            if mode != SolveMode::PruneGen && is_solved.is_some() && is_solved.unwrap()(current_cube) {
                if mode == SolveMode::Prune {
                    p.report(&format!("solved, pruned cnt: {}", pruned_cnt));
                } else {
                    p.end();
                }
                return (true, current_moves);
            }
            let current_index = fn_get_index(current_cube);
            if visited_indices.contains(&current_index) {
                continue;
            }
            if mode == SolveMode::Prune { // prune
                if let Some(table) = prune_table.as_mut() {
                    if let Some(depth) = table.get(current_index) {
                        if current_moves.0.len() as u8 >= depth {
                            pruned_cnt += 1;
                            continue;
                        }
                    }
                }
            }
            visited_indices.insert(current_index);

            if mode == SolveMode::PruneGen { // prune generation
                if let Some(table) = prune_table.as_mut() {
                    table.insert(current_index, current_moves.0.len() as u8);
                    if current_moves.0.len() as u8 > table.get_max_depth() {
                        table.set_max_depth(current_moves.0.len() as u8);
                        p.report(&format!("max depth: {}", table.get_max_depth()));
                    }
                }
            }

            for m in moves {
                if current_cube.prev_move.is_some() && current_cube.prev_move.unwrap().face == m.face {
                    continue; // skip next move of the same face
                }
                let new_cube = current_cube.apply_move(*m);
                let mut new_moves = current_moves.clone();
                new_moves.push(*m);
                queue.push_back((new_cube, new_moves));
            }
        }

        if mode == SolveMode::PruneGen {
            p.report(&format!("prune table generated, visited states len: {}", visited_indices.len()));
        } else {
            p.report(&format!("no solution found, visited states len: {}", visited_indices.len()));
        }
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

    pub fn permutations_to_index(permutations: &[u8]) -> u64 { // n: size of all allowed elements
        let mut index = 0;
        let n = permutations.len();
        for i in 0..n {
            let mut count = 0;
            for j in i+1..n {
                if permutations[j] < permutations[i] {
                    count += 1;
                }
            }
            index += count * FACTORIALS[(n - i - 1) as usize];
        }
        index
    }

    // helper function to get the combination index of cubie positions in permutation
    pub fn get_cubies_position_index<const N: usize>(permutations: &[u8], cubies: &[u8; N]) -> u64 {
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
        Self::combinations_to_index(&perm) as u64
    }

    fn is_solved_g0(cube: cube::Cube) -> bool { // edge orientations are all 0
        cube.edge_orientations == [0; 12]
    }

    fn get_g0_index(cube: cube::Cube) -> u64 {
        Self::orientations_to_index(&cube.edge_orientations, 2) as u64
    }

    pub fn solve_g0(cube: cube::Cube) -> (bool, cube::Moves) {
        Self::solve_group("solve_g0".to_string(), SolveMode::Bfs, cube, Some(Self::is_solved_g0), Self::get_g0_index, &Self::G0_MOVES, None)
    }

    pub fn get_g1_index(cube: cube::Cube) -> u64 {
        let corner_orientation_index = Self::orientations_to_index(&cube.corner_orientations, 3);
        let lr_mid_slice_combination_index = Self::get_cubies_position_index(&cube.edge_permutations, &cube::Cube::LR_MID_SLICE_EDGES);
        corner_orientation_index as u64 * 495 + lr_mid_slice_combination_index // 495: comb(12, 4)
    }

    fn is_solved_g1(cube: cube::Cube) -> bool { // corner orientations are all 0; LR mid slice combination match
        let g1_index = Self::get_g1_index(cube);
        g1_index == 267 // combination index of [0, 2, 8, 10]
    }

    pub fn solve_g1(cube: cube::Cube) -> (bool, cube::Moves) {
        let mut prune_table = PruneTable::load_g1();
        Self::solve_group("solve_g1".to_string(), SolveMode::Prune, cube, Some(Self::is_solved_g1), Self::get_g1_index, &Self::G1_MOVES, Some(&mut prune_table))
    }

    pub fn get_g2_index(cube: cube::Cube) -> u64 {
        let ud_mid_slice_combination_index = Self::get_cubies_position_index(&cube.edge_permutations, &cube::Cube::UD_MID_SLICE_EDGES);
        let ht1_i = Self::get_cubies_position_index(&cube.corner_permutations, &cube::Cube::HALF_TETRAD_1_CORNERS);
        let ht2_i = Self::get_cubies_position_index(&cube.corner_permutations, &cube::Cube::HALF_TETRAD_2_CORNERS);
        let ht3_i = Self::get_cubies_position_index(&cube.corner_permutations, &cube::Cube::HALF_TETRAD_3_CORNERS);
        let ht_size = comb(8, 2);
        return ht1_i + ht_size*(ht2_i + ht_size*(ht3_i + ht_size*ud_mid_slice_combination_index));
    }

    fn is_solved_g2(cube: cube::Cube) -> bool { // first, second & third half-tetrad combination match, ud mid slice combination match
        Self::get_g2_index(cube) == 1518553
    }

    pub fn solve_g2(cube: cube::Cube) -> (bool, cube::Moves) {
        let mut prune_table = PruneTable::load_g2();
        Self::solve_group("solve_g2".to_string(), SolveMode::Prune, cube, Some(Self::is_solved_g2), Self::get_g2_index, &Self::G2_MOVES, Some(&mut prune_table))
    }

    pub fn get_g3_index(cube: cube::Cube) -> u64 {
        let e_perm_index = Self::permutations_to_index(&cube.edge_permutations);
        let c_perm_index = Self::permutations_to_index(&cube.corner_permutations);
        return c_perm_index + FACTORIALS[8] * e_perm_index;
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
        let mut prune_table = PruneTable::load_g3();
        Self::solve_group("solve_g3".to_string(), SolveMode::Prune, cube, Some(Self::is_solved_g3), Self::get_g3_index, &Self::G3_MOVES, Some(&mut prune_table))
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

    pub fn gen_prune_table_g1(cube: cube::Cube) -> PruneTable {
        let mut table = PruneTable::new();
        Self::solve_group("gen_prune_table_g1".to_string(), SolveMode::PruneGen, cube, None, Self::get_g1_index, &Self::G1_MOVES, Some(&mut table));
        table
    }

    pub fn gen_prune_table_g2(cube: cube::Cube) -> PruneTable {
        let mut table = PruneTable::new();
        Self::solve_group("gen_prune_table_g2".to_string(), SolveMode::PruneGen, cube, None, Self::get_g2_index, &Self::G2_MOVES, Some(&mut table));
        table
    }

    pub fn gen_prune_table_g3(cube: cube::Cube) -> PruneTable {
        let mut table = PruneTable::new();
        Self::solve_group("gen_prune_table_g3".to_string(), SolveMode::PruneGen, cube, None, Self::get_g3_index, &Self::G3_MOVES, Some(&mut table));
        table
    }
}
