use rand::{RngExt};
use std::fmt;
use std::vec::Vec;

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
    pub face: Face,
    pub dir: Dir,
}

impl Mov {
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("{:?}", self.face));
        match self.dir {
            Dir::CW => s.push_str(""),
            Dir::CCW => s.push_str("'"),
            Dir::HT => s.push_str("2"),
        }
        s
    }
}

pub const U: Mov = Mov { face: Face::U, dir: Dir::CW };
pub const UP: Mov = Mov { face: Face::U, dir: Dir::CCW };
pub const U2: Mov = Mov { face: Face::U, dir: Dir::HT };
pub const D: Mov = Mov { face: Face::D, dir: Dir::CW };
pub const DP: Mov = Mov { face: Face::D, dir: Dir::CCW };
pub const D2: Mov = Mov { face: Face::D, dir: Dir::HT };
pub const L: Mov = Mov { face: Face::L, dir: Dir::CW };
pub const LP: Mov = Mov { face: Face::L, dir: Dir::CCW };
pub const L2: Mov = Mov { face: Face::L, dir: Dir::HT };
pub const R: Mov = Mov { face: Face::R, dir: Dir::CW };
pub const RP: Mov = Mov { face: Face::R, dir: Dir::CCW };
pub const R2: Mov = Mov { face: Face::R, dir: Dir::HT };
pub const F: Mov = Mov { face: Face::F, dir: Dir::CW };
pub const FP: Mov = Mov { face: Face::F, dir: Dir::CCW };
pub const F2: Mov = Mov { face: Face::F, dir: Dir::HT };
pub const B: Mov = Mov { face: Face::B, dir: Dir::CW };
pub const BP: Mov = Mov { face: Face::B, dir: Dir::CCW };
pub const B2: Mov = Mov { face: Face::B, dir: Dir::HT };

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Moves(pub Vec<Mov>);

impl Moves {
    pub fn to_string(&self) -> String {
        self.0.iter().map(|m| m.to_string()).collect::<Vec<String>>().join(" ")
    }

    pub fn push(&mut self, mov: Mov) {
        self.0.push(mov);
    }

    pub fn extend(&mut self, moves: Moves) {
        self.0.extend(moves.0);
    }
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
    pub prev_move: Option<Mov>,                  // used to avoid repeating the same face move in bfs
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

    pub const LR_MID_SLICE_EDGES: [u8; 4] = [0, 2, 8, 10];
    pub const UD_MID_SLICE_EDGES: [u8; 4] = [4, 5, 6, 7];
    pub const FB_MID_SLICE_EDGES: [u8; 4] = [1, 3, 9, 11];
    pub const HALF_TETRAD_1_CORNERS: [u8; 2] = [0, 2];
    pub const HALF_TETRAD_2_CORNERS: [u8; 2] = [5, 7];
    pub const HALF_TETRAD_3_CORNERS: [u8; 2] = [1, 3];
    pub const TETRAD_1_CORNERS: [u8; 4] = [0, 2, 5, 7];
    pub const TETRAD_2_CORNERS: [u8; 4] = [1, 3, 4, 6];

    pub fn new() -> Self {
        Self {
            corner_orientations: [0; 8], // all corners are oriented 0
            corner_permutations: (0..8).collect::<Vec<u8>>().try_into().unwrap(), // 0 to 7
            edge_orientations: [0; 12],  // all edges are oriented 0
            edge_permutations: (0..12).collect::<Vec<u8>>().try_into().unwrap(),  // 0 to 11
            prev_move: None,
        }
    }

    pub fn u(self, clockwise: bool, mov: Option<Mov>) -> Self {
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
            prev_move: mov,
        };
    }

    pub fn d(self, clockwise: bool, mov: Option<Mov>) -> Self {
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
            prev_move: mov,
        };
    }

    pub fn l(self, clockwise: bool, mov: Option<Mov>) -> Self {
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
            prev_move: mov,
        };
    }

    pub fn r(self, clockwise: bool, mov: Option<Mov>) -> Self {
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
            prev_move: mov,
        };
    }

    pub fn f(self, clockwise: bool, mov: Option<Mov>) -> Self {
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
            prev_move: mov,
        };
    }

    pub fn b(self, clockwise: bool, mov: Option<Mov>) -> Self {
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
            prev_move: mov,
        };
    }

    pub fn char_to_face(c: char) -> Option<Face> {
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
            Face::U => self.u(mov.dir == Dir::CW, Some(mov)), Face::D => self.d(mov.dir == Dir::CW, Some(mov)),
            Face::L => self.l(mov.dir == Dir::CW, Some(mov)), Face::R => self.r(mov.dir == Dir::CW, Some(mov)),
            Face::F => self.f(mov.dir == Dir::CW, Some(mov)), Face::B => self.b(mov.dir == Dir::CW, Some(mov)),
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
                cube = cube.apply_move(Mov{face, dir});
            }
        }
        cube
    }

    pub fn apply_moves(self, moves: Moves) -> Self {
        let mut cube = self;
        for mov in moves.0 {
            cube = cube.apply_move(mov);
        }
        cube
    }

    pub fn scramble(self, n: u32) -> (Self, Moves) {
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
        let mut scrambled_moves = Moves(vec![]);
        let mut moves_applied_count = 0;
        while moves_applied_count < n {
            let face = index_to_face(rng.random_range(0..6));
            let dir = index_to_dir(rng.random_range(0..3));
            let mov = Mov { face, dir };
            if cube.prev_move.is_some() && cube.prev_move.unwrap().face == mov.face {
                continue;
            }
            cube = cube.apply_move(mov);
            scrambled_moves.0.push(mov);
            moves_applied_count += 1;
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
