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

const FACTORIALS: [u32; 13] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800, 39916800, 479001600];

pub fn comb(n: u32, r: u32) -> u32 {
    if n < r {
        return 0;
    } 
    FACTORIALS[n as usize] / (FACTORIALS[r as usize] * FACTORIALS[(n - r) as usize])
}

pub fn perm(n: u32, r: u32) -> u32 {
    if n < r {
        return 0;
    } 
    FACTORIALS[n as usize] / FACTORIALS[(n - r) as usize]
}

pub struct Solver;

impl Solver { // Thistlethwaite solver
    pub const G0_MOVES: [cube::Mov; 18] = [U, UP, U2, D, DP, D2, L, LP, L2, R, RP, R2, F, FP, F2, B, BP, B2];
    pub const G1_MOVES: [cube::Mov; 14] = [U2, D2, L, LP, L2, R, RP, R2, F, FP, F2, B, BP, B2];
    pub const G2_MOVES: [cube::Mov; 10] = [U2, D2, L, LP, L2, R, RP, R2, F2, B2];
    pub const G3_MOVES: [cube::Mov; 6] = [U2, D2, L2, R2, F2, B2];

    pub fn solve_group(
        name: String,
        mode: SolveMode,
        cube: cube::Cube,
        is_solved: Option<fn(cube::Cube) -> bool>,
        fn_get_index: fn(cube::Cube) -> u32,
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
            index += comb(c[i] as u32, (i + 1) as u32) as u32;
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
                perm(remaining_after_this as u32, still_to_choose as u32) as u32
            };
            index += count * multiplier as u32;
            used[p as usize] = true;
        }
        index
    }

    // helper function to get the combination index of cubie positions in permutation
    pub fn get_cubies_position_index<const N: usize>(permutations: &[u8], cubies: &[u8; N]) -> u32 {
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
        Self::combinations_to_index(&perm) as u32
    }

    // helper function to get the permutation index on given positions in permutation
    fn get_cube_permutation_index_at_position(permutations: &[u8], positions: [u8; 4]) -> u32 {
        let mut perm = [0; 4];
        for i in 0..4 {
            perm[i] = permutations[positions[i] as usize];
        }
        Self::permutations_to_index(&perm, 8)
    }

    fn is_solved_g0(cube: cube::Cube) -> bool { // edge orientations are all 0
        cube.edge_orientations == [0; 12]
    }

    fn get_g0_index(cube: cube::Cube) -> u32 {
        Self::orientations_to_index(&cube.edge_orientations, 2) as u32
    }

    pub fn solve_g0(cube: cube::Cube) -> (bool, cube::Moves) {
        Self::solve_group("solve_g0".to_string(), SolveMode::Bfs, cube, Some(Self::is_solved_g0), Self::get_g0_index, &Self::G0_MOVES, None)
    }

    pub fn get_g1_index(cube: cube::Cube) -> u32 {
        let corner_orientation_index = Self::orientations_to_index(&cube.corner_orientations, 3);
        let lr_slice_combination_index = Self::get_cubies_position_index(&cube.edge_permutations, &cube::Cube::LR_SLICE_EDGES);
        corner_orientation_index as u32 * 495 + lr_slice_combination_index // 495: comb(12, 4)
    }

    fn is_solved_g1(cube: cube::Cube) -> bool { // corner orientations are all 0; LR mid slice combination match
        let g1_index = Self::get_g1_index(cube);
        g1_index == 267 // combination index of [0, 2, 8, 10]
    }

    pub fn solve_g1(cube: cube::Cube) -> (bool, cube::Moves) {
        let mut prune_table = PruneTable::load_g1();
        Self::solve_group("solve_g1".to_string(), SolveMode::Prune, cube, Some(Self::is_solved_g1), Self::get_g1_index, &Self::G1_MOVES, Some(&mut prune_table))
    }

    pub fn get_g2_index(cube: cube::Cube) -> u32 {
        let ud_slice_comb_index = Self::get_cubies_position_index(&cube.edge_permutations, &cube::Cube::UD_SLICE_EDGES);
        let ht1_i = Self::get_cubies_position_index(&cube.corner_permutations, &cube::Cube::HALF_TETRAD_1_CORNERS);
        let ht2_i = Self::get_cubies_position_index(&cube.corner_permutations, &cube::Cube::HALF_TETRAD_2_CORNERS);
        let ht3_i = Self::get_cubies_position_index(&cube.corner_permutations, &cube::Cube::HALF_TETRAD_3_CORNERS);
        let ht_size = comb(8, 2);
        return ht1_i + ht_size*(ht2_i + ht_size*(ht3_i + ht_size*ud_slice_comb_index));
    }

    fn is_solved_g2(cube: cube::Cube) -> bool { // first, second & third half-tetrad combination match, ud mid slice combination match
        Self::get_g2_index(cube) == 1518553
    }

    pub fn solve_g2(cube: cube::Cube) -> (bool, cube::Moves) {
        let mut prune_table = PruneTable::load_g2();
        Self::solve_group("solve_g2".to_string(), SolveMode::Prune, cube, Some(Self::is_solved_g2), Self::get_g2_index, &Self::G2_MOVES, Some(&mut prune_table))
    }

    pub fn get_g3_index(cube: cube::Cube) -> u32 {
        let e1_index = Self::get_cube_permutation_index_at_position(&cube.edge_permutations, cube::Cube::LR_SLICE_EDGES);
        let e2_index = Self::get_cube_permutation_index_at_position(&cube.edge_permutations, cube::Cube::UD_SLICE_EDGES);
        let e3_index = Self::get_cube_permutation_index_at_position(&cube.edge_permutations, cube::Cube::FB_SLICE_EDGES);
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

    pub fn solve_g3(cube: cube::Cube) -> (bool, cube::Moves) {
        let mut prune_table = PruneTable::load_g3();
        Self::solve_group("solve_g3".to_string(), SolveMode::Prune, cube, Some(Self::is_solved_g3), Self::get_g3_index, &Self::G3_MOVES, Some(&mut prune_table))
    }

    pub fn solve_thistlethwaite(cube: cube::Cube, name: String, print_moves: bool) -> (bool, cube::Moves) {
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

impl Solver { // Kociemba solver
    pub const PHASE1_MOVES: [cube::Mov; 18] = [U, UP, U2, D, DP, D2, L, LP, L2, R, RP, R2, F, FP, F2, B, BP, B2];

    pub fn get_phase1_index(cube: cube::Cube) -> u32 {
        let corner_orientation_index = Self::orientations_to_index(&cube.corner_orientations, 3);
        let edge_orientation_index = Self::orientations_to_index(&cube.edge_orientations, 2);
        let ud_slice_comb_index = Self::get_cubies_position_index(&cube.edge_permutations, &cube::Cube::UD_SLICE_EDGES);
        ud_slice_comb_index + comb(12, 4) * (edge_orientation_index as u32 + 4096 * corner_orientation_index as u32)
    }

    pub fn is_solved_phase1(cube: cube::Cube) -> bool {
        Self::get_phase1_index(cube) == 132234
    }

    pub fn solve_phase1(cube: cube::Cube) -> (bool, cube::Moves) {
        Self::solve_group("solve_phase1".to_string(), SolveMode::Bfs, cube, Some(Self::is_solved_phase1), Self::get_phase1_index, &Self::PHASE1_MOVES, None)
    }

    pub fn solve_kociemba(cube: cube::Cube, name: String, print_moves: bool) -> (bool, cube::Moves) {
        let p = profile::Profile::start(&name);

        let (phase1_success, moves_phase1) = Self::solve_phase1(cube.clone());
        let cube_phase1 = cube.apply_moves(moves_phase1.clone());
        if print_moves {
            println!("Phase1 Moves: {}", moves_phase1.to_string());
            println!("{}", cube_phase1);
        }

        p.end();
        (phase1_success, moves_phase1)
    }

    pub fn gen_prune_table_phase1(cube: cube::Cube) -> PruneTable {
        let mut table = PruneTable::new();
        Self::solve_group("gen_prune_table_phase1".to_string(), SolveMode::PruneGen, cube, None, Self::get_phase1_index, &Self::PHASE1_MOVES, Some(&mut table));
        table
    }
}
