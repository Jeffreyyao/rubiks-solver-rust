use std::{env, io};

mod cube;
mod solver;
mod profile;

use crate::cube::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} [sim|solve-rand|debug]", args[0]);
        return;
    }
    if args[1] == "sim" { // simulate moves in a while loop
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
    } else if args[1] == "solve-rand" {
        let (cube, scrambled_moves) = cube::Cube::new().scramble(25);
        println!("Scrambled moves: {}", scrambled_moves.to_string());
        println!("{}", cube);
        solver::Solver::solve(cube, "solve-rand".to_string(), true);
    } else if args[1] == "solve-fixed" {
        let cube = cube::Cube::new().apply_moves(&[R, U, RP, UP, RP, F, R2, UP, RP, UP, R, U, RP, FP]);
        println!("{}", cube);
        solver::Solver::solve(cube, "solve-fixed".to_string(), true);
    } else if args[1] == "debug" {
        println!("{}", solver::Solver::get_cubie_position_combination_index(&[3, 6, 0, 1, 4, 10, 11, 7, 2, 5, 8, 9], &[0, 2, 8, 10]));
    }
}
