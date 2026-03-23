use rand::{RngExt};
use std::fmt;

// corners
// 0  URF    1  UFL
// 2  ULB    3  UBR
// 4  DRF    5  DFL
// 6  DLB    7  DBR

// edges
// 0  UF   1  UL   2   UB   3   UR
// 4  FR   5  FL   6   BL   7   BR
// 8  DF   9  DL   10  DB   11  DR

pub type CornerOrientations = [u8; 8]; // orientaion of L/R faces
pub type CornerPermutations = [u8; 8];

pub type EdgeOrientations = [u8; 12]; // edge orientation flips on U/D moves
pub type EdgePermutations = [u8; 12];

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Face {
    U, D, L, R, F, B
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Dir {
    CW, CCW, HT
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Mov {
    face: Face,
    dir: Dir,
}

fn _permute<const N: usize>(permutation: [u8; N], indices: &[u8; 4], clockwise: bool) -> [u8; N] {
    let mut _perm = permutation.clone();
    for i in 0..4 {
        let p_index;
        if clockwise {
            p_index = if i == 0 { 3 } else { i - 1 };
        } else {
            p_index = if i == 3 { 0 } else { i + 1 };
        }
        _perm[indices[i] as usize] = permutation[indices[p_index] as usize];
    }
    return _perm;
}

fn _orient_edges(edge_orientations: [u8; 12], indices: &[u8; 4]) -> [u8; 12] {
    let mut _edge_orientations = edge_orientations.clone();
    for i in 0..4 {
        _edge_orientations[indices[i] as usize] = (edge_orientations[indices[i] as usize] + 1) % 2;
    }
    return _edge_orientations;
}

fn _orient_corners(corner_orientations: [u8; 8], indices: &[u8; 4]) -> [u8; 8] {
    let mut _corner_orientations = corner_orientations.clone();
    for i in 0..4 {
        _corner_orientations[indices[i] as usize] =
            (corner_orientations[indices[i] as usize] + ((i as u8 % 2) + 1)) % 3;
    }
    return _corner_orientations;
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Cube {
    pub corner_orientations: CornerOrientations, // 3 orientations per corner
    pub corner_permutations: CornerPermutations, // 8 corners
    pub edge_orientations: EdgeOrientations,     // 2 orientations per edge
    pub edge_permutations: EdgePermutations,     // 12 edges
}

impl Cube {
    /*
       indices in clockwise order for each face;
       first index matters for corner indices because
       calculating corner orientation requires incrementing
       orientation at specific corner indices
    */
    const U_CORNER_INDICES: [u8; 4] = [0, 1, 2, 3];
    const U_EDGE_INDICES: [u8; 4] = [0, 1, 2, 3];
    const D_CORNER_INDICES: [u8; 4] = [7, 6, 5, 4];
    const D_EDGE_INDICES: [u8; 4] = [11, 10, 9, 8];
    const L_CORNER_INDICES: [u8; 4] = [2, 1, 5, 6];
    const L_EDGE_INDICES: [u8; 4] = [1, 5, 9, 6];
    const R_CORNER_INDICES: [u8; 4] = [0, 3, 7, 4];
    const R_EDGE_INDICES: [u8; 4] = [3, 7, 11, 4];
    const F_CORNER_INDICES: [u8; 4] = [1, 0, 4, 5];
    const F_EDGE_INDICES: [u8; 4] = [0, 4, 8, 5];
    const B_CORNER_INDICES: [u8; 4] = [3, 2, 6, 7];
    const B_EDGE_INDICES: [u8; 4] = [2, 6, 10, 7];

    pub const G2_SLICE_EDGES: [u8; 4] = [0, 2, 8, 10];
    pub const G3_TETRAD_CORNERS: [u8; 4] = [0, 2, 5, 7];
    pub const G3_SLICE_EDGES: [u8; 4] = [4, 5, 6, 7];

    pub fn new() -> Self {
        Self {
            corner_orientations: [0; 8], // all corners are oriented 0
            corner_permutations: (0..8).collect::<Vec<u8>>().try_into().unwrap(), // 0 to 7
            edge_orientations: [0; 12],  // all edges are oriented 0
            edge_permutations: (0..12).collect::<Vec<u8>>().try_into().unwrap(), // 0 to 11
        }
    }

    pub fn new_from_indices(corner_orientations: CornerOrientations, corner_permutations: CornerPermutations, edge_orientations: EdgeOrientations, edge_permutations: EdgePermutations) -> Self {
        Self {
            corner_orientations,
            corner_permutations,
            edge_orientations,
            edge_permutations,
        }
    }

    pub fn u(self, clockwise: bool) -> Self {
        return Self {
            corner_orientations: _permute(
                _orient_corners(self.corner_orientations, &Self::U_CORNER_INDICES),
                &Self::U_CORNER_INDICES,
                clockwise,
            ),
            corner_permutations: _permute(
                self.corner_permutations,
                &Self::U_CORNER_INDICES,
                clockwise,
            ),
            edge_orientations: _permute(
                _orient_edges(self.edge_orientations, &Self::U_EDGE_INDICES),
                &Self::U_EDGE_INDICES,
                clockwise,
            ),
            edge_permutations: _permute(self.edge_permutations, &Self::U_EDGE_INDICES, clockwise),
        };
    }

    pub fn d(self, clockwise: bool) -> Self {
        return Self {
            corner_orientations: _permute(
                _orient_corners(self.corner_orientations, &Self::D_CORNER_INDICES),
                &Self::D_CORNER_INDICES,
                clockwise,
            ),
            corner_permutations: _permute(
                self.corner_permutations,
                &Self::D_CORNER_INDICES,
                clockwise,
            ),
            edge_orientations: _permute(
                _orient_edges(self.edge_orientations, &Self::D_EDGE_INDICES),
                &Self::D_EDGE_INDICES,
                clockwise,
            ),
            edge_permutations: _permute(self.edge_permutations, &Self::D_EDGE_INDICES, clockwise),
        };
    }

    pub fn l(self, clockwise: bool) -> Self {
        return Self {
            corner_orientations: _permute(
                self.corner_orientations,
                &Self::L_CORNER_INDICES,
                clockwise,
            ),
            corner_permutations: _permute(
                self.corner_permutations,
                &Self::L_CORNER_INDICES,
                clockwise,
            ),
            edge_orientations: _permute(self.edge_orientations, &Self::L_EDGE_INDICES, clockwise),
            edge_permutations: _permute(self.edge_permutations, &Self::L_EDGE_INDICES, clockwise),
        };
    }

    pub fn r(self, clockwise: bool) -> Self {
        return Self {
            corner_orientations: _permute(
                self.corner_orientations,
                &Self::R_CORNER_INDICES,
                clockwise,
            ),
            corner_permutations: _permute(
                self.corner_permutations,
                &Self::R_CORNER_INDICES,
                clockwise,
            ),
            edge_orientations: _permute(self.edge_orientations, &Self::R_EDGE_INDICES, clockwise),
            edge_permutations: _permute(self.edge_permutations, &Self::R_EDGE_INDICES, clockwise),
        };
    }

    pub fn f(self, clockwise: bool) -> Self {
        return Self {
            corner_orientations: _permute(
                _orient_corners(self.corner_orientations, &Self::F_CORNER_INDICES),
                &Self::F_CORNER_INDICES,
                clockwise,
            ),
            corner_permutations: _permute(
                self.corner_permutations,
                &Self::F_CORNER_INDICES,
                clockwise,
            ),
            edge_orientations: _permute(self.edge_orientations, &Self::F_EDGE_INDICES, clockwise),
            edge_permutations: _permute(self.edge_permutations, &Self::F_EDGE_INDICES, clockwise),
        };
    }

    pub fn b(self, clockwise: bool) -> Self {
        return Self {
            corner_orientations: _permute(
                _orient_corners(self.corner_orientations, &Self::B_CORNER_INDICES),
                &Self::B_CORNER_INDICES,
                clockwise,
            ),
            corner_permutations: _permute(
                self.corner_permutations,
                &Self::B_CORNER_INDICES,
                clockwise,
            ),
            edge_orientations: _permute(self.edge_orientations, &Self::B_EDGE_INDICES, clockwise),
            edge_permutations: _permute(self.edge_permutations, &Self::B_EDGE_INDICES, clockwise),
        };
    }

    fn move_to_char(mov: Mov) -> String {
        let mut s = String::new();
        match mov.face {
            Face::U => s.push('u'), Face::D => s.push('d'),
            Face::L => s.push('l'), Face::R => s.push('r'),
            Face::F => s.push('f'), Face::B => s.push('b'),
        }
        if mov.dir == Dir::CCW {
            s.push('\'');
        } else if mov.dir == Dir::HT {
            s.push('2');
        }
        s
    }

    fn char_to_face(c: char) -> Option<Face> {
        match c {
            'u' | 'U' => Some(Face::U), 'd' | 'D' => Some(Face::D),
            'l' | 'L' => Some(Face::L), 'r' | 'R' => Some(Face::R),
            'f' | 'F' => Some(Face::F), 'b' | 'B' => Some(Face::B),
            _ => None,
        }
    }

    fn char_to_dir(c: char) -> Option<Dir> {
        match c {
            '\'' => Some(Dir::CCW),
            '2' => Some(Dir::HT),
            _ => None,
        }
    }

    pub fn apply_move(self, mov: Mov) -> Self {
        let mut c = match mov.face {
            Face::U => self.u(mov.dir == Dir::CW), Face::D => self.d(mov.dir == Dir::CW),
            Face::L => self.l(mov.dir == Dir::CW), Face::R => self.r(mov.dir == Dir::CW),
            Face::F => self.f(mov.dir == Dir::CW), Face::B => self.b(mov.dir == Dir::CW),
        };
        if mov.dir == Dir::HT {
            c = c.apply_move(Mov { face: mov.face, dir: Dir::CCW });
        }
        c
    }

    pub fn apply_sequence(self, s: &str) -> Self {
        let mut cube = self;
        let mut chars = s.chars().peekable();
        while let Some(char) = chars.next() {
            if let Some(face) = Self::char_to_face(char) {
                let mut dir = Dir::CW;
                if let Some(&next_char) = chars.peek() {
                    if let Some(d) = Self::char_to_dir(next_char) {
                        dir = d;
                    }
                }
                cube = cube.apply_move(Mov { face, dir });
            }
        }
        cube
    }

    pub fn scramble(self, n: u32) -> (Self, String) {
        fn index_to_face(index: u8) -> Face {
            match index {
                0 => Face::U,
                1 => Face::D,
                2 => Face::L,
                3 => Face::R,
                4 => Face::F,
                5 => Face::B,
                _ => Face::U,
            }
        }
        fn index_to_dir(index: u8) -> Dir {
            match index {
                0 => Dir::CW,
                1 => Dir::CCW,
                2 => Dir::HT,
                _ => Dir::CW,
            }
        }
        let mut cube = self;
        let mut rng = rand::rng();
        let mut scrambled_moves = String::new();
        for _ in 0..n {
            let face = index_to_face(rng.random_range(0..6));
            let dir = index_to_dir(rng.random_range(0..2));
            cube = cube.apply_move(Mov { face, dir });
            scrambled_moves.push_str(&Self::move_to_char(Mov { face, dir }));
        }
        (cube, scrambled_moves)
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "CO: {:?}", self.corner_orientations)?;
        writeln!(f, "CP: {:?}", self.corner_permutations)?;
        writeln!(f, "EO: {:?}", self.edge_orientations)?;
        writeln!(f, "EP: {:?}", self.edge_permutations)
    }
}
