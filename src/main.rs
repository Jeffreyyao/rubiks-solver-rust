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

#[derive(Copy, Clone, Debug)]
enum CornerOrientation {
    CO1,
    CO2,
    CO3, // O1: original rotation, O2: 90 degrees clockwise, O3: 90 degrees counterclockwise
}
type CornerOrientations = [CornerOrientation; 8];
type CornerPermutations = [i8; 8];

#[derive(Copy, Clone, Debug)]
enum EdgeOrientation {
    EO1,
    EO2, // O1: original rotation, O2: 180 degrees
}
type EdgeOrientations = [EdgeOrientation; 12]; // edge orientation flips on F/B moves
type EdgePermutations = [i8; 12];

fn _permute<const N: usize>(permutation: [i8; N], indices: &[i8; 4], clockwise: bool) -> [i8; N] {
    let mut _perm = permutation.clone();
    for i in 0..4 {
        if clockwise {
            let p_index = if i == 0 { 3 } else { i - 1 };
            _perm[indices[i] as usize] = permutation[indices[p_index] as usize];
        } else {
            let p_index = if i == 3 { 0 } else { i + 1 };
            _perm[indices[i] as usize] = permutation[indices[p_index] as usize];
        }
    }
    return _perm;
}

struct Cube {
    corner_orientations: CornerOrientations, // 3 orientations per corner
    corner_permutations: CornerPermutations, // 8 corners
    edge_orientations: EdgeOrientations,     // 2 orientations per edge
    edge_permutations: EdgePermutations,     // 12 edges
}

impl Cube {
    fn new() -> Self {
        Self {
            corner_orientations: [CornerOrientation::CO1; 8],
            corner_permutations: (0..8).collect::<Vec<i8>>().try_into().unwrap(), // 0 to 7
            edge_orientations: [EdgeOrientation::EO1; 12],
            edge_permutations: (0..12).collect::<Vec<i8>>().try_into().unwrap(), // 0 to 11
        }
    }

    fn u(self, clockwise: bool) -> Self {
        return Self {
            corner_orientations: self.corner_orientations,
            corner_permutations: _permute(self.corner_permutations, &[0, 1, 2, 3], clockwise),
            edge_orientations: self.edge_orientations,
            edge_permutations: _permute(self.edge_permutations, &[0, 1, 2, 3], clockwise),
        };
    }

    fn d(self, clockwise: bool) -> Self {
        return Self {
            corner_orientations: self.corner_orientations,
            corner_permutations: _permute(self.corner_permutations, &[7, 6, 5, 4], clockwise),
            edge_orientations: self.edge_orientations,
            edge_permutations: _permute(self.edge_permutations, &[11, 10, 9, 8], clockwise),
        };
    }

    fn l(self, clockwise: bool) -> Self {
        return Self {
            corner_orientations: self.corner_orientations,
            corner_permutations: _permute(self.corner_permutations, &[1, 5, 6, 2], clockwise),
            edge_orientations: self.edge_orientations,
            edge_permutations: _permute(self.edge_permutations, &[1, 5, 6, 9], clockwise),
        };
    }
    fn r(self, clockwise: bool) -> Self {
        return Self {
            corner_orientations: self.corner_orientations,
            corner_permutations: _permute(self.corner_permutations, &[0, 3, 4, 7], clockwise),
            edge_orientations: self.edge_orientations,
            edge_permutations: _permute(self.edge_permutations, &[3, 7, 11, 4], clockwise),
        };
    }
    fn f(self, clockwise: bool) -> Self {
        return Self {
            corner_orientations: self.corner_orientations,
            corner_permutations: _permute(self.corner_permutations, &[0, 4, 5, 1], clockwise),
            edge_orientations: self.edge_orientations,
            edge_permutations: _permute(self.edge_permutations, &[0, 4, 8, 5], clockwise),
        };
    }

    fn b(self, clockwise: bool) -> Self {
        return Self {
            corner_orientations: self.corner_orientations,
            corner_permutations: _permute(self.corner_permutations, &[3, 2, 6, 7], clockwise),
            edge_orientations: self.edge_orientations,
            edge_permutations: _permute(self.edge_permutations, &[2, 6, 10, 7], clockwise),
        };
    }

    fn dump(&self) {
        println!("corner orientations: {:?}", self.corner_orientations);
        println!("corner permutations: {:?}", self.corner_permutations);
        println!("edge orientations: {:?}", self.edge_orientations);
        println!("edge permutations: {:?}", self.edge_permutations);
    }
}

fn main() {
    let mut cube = Cube::new();
    cube.dump();
    let mut buffer = String::new();
    while let Ok(_) = io::stdin().read_line(&mut buffer) {
        println!("input: {}", buffer);
        let line = buffer.trim();
        let face = line.chars().next().unwrap();
        let clockwise = !line.ends_with("'");
        if face == 'u' {
            cube = cube.u(clockwise);
        } else if face == 'd' {
            cube = cube.d(clockwise);
        } else if face == 'l' {
            cube = cube.l(clockwise);
        } else if face == 'r' {
            cube = cube.r(clockwise);
        } else if line == "f'" {
            cube = cube.f(clockwise);
        } else if face == 'b' {
            cube = cube.b(clockwise);
        } else {
            println!("invalid move");
            continue;
        }
        cube.dump();
        buffer.clear();
    }
}
