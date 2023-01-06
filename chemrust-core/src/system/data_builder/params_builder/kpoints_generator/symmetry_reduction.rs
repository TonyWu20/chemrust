/// Find symmetry equivalent k-points to generate irreducible k-points.
/// # Step:
/// 1. Sample the reciprocal lattice space with specified spacing distance,
/// which results in a [P, Q, R] grid.
/// 2. Generate permutations of k-points from the grid. The fractional coordinates are
/// generated following the formula: f_i = (2 * r - q - 1) / (2 * q), where r is (1..= q)
/// 3. If we take the numerator part of the f_i: (2 * r - q -1), it would generate a series of
/// `i32` which are symmetric around the zero. The k-point coordinate can be expressed as [n_x, n_y, n_z]
/// 4. "Hash" the coordinate by the formula: H = n_x * Q * R + n_y * R + n_z. If |H_1| == |H_2|, they are
/// the most basic symmetry equivalent points: the operation is identical with diagonal to be all -1.
/// 5. Apply other symmetry operations to all points. Calculate the new hash key for the transformed point,
/// lookup the key, if found, merge them in a vector, indicating they are symmetry equivalent.
/// 6. The final table will naturally indicates the degeneracies of the irreducible k-points, which leads to
/// the weight determination of these irreducible k-points. From all the possible k-points, select the one with least
/// negative numbers as the "representative", which is just a convention.
use nalgebra::{Matrix3, Matrix4, Point3, Point4, Vector3, Vector4};
use num::integer::lcm;
use std::collections::{HashMap, HashSet};

use crate::data::{format::DataFormat, param::KPoint};

pub struct SymSpace {
    grid: [u32; 3],
    symmetry_operations: Vec<SymOps>,
    is_hexagonal: bool,
    mesh: [Vec<i32>; 3],
}

