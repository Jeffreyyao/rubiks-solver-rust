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
        let p = profile::Profile::start("solve-rand");
        let (cube, scrambled_moves) = cube::Cube::new().scramble(25);
        println!("Scrambled moves: {}", scrambled_moves.to_string());
        println!("{}", cube);

        let (g0_success, moves_g0) = solver::Solver::solve_g0(cube);
        let cube_g0 = cube.apply_moves(&moves_g0.0);
        println!("Moves G0: {}", moves_g0.to_string());
        println!("{}", cube_g0);
        if !g0_success { return; }

        let (g1_success, moves_g1) = solver::Solver::solve_g1(cube_g0);
        let cube_g1 = cube_g0.apply_moves(&moves_g1.0);
        println!("Moves G1: {}", moves_g1.to_string());
        println!("{}", cube_g1);
        if !g1_success { return; }

        let (g2_success, moves_g2) = solver::Solver::solve_g2(cube_g1);
        let cube_g2 = cube_g1.apply_moves(&moves_g2.0);
        println!("Moves G2: {}", moves_g2.to_string());
        println!("{}", cube_g2);
        if !g2_success { return; }

        let (g3_success, moves_g3) = solver::Solver::solve_g3(cube_g2);
        let cube_g3 = cube_g2.apply_moves(&moves_g3.0);
        println!("Moves G3: {}", moves_g3.to_string());
        println!("{}", cube_g3);
        if !g3_success { return; }

        p.end();
        println!("Full solution: {} {} {} {}", moves_g0.to_string(), moves_g1.to_string(), moves_g2.to_string(), moves_g3.to_string());
    } else if args[1] == "debug" {
        let c = cube::Cube::new();
        println!("{}", solver::Solver::get_g1_index(c));
    }
}
