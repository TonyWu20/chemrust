use std::collections::{HashMap, HashSet};

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

struct SymmetrySpace {
    grid: [u32; 3],
    points: Vec<Point3<f64>>,
    lookup_tab: HashMap<i32, Point3<f64>>,
    // size is: [[u32; N_points]; N_operations]
    symmetry_numbers: Vec<Vec<i32>>,
    // A transposed view of `symmetry_numbers`
    equivalences: Option<Vec<Vec<u32>>>,
}

impl SymmetrySpace {
    fn from_grid(grid: &[u32; 3]) -> Self {
        let points = all_reducible_kpts_in_grid(grid);
        let symmetry_numbers: Vec<i32> = points.iter().map(|&p| p.sym_num(grid)).collect();
        let pair: Vec<(i32, Point3<f64>)> = symmetry_numbers
            .iter()
            .zip(points.iter())
            .map(|(&num, point)| -> (i32, Point3<f64>) { (num, *point) })
            .collect();
        let lookup_tab = HashMap::from_iter(pair.into_iter());
        Self {
            grid: *grid,
            points,
            lookup_tab,
            symmetry_numbers: vec![symmetry_numbers],
            equivalences: None,
        }
    }
    fn add_symmetry_operations(&mut self, ops: &Matrix3<f64>) {
        let new_sym_nums: Vec<i32> = self
            .points
            .iter()
            .map(|point| {
                let transformed = ops * point;
                transformed.sym_num(&self.grid)
            })
            .collect();
        self.symmetry_numbers.push(new_sym_nums);
    }
    fn view_equivalences(&mut self) {
        let point_nums = self.points.len();
        let equiv: Vec<Vec<u32>> = (0..point_nums)
            .into_iter()
            .map(|idx| -> Vec<u32> {
                let mut combos: Vec<u32> = self
                    .symmetry_numbers
                    .iter()
                    .map(|array| array[idx].abs() as u32)
                    .collect();
                let redup: HashSet<u32> = HashSet::from_iter(combos.drain(..).into_iter());
                let mut result: Vec<u32> = redup.into_iter().collect();
                result.sort();
                result
            })
            .collect();
        self.equivalences = Some(equiv);
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

fn all_reducible_kpts_in_grid(grid: &[u32; 3]) -> Vec<Point3<f64>> {
    (1..=grid[0] as i32)
        .into_iter()
        .flat_map(|x| -> Vec<Point3<f64>> {
            (1..=grid[1] as i32)
                .into_iter()
                .flat_map(|y| -> Vec<Point3<f64>> {
                    (1..=grid[2] as i32)
                        .into_iter()
                        .map(|z| -> Point3<f64> {
                            let get_frac = |r: i32, q: u32| -> f64 {
                                (2 * r - q as i32 - 1) as f64 / (2_f64 * q as f64)
                            };
                            Point3::new(
                                get_frac(x, grid[0]),
                                get_frac(y, grid[1]),
                                get_frac(z, grid[2]),
                            )
                        })
                        .collect()
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use nalgebra::{Matrix3, Point3, Vector3};

    use super::SymmetrySpace;
    fn gen_kpoint(grid: &[u32; 3], operations: &[Matrix3<f64>]) {
        let mut space = SymmetrySpace::from_grid(&grid);
        operations
            .iter()
            .for_each(|ops| space.add_symmetry_operations(ops));
        space.view_equivalences();
        let equiv_set: HashSet<Vec<u32>> =
            HashSet::from_iter(space.equivalences.as_mut().unwrap().drain(..).into_iter());
        let mut cds: Vec<Point3<f64>> = equiv_set
            .iter()
            .map(|array| {
                let key = *array.iter().max().unwrap() as i32;
                *space.lookup_tab.get(&key).unwrap()
            })
            .collect();
        cds.sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap());
        cds.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
        cds.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
        cds.reverse();
        let total_num = equiv_set
            .iter()
            .map(|set| set.len())
            .reduce(|acc, x| acc + x)
            .unwrap();
        let base_weight = 1_f64 / total_num as f64;
        let weights: Vec<f64> = equiv_set
            .iter()
            .map(|set| set.len() as f64 * base_weight)
            .collect();
        cds.iter()
            .zip(weights.iter())
            .for_each(|(kpt, w)| println!("{:20.16}{:20.16}{:20.16}{:7.3}", kpt.x, kpt.y, kpt.z, w))
    }

    #[test]
    fn symmetry() {
        let ops_1 = Matrix3::from_diagonal(&Vector3::new(-1.0, -1.0, -1.0));
        let ops_2 = Matrix3::from_diagonal(&Vector3::new(1.0, -1.0, -1.0));
        let ops_3 = Matrix3::from_diagonal(&Vector3::new(-1.0, 1.0, -1.0));
        let ops_4 = Matrix3::from_diagonal(&Vector3::new(-1.0, -1.0, 1.0));
        let d2_grid: [u32; 3] = [3, 4, 5];
        println!("d2, grid size: {:?}", d2_grid);
        gen_kpoint(&d2_grid, &[ops_1, ops_2, ops_3, ops_4]);
        let c2_grid = [5, 5, 5];
        let c2_ops = [ops_1, ops_3];
        println!("c2, grid size: {:?}", c2_grid);
        gen_kpoint(&c2_grid, &c2_ops);
    }
}
