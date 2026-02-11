use std::io;

// corners
// 0  URF    1  UFL
// 2  ULB    3  UBR
// 4  DRF    5  DFL
// 6  DLB    7  DBR

// edges
// 0  UF   1  UL   2   UB   3   UR
// 4  FR   5  FL   6   BL   7   BR
// 8  DF   9  DL   10  DB   11  DR

type CornerOrientations = [i8; 8]; // orientaion of U/D faces
type CornerPermutations = [i8; 8];

type EdgeOrientations = [i8; 12]; // edge orientation flips on F/B moves
type EdgePermutations = [i8; 12];

fn _permute<const N: usize>(permutation: [i8; N], indices: &[i8; 4], clockwise: bool) -> [i8; N] {
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

fn _orient_corners(corner_orientations: [i8; 8], indices: &[i8; 4]) -> [i8; 8] {
    let mut _corner_orientations = corner_orientations.clone();
    for i in 0..4 {
        let incremented_orientation = if i % 2 == 0 { 1 } else { 2 };
        println!(
            "i: {}, cubie: {}, orientation: {}, incremented_orientation: {}",
            i, indices[i], corner_orientations[indices[i] as usize], incremented_orientation
        );
        _corner_orientations[indices[i] as usize] =
            (corner_orientations[indices[i] as usize] + incremented_orientation) % 3;
    }
    return _corner_orientations;
}

struct Cube {
    corner_orientations: CornerOrientations, // 3 orientations per corner
    corner_permutations: CornerPermutations, // 8 corners
    edge_orientations: EdgeOrientations,     // 2 orientations per edge
    edge_permutations: EdgePermutations,     // 12 edges
}

impl Cube {
    /*
       indices in clockwise order for each face;
       start index matters for corner indices because
       calculating corner orientation requires incrementing
       orientation at specific corner indices
    */
    const U_CORNER_INDICES: [i8; 4] = [0, 1, 2, 3];
    const U_EDGE_INDICES: [i8; 4] = [0, 1, 2, 3];
    const D_CORNER_INDICES: [i8; 4] = [7, 6, 5, 4];
    const D_EDGE_INDICES: [i8; 4] = [11, 10, 9, 8];
    const L_CORNER_INDICES: [i8; 4] = [2, 1, 5, 6];
    const L_EDGE_INDICES: [i8; 4] = [1, 5, 9, 6];
    const R_CORNER_INDICES: [i8; 4] = [0, 3, 7, 4];
    const R_EDGE_INDICES: [i8; 4] = [3, 7, 11, 4];
    const F_CORNER_INDICES: [i8; 4] = [1, 0, 4, 5];
    const F_EDGE_INDICES: [i8; 4] = [0, 4, 8, 5];
    const B_CORNER_INDICES: [i8; 4] = [3, 2, 6, 7];
    const B_EDGE_INDICES: [i8; 4] = [2, 6, 10, 7];

    fn new() -> Self {
        Self {
            corner_orientations: [0; 8], // all corners are oriented 0
            corner_permutations: (0..8).collect::<Vec<i8>>().try_into().unwrap(), // 0 to 7
            edge_orientations: [0; 12],  // all edges are oriented 0
            edge_permutations: (0..12).collect::<Vec<i8>>().try_into().unwrap(), // 0 to 11
        }
    }

    fn u(self, clockwise: bool) -> Self {
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
            edge_orientations: self.edge_orientations,
            edge_permutations: _permute(self.edge_permutations, &Self::U_EDGE_INDICES, clockwise),
        };
    }

    fn d(self, clockwise: bool) -> Self {
        return Self {
            corner_orientations: self.corner_orientations,
            corner_permutations: _permute(
                self.corner_permutations,
                &Self::D_CORNER_INDICES,
                clockwise,
            ),
            edge_orientations: self.edge_orientations,
            edge_permutations: _permute(self.edge_permutations, &Self::D_EDGE_INDICES, clockwise),
        };
    }

    fn l(self, clockwise: bool) -> Self {
        return Self {
            corner_orientations: _permute(
                _orient_corners(self.corner_orientations, &Self::L_CORNER_INDICES),
                &Self::L_CORNER_INDICES,
                clockwise,
            ),
            corner_permutations: _permute(
                self.corner_permutations,
                &Self::L_CORNER_INDICES,
                clockwise,
            ),
            edge_orientations: self.edge_orientations,
            edge_permutations: _permute(self.edge_permutations, &Self::L_EDGE_INDICES, clockwise),
        };
    }

    fn r(self, clockwise: bool) -> Self {
        return Self {
            corner_orientations: _permute(
                _orient_corners(self.corner_orientations, &Self::R_CORNER_INDICES),
                &Self::R_CORNER_INDICES,
                clockwise,
            ),
            corner_permutations: _permute(
                self.corner_permutations,
                &Self::R_CORNER_INDICES,
                clockwise,
            ),
            edge_orientations: self.edge_orientations,
            edge_permutations: _permute(self.edge_permutations, &Self::R_EDGE_INDICES, clockwise),
        };
    }

    fn f(self, clockwise: bool) -> Self {
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
            edge_orientations: self.edge_orientations,
            edge_permutations: _permute(self.edge_permutations, &Self::F_EDGE_INDICES, clockwise),
        };
    }

    fn b(self, clockwise: bool) -> Self {
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
            edge_orientations: self.edge_orientations,
            edge_permutations: _permute(self.edge_permutations, &Self::B_EDGE_INDICES, clockwise),
        };
    }

    fn dump(&self) {
        println!("CO: {:?}", self.corner_orientations);
        println!("CP: {:?}", self.corner_permutations);
        println!("EO: {:?}", self.edge_orientations);
        println!("EP: {:?}", self.edge_permutations);
    }

    fn apply_move(self, face: char, prime: bool, half_turn: bool) -> Self {
        let clockwise = !prime;
        let mut c = if face == 'u' || face == 'U' {
            self.u(clockwise)
        } else if face == 'd' || face == 'D' {
            self.d(clockwise)
        } else if face == 'l' || face == 'L' {
            self.l(clockwise)
        } else if face == 'r' || face == 'R' {
            self.r(clockwise)
        } else if face == 'f' || face == 'F' {
            self.f(clockwise)
        } else if face == 'b' || face == 'B' {
            self.b(clockwise)
        } else {
            return self;
        };
        if half_turn {
            c = c.apply_move(face, prime, false);
        }
        c
    }

    fn apply_sequence(self, s: &str) -> Self {
        let mut cube = self;
        let s = s.replace(' ', "");
        let mut chars = s.chars().peekable();
        while let Some(face) = chars.next() {
            if !matches!(face, 'u'|'U'|'d'|'D'|'l'|'L'|'r'|'R'|'f'|'F'|'b'|'B') {
                continue;
            }
            let prime = chars.peek().copied() == Some('\'');
            if prime {
                chars.next();
            }
            let half_turn = chars.peek().copied() == Some('2');
            if half_turn {
                chars.next();
            }
            cube = cube.apply_move(face, prime, half_turn);
        }
        cube
    }
}

fn main() {
    let mut cube = Cube::new();
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
}
