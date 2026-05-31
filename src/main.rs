use std::{env, io};

mod cube;
mod solver;
mod profile;
mod prune_table;

use crate::cube::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} [sim|solve-rand|solve-fixed|debug]", args[0]);
        return;
    }
    match args[1].as_str() {
        "sim" => {
            let mut cube = cube::Cube::new();
            println!("Enter move sequences, empty line to dump and continue:");
            let mut buffer = String::new();
            while let Ok(_) = io::stdin().read_line(&mut buffer) {
                let line = buffer.trim();
                if line.is_empty() {
                    println!("{}", cube);
                } else {
                    cube = cube.apply_sequence(line);
                    println!("Applied: {}", line);
                    println!("{}", cube);
                }
                buffer.clear();
            }
        }
        "solve-rand" => {
            let (cube, scrambled_moves) = cube::Cube::new().scramble(25);
            println!("Scrambled moves: {}", scrambled_moves.to_string());
            println!("{}", cube);
            solver::ThistlethwaiteSolver::solve(cube, "solve-rand".to_string(), true);
        }
        "solve-fixed" => {
            let moves = cube::Moves(vec![L2, U, F2, DP, F, U, F, D2, BP, F2, RP, BP, U2, RP, D2, R, L, DP, U, D, L2, DP, UP, B2, F]);
            let cube = cube::Cube::new().apply_moves(moves.clone());
            println!("Fixed moves: {}", moves.to_string());
            println!("{}", cube);
            solver::ThistlethwaiteSolver::solve(cube, "solve-fixed".to_string(), true);
        }
        "prune-gen" => {
            prune_table::PruneTable::gen_g1();
            prune_table::PruneTable::gen_g2();
            prune_table::PruneTable::gen_g3();
        }
        "debug" => {
            let cube = cube::Cube::new().apply_moves(cube::Moves(vec![U, F]));
            println!("{}", solver::ThistlethwaiteSolver::get_g1_index(cube));
        }
        _ => {
            println!("Usage: {} [sim|solve-rand|solve-fixed|prune-gen|debug]", args[0]);
        }
    }
}
