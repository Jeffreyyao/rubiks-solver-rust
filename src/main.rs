use std::{env, io};

mod cube;
mod solver;
mod profile;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} [sim|debug]", args[0]);
        return;
    }
    if args[1] == "sim" { // simulate moves in a while loop
        let mut cube = cube::Cube::new();
        println!("Enter move sequences, empty line to dump and continue:");
        let mut buffer = String::new();
        while let Ok(_) = io::stdin().read_line(&mut buffer) {
            let line = buffer.trim();
            if line.is_empty() {
                cube.dump();
            } else {
                cube = cube.apply_sequence(line);
                println!("Applied: {}", line);
                cube.dump();
            }
            buffer.clear();
        }
    } else if args[1] == "debug" {
        // let cube = cube::Cube::new().apply_sequence("dbfur'l'd");
        // let cube = cube::Cube::new().apply_sequence("rur'u'r'frru'r'u'rur'f'");
        let (cube, scrambled_moves) = cube::Cube::new().scramble(20);
        println!("Scrambled moves: {}", scrambled_moves);
        cube.dump();
        let moves_g0 = solver::Solver::solve_g0(cube).join("");
        let cube_g0 = cube.apply_sequence(&moves_g0);
        println!("Moves G0: {}", moves_g0);
        cube_g0.dump();
        let moves_g1 = solver::Solver::solve_g1(cube_g0).join("");
        let cube_g1 = cube_g0.apply_sequence(&moves_g1);
        println!("Moves G1: {}", moves_g1);
        cube_g1.dump();
    }
}
