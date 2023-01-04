use std::collections::{HashMap, HashSet};

use itertools::Itertools;
/**
Find symmetry equivalent k-points to generate irreducible k-points.
# Step:
1. Sample the reciprocal lattice space with specified spacing distance,
which results in a [P, Q, R] grid.
2. Generate permutations of k-points from the grid. The fractional coordinates are
generated following the formula: f_i = (2 * r - q - 1) / (2 * q), where r is (1..= q)
3. If we take the numerator part of the f_i: (2 * r - q -1), it would generate a series of
`i32` which are symmetric around the zero. The k-point coordinate can be expressed as [n_x, n_y, n_z]
4. "Hash" the coordinate by the formula: H = n_x * Q * R + n_y * R + n_z. If |H_1| == |H_2|, they are
the most basic symmetry equivalent points: the operation is identical with diagonal to be all -1.
5. Apply other symmetry operations to all points. Calculate the new hash key for the transformed point,
lookup the key, if found, merge them in a vector, indicating they are symmetry equivalent.
6. The final table will naturally indicates the degeneracies of the irreducible k-points, which leads to
the weight determination of these irreducible k-points. From all the possible k-points, select the one with least
negative numbers as the "representative", which is just a convention.
*/
use nalgebra::{Matrix3, Point3};
use num::integer::lcm;

use crate::data::{format::DataFormat, param::KPoint};

pub struct SymSpace {
    grid: [u32; 3],
    symmetry_operations: Vec<Matrix3<f64>>,
    is_hexagonal: bool,
}

#[derive(Debug)]
struct SymKPoint {
    coord: Point3<f64>,
    multiplicity: u32,
}

impl SymKPoint {
    fn new(coord: Point3<f64>, multiplicity: u32) -> Self {
        Self {
            coord,
            multiplicity,
        }
    }
}

trait MPSpace {
    fn mesh(num_points: u32) -> Vec<f64> {
        let q = num_points as i32;
        (1..=q)
            .into_iter()
            .rev()
            .map(|r| (2_f64 * r as f64 - q as f64 - 1_f64) / (2_f64 * q as f64))
            .collect()
    }
    fn hexagonal_mesh(num_points: u32) -> Vec<f64> {
        let q = num_points as i32;
        (1..=q)
            .into_iter()
            .rev()
            .map(|r| (r - 1) as f64 / q as f64)
            .collect()
    }
}

impl MPSpace for SymSpace {}

impl SymSpace {
    pub fn new(grid: [u32; 3], symmetry_operations: &[Matrix3<f64>], is_hexagonal: bool) -> Self {
        // let group = symmetry_operations
        //     .iter()
        //     .powerset()
        //     .filter(|c| !c.is_empty())
        //     .map(|c| c.into_iter().product::<Matrix3<f64>>())
        //     .collect();
        Self {
            grid,
            symmetry_operations: symmetry_operations.to_vec(),
            is_hexagonal,
        }
    }
    fn recover_index(&self, address: i32) -> [i32; 3] {
        let [a, b, c] = self.grid;
        [address % a as i32, address % b as i32, address % c as i32]
    }
    fn inspect_symmetry(
        &self,
        point: &Point3<f64>,
        sym_num_hash_set: &mut HashSet<i32>,
        irreducible_points: &mut Vec<SymKPoint>,
    ) {
        let p_hash_key = point.sym_num(&self.grid);
        // Look up at the table
        match sym_num_hash_set.get(&p_hash_key) {
            // Already added by other points' symmetry operations
            Some(_) => {}
            // New equivalent point found
            None => {
                // Add to hash table
                sym_num_hash_set.insert(p_hash_key);
                println!("Added: {}, {}", point, p_hash_key);
                println!("Equivalent:");
                let mut local_sym_set: HashMap<i32, usize> = HashMap::new();
                // Apply all symmetry operations in the system
                self.symmetry_operations
                    .iter()
                    .enumerate()
                    .map(|(i, ops)| {
                        let sym_point = ops * point;
                        (sym_point.sym_num(&self.grid), i)
                    })
                    // Exclude situation when the symmetry operation repeats itself
                    // E.g., the origin (0.0, 0.0, 0.0)
                    .filter(|&n| n.0 != p_hash_key)
                    .for_each(|(key, p)| {
                        local_sym_set.insert(key, p);
                    });
                local_sym_set.iter().for_each(|(key, p)| {
                    sym_num_hash_set.insert(*key);
                    println!("{}", p);
                });
                println!("Equivalent check ends");
                let irreducible_point = SymKPoint::new(*point, 1 + local_sym_set.len() as u32);
                irreducible_points.push(irreducible_point);
            }
        }
    }
    fn reduce_with_symmetry(&self) -> Vec<SymKPoint> {
        let [a, b, c] = self.grid;
        let a_mesh = if self.is_hexagonal {
            Self::hexagonal_mesh(a)
        } else {
            Self::mesh(a)
        };
        let b_mesh = if self.is_hexagonal {
            Self::hexagonal_mesh(b)
        } else {
            Self::mesh(b)
        };
        let c_mesh = Self::mesh(c);
        let mut hash_set: HashSet<i32> = HashSet::new();
        let mut irreducible_kpoints: Vec<SymKPoint> = Vec::new();
        // Iterate over the permutations of the possible k-point coord values
        // along the three directions.
        a_mesh.iter().for_each(|a| {
            b_mesh.iter().for_each(|b| {
                c_mesh.iter().for_each(|c| {
                    // Initialize a point to calculate its hash key.
                    let p: Point3<f64> = Point3::new(*a, *b, *c);
                    self.inspect_symmetry(&p, &mut hash_set, &mut irreducible_kpoints);
                })
            })
        });
        irreducible_kpoints
    }
    pub fn weighted_kpoints<T: DataFormat>(&self) -> Vec<KPoint<T>> {
        let irreducible_points = self.reduce_with_symmetry();
        let m_sum = irreducible_points
            .iter()
            .map(|p| p.multiplicity)
            .reduce(|acc, x| acc + x)
            .unwrap();
        irreducible_points
            .iter()
            .map(|p| {
                let point = p.coord;
                let weight = p.multiplicity as f64 / m_sum as f64;
                KPoint::new([point.x, point.y, point.z, weight])
            })
            .collect()
    }
    fn display_kpoints<T: DataFormat>(&self) {
        let kpts = self.weighted_kpoints::<T>();
        println!("%BLOCK KPOINTS_LIST");
        for kpt in kpts.iter() {
            let [x, y, z, w] = kpt.content();
            let w = (w * 1000_f64).floor() / 1000_f64;
            println!("{:20.16}{:20.16}{:20.16}{:20.16}", x, y, z, w);
        }
        println!("%ENDBLOCK KPOINTS_LIST");
    }
}

