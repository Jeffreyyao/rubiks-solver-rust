use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

use crate::cube;
use crate::solver;

pub struct Table {
    entries: HashMap<u32, u32>
}

impl Table {
    fn new(filename: &str) -> Self {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut entries = HashMap::new();
        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split_whitespace().collect();
            let key = parts[0].parse::<u32>().unwrap();
            let value = parts[1].parse::<u32>().unwrap();
            entries.insert(key, value);
        }
        Self {
            entries,
        }
    }

    fn new_empty() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    fn get(&self, key: u32) -> Option<u32> {
        self.entries.get(&key).copied()
    }

    fn set(&mut self, key: u32, value: u32) {
        self.entries.insert(key, value);
    }

    pub fn save(&self, filename: &str) {
        let file = File::create(filename).unwrap();
        let mut writer = BufWriter::new(file);
        for (key, value) in &self.entries {
            writeln!(writer, "{} {}", key, value).unwrap();
        }
    }
}

pub struct HeuristicTable {
    tables: HashMap<String, Table>
}

impl HeuristicTable {
    pub fn build_g1_heuristics() -> Table {
        let mut table = Table::new_empty();
        let cube = cube::Cube::new();
        let moves = solver::Solver::G1_MOVES;
        let mut queue: VecDeque<(cube::Cube, Vec<String>)> = VecDeque::from([(cube, vec![])]);
        let mut visited_corner_orientations = HashSet::from([solver::Solver::orientations_to_index(&cube.corner_orientations, 3)]);
        let mut visited_edge_permutations = HashSet::from([solver::Solver::get_g1_index(cube)]);
        
        table
    }
}