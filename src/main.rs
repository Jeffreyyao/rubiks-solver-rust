// corners
// 0  URF    1  UFL
// 2  ULB    3  UBR
// 4  DRF    5  DFL
// 6  DLB    7  DBR

// edges
// 0  UR   1  UF   2  UL   3  UB
// 4  DR   5  DF   6  DL   7  DB
// 8  FR   9  FL  10  BL  11  BR

#[derive(Copy, Clone, Debug)]
enum CornerOrientation {
    O1, O2, O3,
}

type CornerOrientations = [CornerOrientation; 8];

type CornerPermutations = [i8; 8];

#[derive(Copy, Clone, Debug)]
enum EdgeOrientation {
    O1, O2,
}

type EdgeOrientations = [EdgeOrientation; 12];

type EdgePermutations = [i8; 12];

struct Cube {
    corner_orientations: CornerOrientations, // 3 orientations per corner
    corner_permutations: CornerPermutations, // 8 corners
    edge_orientations: EdgeOrientations, // 2 orientations per edge
    edge_permutations: EdgePermutations, // 12 edges
}

impl Cube {
    fn new() -> Self {
        Self {
            corner_orientations: [CornerOrientation::O1; 8],
            corner_permutations: [1, 2, 3, 4, 5, 6, 7, 8],
            edge_orientations: [EdgeOrientation::O1; 12],
            edge_permutations: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        }
    }

    fn u(self, clockwise: Option<bool>) -> Self {
        let clockwise = clockwise.unwrap_or(true);
        let cp = self.corner_permutations;
        let ep = self.edge_permutations;
        if clockwise {
            return Self {
                corner_orientations: self.corner_orientations,
                corner_permutations: [
                    cp[3], cp[0], cp[1], cp[2], cp[4], cp[5], cp[6], cp[7],
                ],
                edge_orientations: self.edge_orientations,
                edge_permutations: [
                    ep[3], ep[0], ep[1], ep[2], ep[4], ep[5], ep[6], ep[7], ep[8], ep[9], ep[10],
                    ep[11],
                ],
            };
        } else {
            return Self {
                corner_orientations: self.corner_orientations,
                corner_permutations: [
                    cp[1], cp[2], cp[3], cp[0], cp[5], cp[6], cp[7], cp[4],
                ],
                edge_orientations: self.edge_orientations,
                edge_permutations: [
                    ep[1], ep[2], ep[3], ep[0], ep[5], ep[6], ep[7], ep[4], ep[9], ep[10], ep[11],
                    ep[8],
                ],
            };
        }
    }

    fn dump(self) {
        println!("corner orientations: {:?}", self.corner_orientations);
        println!("corner permutations: {:?}", self.corner_permutations);
        println!("edge orientations: {:?}", self.edge_orientations);
        println!("edge permutations: {:?}", self.edge_permutations);
    }
}

fn main() {
    let cube = Cube::new();
    cube.dump();
}