trait SymHash {
    fn sym_num(&self, grid: &[u32; 3]) -> i32 {
        self.rank(grid)
    }
    fn rank(&self, grid: &[u32; 3]) -> i32;
}

impl SymHash for Point3<f64> {
    fn rank(&self, grid: &[u32; 3]) -> i32 {
        let d2d1 = grid[1] * grid[2];
        let d1 = grid[2];
        let lcm = lcm(grid[0], lcm(grid[1], grid[2]));
        let [p, q, r] = [self.x, self.y, self.z];
        let hashed = (p * d2d1 as f64 + q * d1 as f64 + r) * lcm as f64;
        hashed.round() as i32
    }
}

#[cfg(test)]
mod test {

    use itertools::Itertools;
    use nalgebra::{Matrix3, Point3, Rotation3, Vector3};

    use crate::data::{format::cell::Cell, param::KPoint};

    use super::SymSpace;
    fn d2_space() {
        let ops_1 = Matrix3::from_diagonal(&Vector3::new(1.0, 1.0, 1.0));
        let ops_2 = Matrix3::from_diagonal(&Vector3::new(-1.0, -1.0, 1.0));
        let ops_3 = Matrix3::from_diagonal(&Vector3::new(-1.0, 1.0, -1.0));
        let ops_4 = Matrix3::from_diagonal(&Vector3::new(1.0, -1.0, -1.0));
        let ops_5 = Matrix3::from_diagonal(&Vector3::new(-1.0, -1.0, -1.0));
        let ops_6 = Matrix3::from_diagonal(&Vector3::new(1.0, 1.0, -1.0));
        let ops_7 = Matrix3::from_diagonal(&Vector3::new(1.0, -1.0, 1.0));
        let ops_8 = Matrix3::from_diagonal(&Vector3::new(-1.0, 1.0, 1.0));
        let sym_ops = vec![ops_1, ops_2, ops_3, ops_4, ops_5, ops_6, ops_7, ops_8];
        let d2_grid: [u32; 3] = [3, 4, 5];
        let d2_space = SymSpace::new(d2_grid, &sym_ops, false);
        println!("d2:");
        d2_space.display_kpoints::<Cell>();
    }
    fn c2_space() {
        let ops_1 = Matrix3::from_diagonal(&Vector3::new(-1.0, -1.0, -1.0));
        let ops_2 = Matrix3::from_diagonal(&Vector3::new(-1.0, 1.0, -1.0));
        let c2_grid: [u32; 3] = [5, 5, 5];
        let c2_space = SymSpace::new(c2_grid, &vec![ops_1, ops_2], false);
        println!("c2:");
        c2_space.display_kpoints::<Cell>();
    }
    fn c2v_space() {
        let ops_1 = Matrix3::from_diagonal(&Vector3::new(-1.0, -1.0, -1.0));
        let ops_4 = Matrix3::from_diagonal(&Vector3::new(-1.0, -1.0, 1.0));
        let ops_5 = Matrix3::from_diagonal(&Vector3::new(-1.0, 1.0, 1.0));
        let ops_6 = Matrix3::from_diagonal(&Vector3::new(1.0, -1.0, 1.0));
        let c2v_ops = vec![ops_1, ops_4, ops_5, ops_6];
        let c2v_space = SymSpace::new([6, 6, 6], &c2v_ops, false);
        println!("c2v:");
        c2v_space.display_kpoints::<Cell>();
    }
    fn c2h_space() {
        let ops_1 = Matrix3::from_diagonal(&Vector3::new(-1.0, -1.0, -1.0));
        let ops_6 = Matrix3::from_diagonal(&Vector3::new(1.0, -1.0, 1.0));
        let ops_3 = Matrix3::from_diagonal(&Vector3::new(-1.0, 1.0, -1.0));
        let c2h_ops = vec![ops_1, ops_3, ops_6];
        let c2h_space = SymSpace::new([2, 2, 2], &c2h_ops, false);
        println!("c2h: ");
        c2h_space.display_kpoints::<Cell>();
    }
    fn d6h_space() {
        let d6h_ops = vec![
            Matrix3::from_diagonal_element(1.0),
            Matrix3::new(0.0, -1.0, 0.0, 1.0, -1.0, 0.0, 0.0, 0.0, 1.0),
            Matrix3::new(-1.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0),
            Matrix3::new(-1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0),
            Matrix3::new(0.0, 1.0, 0.0, -1.0, 1.0, 0.0, 0.0, 0.0, 1.0),
            Matrix3::new(1.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0),
            Matrix3::new(0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, -1.0),
            Matrix3::new(1.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0),
            Matrix3::new(-1.0, 0.0, 0.0, -1.0, 1.0, 0.0, 0.0, 0.0, -1.0),
            Matrix3::new(0.0, -1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, -1.0),
            Matrix3::new(-1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, -1.0),
            Matrix3::new(1.0, 0.0, 0.0, 1.0, -1.0, 0.0, 0.0, 0.0, -1.0),
            Matrix3::new(-1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0),
            Matrix3::new(0.0, 1.0, 0.0, -1.0, 1.0, 0.0, 0.0, 0.0, -1.0),
            Matrix3::new(0.0, 1.0, 0.0, -1.0, 1.0, 0.0, 0.0, 0.0, -1.0),
            Matrix3::new(1.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, -1.0),
            Matrix3::new(
                1_f64, 0_f64, 0_f64, 0_f64, 1_f64, 0_f64, 0_f64, 0_f64, -1_f64,
            ),
            Matrix3::new(
                0_f64, -1_f64, 0_f64, 1_f64, -1_f64, 0_f64, 0_f64, 0_f64, -1_f64,
            ),
            Matrix3::new(
                -1_f64, 1_f64, 0_f64, -1_f64, 0_f64, 0_f64, 0_f64, 0_f64, -1_f64,
            ),
            Matrix3::new(
                0_f64, -1_f64, 0_f64, -1_f64, 0_f64, 0_f64, 0_f64, 0_f64, 1_f64,
            ),
            Matrix3::new(
                -1_f64, 1_f64, 0_f64, 0_f64, 1_f64, 0_f64, 0_f64, 0_f64, 1_f64,
            ),
            Matrix3::new(
                1_f64, 0_f64, 0_f64, 1_f64, -1_f64, 0_f64, 0_f64, 0_f64, 1_f64,
            ),
            Matrix3::new(
                0_f64, 1_f64, 0_f64, 1_f64, 0_f64, 0_f64, 0_f64, 0_f64, 1_f64,
            ),
            Matrix3::new(
                1_f64, -1_f64, 0_f64, 0_f64, -1_f64, 0_f64, 0_f64, 0_f64, 1_f64,
            ),
            Matrix3::new(
                -1_f64, 0_f64, 0_f64, -1_f64, 1_f64, 0_f64, 0_f64, 0_f64, 1_f64,
            ),
        ];
        let d6h_space = SymSpace::new([2, 2, 2], &d6h_ops, true);
        println!("d6h: {} operations", d6h_ops.len());
        d6h_space.display_kpoints::<Cell>();
    }

    #[test]
    fn symmetry() {
        d2_space();
        d6h_space();
    }
    #[test]
    fn sym_ops() {
        let ops_3 = Matrix3::from_diagonal(&Vector3::new(-1.0, 1.0, -1.0));
        let ops_4 = Matrix3::from_diagonal(&Vector3::new(-1.0, -1.0, -1.0));
        let set = vec![ops_3, ops_4];
        // println!("{:?}", ops_3 * ops_4);
        set.iter()
            .powerset()
            .filter(|c| !c.is_empty())
            .map(|c| c.into_iter().product::<Matrix3<f64>>())
            .for_each(|m| println!("{:?}", m));
        let ops_5 = Matrix3::new(-1.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Point3::new(1.0 / 2.0, 1.0 / 2.0, 1.0 / 4.0);
        let t_p = ops_5 * p;
        println!("{:?}", t_p)
    }
}
