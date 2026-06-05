use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Read, Write};

use crate::cube;
use crate::solver;

#[derive(Clone, Debug)]
pub struct PruneTable {
    max_depth: u8,
    table: HashMap<u32, u8>, // maps index to depth
}

pub const PRUNE_TABLE_G1_FILENAME: &str = "prune_tables/thistlethwaite_g1";
pub const PRUNE_TABLE_G2_FILENAME: &str = "prune_tables/thistlethwaite_g2";
pub const PRUNE_TABLE_G3_FILENAME: &str = "prune_tables/thistlethwaite_g3";
pub const PRUNE_TABLE_PHASE1_FILENAME: &str = "prune_tables/kociemba_phase1";

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

    pub fn insert(&mut self, index: u32, depth: u8) {
        self.table.insert(index, depth);
    }

    pub fn get(&self, index: u32) -> Option<u8> {
        Some(self.max_depth - self.table.get(&index).cloned().unwrap_or(0))
    }

    pub fn save(&self, filename: &str) -> Result<(), std::io::Error> {
        let file = File::create(filename)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(&(self.max_depth.to_le_bytes()))?;
        for (index, depth) in self.table.iter() {
            writer.write_all(&(index.to_le_bytes()))?;
            writer.write_all(&(depth.to_le_bytes()))?;
        }
        Ok(())
    }

    pub fn load(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut table = Self::new();
        let mut file = File::open(filename)?;
        let mut max_depth_bytes: [u8; 1] = [0; 1];
        file.read_exact(&mut max_depth_bytes)?;
        table.max_depth = max_depth_bytes[0];
        let mut mapping_bytes: [u8; 5] = [0; 5];
        while let Ok(()) = file.read_exact(&mut mapping_bytes) {
            let index = u32::from_le_bytes(mapping_bytes[..4].try_into()?);
            let depth = mapping_bytes[4];
            table.insert(index, depth);
        }
        Ok(table)
    }

    pub fn gen_g1() {
        let cube = cube::Cube::new();
        let table = solver::Solver::gen_prune_table_g1(cube);
        let _ = table.save(PRUNE_TABLE_G1_FILENAME);
    }

    pub fn load_g1() -> Self {
        Self::load(PRUNE_TABLE_G1_FILENAME).unwrap()
    }

    pub fn gen_g2() {
        let cube = cube::Cube::new();
        let table = solver::Solver::gen_prune_table_g2(cube);
        let _ = table.save(PRUNE_TABLE_G2_FILENAME);
    }

    pub fn load_g2() -> Self {
        Self::load(PRUNE_TABLE_G2_FILENAME).unwrap()
    }
    
    pub fn gen_g3() {
        let cube = cube::Cube::new();
        let table = solver::Solver::gen_prune_table_g3(cube);
        let _ = table.save(PRUNE_TABLE_G3_FILENAME);
    }

    pub fn load_g3() -> Self {
        Self::load(PRUNE_TABLE_G3_FILENAME).unwrap()
    }

    pub fn gen_phase1() {
        let cube = cube::Cube::new();
        let table = solver::Solver::gen_prune_table_phase1(cube);
        let _ = table.save(PRUNE_TABLE_PHASE1_FILENAME);
    }

    pub fn load_phase1() -> Self {
        Self::load(PRUNE_TABLE_PHASE1_FILENAME).unwrap()
    }
}