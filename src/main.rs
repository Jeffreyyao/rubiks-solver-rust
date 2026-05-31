use std::{env, io};

mod cube;
mod solver;
mod profile;

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
            solver::Solver::solve(cube, "solve-rand".to_string(), true);
        }
        "solve-fixed" => {
            let moves = cube::Moves(vec![L2, U, F2, DP, F, U, F, D2, BP, F2, RP, BP, U2, RP, D2, R, L, DP, U, D, L2, DP, UP, B2, F]);
            let cube = cube::Cube::new().apply_moves(moves.clone());
            println!("Fixed moves: {}", moves.to_string());
            println!("{}", cube);
            solver::Solver::solve(cube, "solve-fixed".to_string(), true);
        }
        "debug" => {
            println!("{}", solver::Solver::get_cubies_position_index(&[3, 6, 0, 1, 4, 10, 11, 7, 2, 5, 8, 9], &[0, 2, 8, 10]));
        }
        _ => {
            println!("Usage: {} [sim|solve-rand|solve-fixed|debug]", args[0]);
        }
    }
}
