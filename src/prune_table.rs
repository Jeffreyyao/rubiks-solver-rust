use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

use crate::cube;
use crate::solver;

#[derive(Clone, Debug)]
pub struct PruneTable {
    max_depth: u8,
    table: HashMap<u64, u8>, // maps index to depth
}

pub const PRUNE_TABLE_G1_FILENAME: &str = "prune_tables/thistlethwaite_g1.txt";
pub const PRUNE_TABLE_G2_FILENAME: &str = "prune_tables/thistlethwaite_g2.txt";
pub const PRUNE_TABLE_G3_FILENAME: &str = "prune_tables/thistlethwaite_g3.txt";
pub const PRUNE_TABLE_PHASE1_FILENAME: &str = "prune_tables/kociemba_phase1.txt";

impl PruneTable {
    pub fn new() -> Self {
        Self {
            max_depth: 0,
            table: HashMap::new(),
        }
    }

    pub fn get_max_depth(&self) -> u8 {
        self.max_depth
    }

    pub fn set_max_depth(&mut self, max_depth: u8) {
        self.max_depth = max_depth;
    }

    pub fn insert(&mut self, index: u64, depth: u8) {
        self.table.insert(index, depth);
    }

    pub fn get(&self, index: u64) -> Option<u8> {
        Some(self.max_depth - self.table.get(&index).cloned().unwrap_or(0))
    }

    pub fn save(&self, filename: &str) {
        let file = File::create(filename).unwrap();
        let mut writer = BufWriter::new(file);
        writeln!(writer, "{}", self.max_depth).unwrap();
        for (index, depth) in self.table.iter() {
            writeln!(writer, "{} {}", index, depth).unwrap();
        }
    }

    pub fn load(filename: &str) -> Self {
        let mut table = Self::new();
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        let first_line = lines.next().unwrap().unwrap();
        table.max_depth = first_line.trim().parse().unwrap();
        for line in lines {
            let line = line.unwrap();
            let mut parts = line.split_whitespace();
            let index: u64 = parts.next().unwrap().parse().unwrap();
            let depth: u8 = parts.next().unwrap().parse().unwrap();
            table.insert(index, depth);
        }
        table
    }

    pub fn gen_g1() {
        let cube = cube::Cube::new();
        let table = solver::Solver::gen_prune_table_g1(cube);
        table.save(PRUNE_TABLE_G1_FILENAME);
    }

    pub fn load_g1() -> Self {
        Self::load(PRUNE_TABLE_G1_FILENAME)
    }

    pub fn gen_g2() {
        let cube = cube::Cube::new();
        let table = solver::Solver::gen_prune_table_g2(cube);
        table.save(PRUNE_TABLE_G2_FILENAME);
    }

    pub fn load_g2() -> Self {
        Self::load(PRUNE_TABLE_G2_FILENAME)
    }
    
    pub fn gen_g3() {
        let cube = cube::Cube::new();
        let table = solver::Solver::gen_prune_table_g3(cube);
        table.save(PRUNE_TABLE_G3_FILENAME);
    }

    pub fn load_g3() -> Self {
        Self::load(PRUNE_TABLE_G3_FILENAME)
    }

    pub fn gen_phase1() {
        let cube = cube::Cube::new();
        let table = solver::Solver::gen_prune_table_phase1(cube);
        table.save(PRUNE_TABLE_PHASE1_FILENAME);
    }

    pub fn load_phase1() -> Self {
        Self::load(PRUNE_TABLE_PHASE1_FILENAME)
    }
}