use std::{env, io};

mod cube;
mod solver;
mod profile;

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
        // let cube = cube::Cube::new().apply_sequence("dbfur'l'd");
        // let cube = cube::Cube::new().apply_sequence("rur'u'r'frru'r'u'rur'f'");
        let (cube, scrambled_moves) = cube::Cube::new().scramble(20);
        println!("Scrambled moves: {}", scrambled_moves);
        println!("{}", cube);
        let moves_g0 = solver::Solver::solve_g0(cube).join("");
        let cube_g0 = cube.apply_sequence(&moves_g0);
        println!("Moves G0: {}", moves_g0);
        println!("{}", cube_g0);
        let moves_g1 = solver::Solver::solve_g1(cube_g0).join("");
        let cube_g1 = cube_g0.apply_sequence(&moves_g1);
        println!("Moves G1: {}", moves_g1);
        println!("{}", cube_g1);
        let moves_g2 = solver::Solver::solve_g2(cube_g1).join("");
        let cube_g2 = cube_g1.apply_sequence(&moves_g2);
        println!("Moves G2: {}", moves_g2);
        println!("{}", cube_g2);
    } else if args[1] == "debug" {
        let cube = cube::Cube::new_from_indices([0, 0, 0, 0, 0, 0, 0, 1], [0, 1, 2, 3, 4, 5, 6, 7], [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        println!("{}", solver::Solver::get_g1_index(cube));
    }
}