#[derive(Debug)]
struct SymKPoint {
    coord: Point3<f64>,
    multiplicity: u32,
    kpt_image: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct SymOps(Matrix3<i32>, i32);

impl SymKPoint {
    fn new(coord: Point3<f64>, multiplicity: u32, kpt_image: Vec<i32>) -> Self {
        Self {
            coord,
            multiplicity,
            kpt_image,
        }
    }
}

trait MPSpace {
    fn mesh(num_points: u32) -> Vec<i32> {
        let q = num_points as i32;
        (1..=q).into_iter().rev().map(|r| 2 * r - q - 1).collect()
    }
    fn hexagonal_mesh(num_points: u32) -> Vec<i32> {
        let q = num_points as i32;
        (1..=q).into_iter().map(|r| 2 * r - q - 2).collect()
    }
}

impl MPSpace for SymSpace {}

impl SymSpace {
    pub fn new(grid: [u32; 3], symmetry_operations: &[SymOps], is_hexagonal: bool) -> Self {
        // let group = symmetry_operations
        //     .iter()
        //     .powerset()
        //     .filter(|c| !c.is_empty())
        //     .map(|c| c.into_iter().product::<Matrix3<f64>>())
        //     .collect();
        let [a, b, c] = grid;
        let a_mesh = if is_hexagonal {
            Self::hexagonal_mesh(a)
        } else {
            Self::mesh(a)
        };
        let b_mesh = if is_hexagonal {
            Self::hexagonal_mesh(b)
        } else {
            Self::mesh(b)
        };
        let c_mesh = Self::mesh(c);
        Self {
            grid,
            symmetry_operations: symmetry_operations.to_vec(),
            is_hexagonal,
            mesh: [a_mesh, b_mesh, c_mesh],
        }
    }
    fn recover_index(&self, address: i32) -> [i32; 3] {
        let [a, b, c] = self.grid;
        [address % a as i32, address % b as i32, address % c as i32]
    }
    fn to_fractional(&self, grid_point: &Point3<i32>) -> Point3<f64> {
        let gp: Point3<f64> = Point3::new(
            grid_point.x as f64,
            grid_point.y as f64,
            grid_point.z as f64,
        );
        let a = 0.5 / self.grid[0] as f64;
        let b = 0.5 / self.grid[1] as f64;
        let c = 0.5 / self.grid[2] as f64;
        let convert_matrix = Matrix3::from_diagonal(&Vector3::new(a, b, c));
        convert_matrix * gp
    }
    fn inspect_symmetry(
        &self,
        point: &Point3<i32>,
        sym_num_hash_set: &mut HashSet<i32>,
        irreducible_points: &mut Vec<SymKPoint>,
    ) {
        let p_hash_key = point.sym_num(&self.grid);
        let mut valid_sym_ops: Vec<i32> = Vec::new();
        valid_sym_ops.push(self.symmetry_operations[0].1);
        // Look up at the table
        match sym_num_hash_set.get(&p_hash_key) {
            // Already added by other points' symmetry operations
            Some(_) => {}
            // New equivalent point found
            None => {
                // Add to hash table
                sym_num_hash_set.insert(p_hash_key);
                let mut local_sym_set: HashMap<i32, i32> = HashMap::new();
                // Apply all symmetry operations in the system
                self.symmetry_operations
                    .iter()
                    .map(|ops| {
                        let sym_point = ops.0 * point;
                        (sym_point.sym_num(&self.grid), ops.1, sym_point)
                    })
                    // Exclude situation when the symmetry operation repeats itself
                    // E.g., the origin (0.0, 0.0, 0.0)
                    .filter(|&n| {
                        n.0 != p_hash_key
                            && self.mesh[0].contains(&n.2.x)
                            && self.mesh[1].contains(&n.2.y)
                            && self.mesh[2].contains(&n.2.z)
                    })
                    .for_each(|(key, i, _)| {
                        if let None = local_sym_set.get(&key) {
                            local_sym_set.insert(key, i);
                        }
                    });
                local_sym_set.iter().for_each(|(key, i)| {
                    sym_num_hash_set.insert(*key);
                    valid_sym_ops.push(*i)
                });
                valid_sym_ops.sort_by(|a, b| {
                    if a.abs() != b.abs() {
                        a.abs().cmp(&b.abs())
                    } else {
                        a.cmp(&b).reverse()
                    }
                });
                let irreducible_point = SymKPoint::new(
                    self.to_fractional(point),
                    1 + local_sym_set.len() as u32,
                    valid_sym_ops,
                );
                irreducible_points.push(irreducible_point);
            }
        }
    }
    fn reduce_with_symmetry(&self) -> Vec<SymKPoint> {
        let mut hash_set: HashSet<i32> = HashSet::new();
        let mut irreducible_kpoints: Vec<SymKPoint> = Vec::new();
        // Iterate over the permutations of the possible k-point coord values
        // along the three directions.
        self.mesh[0].iter().for_each(|a| {
            self.mesh[1].iter().for_each(|b| {
                self.mesh[2].iter().for_each(|c| {
                    // Initialize a point to calculate its hash key.
                    let p: Point3<i32> = Point3::new(*a, *b, *c);
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
    fn display_kpt_images(&self) {
        let ir_points = self.reduce_with_symmetry();
        println!("%BLOCK KPOINT_IMAGES");
        for p in ir_points.iter() {
            let img = &p.kpt_image;
            print!("{:4}", img.len());
            for item in img.iter() {
                print!("{:4}", item)
            }
            println!("");
        }
        println!("%ENDBLOCK KPOINT_IMAGES");
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

impl SymHash for Point3<i32> {
    fn rank(&self, grid: &[u32; 3]) -> i32 {
        let d2d1 = grid[1] * grid[2];
        let d1 = grid[2];
        let address = Vector3::new(d2d1 as i32, d1 as i32, 1);
        let p_v = self - Point3::origin();
        p_v.dot(&address)
    }
}

#[cfg(test)]
mod test {

    use itertools::Itertools;
    use nalgebra::{Matrix3, Point3, Vector3};

    use crate::{
        data::format::cell::Cell,
        system::data_builder::params_builder::kpoints_generator::symmetry_reduction::SymOps,
    };

    use super::SymSpace;
    fn d2_space() {
        let ops_0 = Matrix3::from_diagonal(&Vector3::new(1, 1, 1));
        let ops_1 = Matrix3::from_diagonal(&Vector3::new(-1, -1, -1));
        let ops_2 = Matrix3::from_diagonal(&Vector3::new(-1, -1, 1));
        let ops_3 = Matrix3::from_diagonal(&Vector3::new(1, 1, -1));
        let ops_4 = Matrix3::from_diagonal(&Vector3::new(-1, 1, -1));
        let ops_5 = Matrix3::from_diagonal(&Vector3::new(1, -1, 1));
        let ops_6 = Matrix3::from_diagonal(&Vector3::new(1, -1, -1));
        let ops_7 = Matrix3::from_diagonal(&Vector3::new(-1, 1, 1));
        let sym_ops = vec![
            SymOps(ops_0, 1),
            SymOps(ops_1, -1),
            SymOps(ops_2, 2),
            SymOps(ops_3, -2),
            SymOps(ops_4, 3),
            SymOps(ops_5, -3),
            SymOps(ops_6, 4),
            SymOps(ops_7, -4),
        ];
        let d2_grid: [u32; 3] = [3, 4, 5];
        let d2_space = SymSpace::new(d2_grid, &sym_ops, false);
        println!("d2:");
        d2_space.display_kpoints::<Cell>();
        d2_space.display_kpt_images();
    }
    fn c2h_space() {
        let ops_1 = Matrix3::from_diagonal(&Vector3::new(-1, -1, -1));
        let ops_6 = Matrix3::from_diagonal(&Vector3::new(1, -1, 1));
        let ops_3 = Matrix3::from_diagonal(&Vector3::new(-1, 1, -1));
        let c2h_ops = vec![
            SymOps(Matrix3::from_diagonal_element(1), 1),
            SymOps(ops_1, 2),
            SymOps(ops_3, 3),
            SymOps(ops_6, 4),
        ];
        let c2h_space = SymSpace::new([2, 2, 2], &c2h_ops, false);
        println!("c2h: ");
        c2h_space.display_kpoints::<Cell>();
        c2h_space.display_kpt_images();
    }
    fn c2_space() {
        let c2_ops = vec![
            SymOps(Matrix3::identity(), 1),
            SymOps(Matrix3::from_diagonal(&Vector3::new(-1, -1, -1)), -1),
            SymOps(Matrix3::from_diagonal(&Vector3::new(-1, 1, -1)), 2),
            SymOps(Matrix3::from_diagonal(&Vector3::new(1, -1, 1)), -2),
        ];
        let c2_space = SymSpace::new([5, 5, 5], &c2_ops, false);
        println!("c2:");
        c2_space.display_kpoints::<Cell>();
        c2_space.display_kpt_images();
    }
    fn d6h_space() {
        let d6h_ops = vec![
            SymOps(Matrix3::from_diagonal_element(1), 1),
            SymOps(Matrix3::new(0, 1, 0, -1, 1, 0, 0, 0, 1), 2),
            SymOps(Matrix3::new(1, -1, 0, 1, 0, 0, 0, 0, 1), 3),
            SymOps(Matrix3::new(-1, 0, 0, 0, -1, 0, 0, 0, 1), 4),
            SymOps(Matrix3::new(0, -1, 0, 1, -1, 0, 0, 0, 1), 5),
            SymOps(Matrix3::new(-1, 1, 0, -1, 0, 0, 0, 0, 1), 6),
            SymOps(Matrix3::new(0, 1, 0, 1, 0, 0, 0, 0, -1), 7),
            SymOps(Matrix3::new(-1, 1, 0, 0, 1, 0, 0, 0, -1), 8),
            SymOps(Matrix3::new(1, 0, 0, 1, -1, 0, 0, 0, -1), 9),
            SymOps(Matrix3::new(1, -1, 0, 0, -1, 0, 0, 0, -1), 11),
            SymOps(Matrix3::new(-1, 0, 0, -1, 1, 0, 0, 0, -1), 12),
            SymOps(Matrix3::new(0, -1, 0, -1, 0, 0, 0, 0, -1), 10),
            SymOps(Matrix3::new(-1, 0, 0, 0, -1, 0, 0, 0, -1), 13),
            SymOps(Matrix3::new(0, 1, 0, -1, 1, 0, 0, 0, -1), 14),
            SymOps(Matrix3::new(1, -1, 0, 1, 0, 0, 0, 0, -1), 15),
            SymOps(Matrix3::new(1, 0, 0, 0, 1, 0, 0, 0, -1), 16),
            SymOps(Matrix3::new(0, -1, 0, 1, -1, 0, 0, 0, -1), 17),
            SymOps(Matrix3::new(-1, 1, 0, -1, 0, 0, 0, 0, -1), 18),
            SymOps(Matrix3::new(0, -1, 0, -1, 0, 0, 0, 0, 1), 19),
            SymOps(Matrix3::new(-1, 1, 0, 0, 1, 0, 0, 0, 1), 20),
            SymOps(Matrix3::new(1, 0, 0, 1, -1, 0, 0, 0, 1), 21),
            SymOps(Matrix3::new(0, 1, 0, 1, 0, 0, 0, 0, 1), 22),
            SymOps(Matrix3::new(1, -1, 0, 0, -1, 0, 0, 0, 1), 23),
            SymOps(Matrix3::new(-1, 0, 0, -1, 1, 0, 0, 0, 1), 24),
        ];
        let d6h_space = SymSpace::new([2, 2, 2], &d6h_ops, true);
        println!("d6h: {} operations", d6h_ops.len());
        d6h_space.display_kpoints::<Cell>();
        d6h_space.display_kpt_images();
    }

    #[test]
    fn symmetry() {
        c2h_space();
        d2_space();
        d6h_space();
        c2_space();
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
