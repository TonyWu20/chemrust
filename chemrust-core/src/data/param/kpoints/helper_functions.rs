use std::{collections::HashMap, ops::Index};

use crate::data::{format::DataFormat, param::KPoint};

pub fn grid_size_determine(length: f64, spacing: f64) -> u32 {
    let div = length / spacing;
    let rounded = div.round();
    if rounded >= 1.0 {
        rounded as u32
    } else {
        1
    }
}

pub fn mp_grid_generate(norms: &[f64; 3], spacing: f64) -> [u32; 3] {
    let grid: Vec<u32> = norms
        .into_iter()
        .map(|&i| grid_size_determine(i, spacing))
        .collect();
    grid.try_into().unwrap()
}

pub fn fractional_num(q: u32) -> Vec<f64> {
    (1..=q)
        .into_iter()
        .map(|r| {
            let r: f64 = r as f64;
            let q: f64 = q as f64;
            (2_f64 * r - q - 1_f64) / (2_f64 * q)
        })
        .collect()
}

pub fn reducible_kpts(grid: &[u32; 3]) -> Vec<[f64; 3]> {
    let x_coords = fractional_num(grid[0]);
    let y_coords = fractional_num(grid[1]);
    let z_coords = fractional_num(grid[2]);
    x_coords
        .iter()
        .flat_map(|&x| -> Vec<[f64; 3]> {
            y_coords
                .iter()
                .flat_map(|&y| -> Vec<[f64; 3]> {
                    z_coords
                        .iter()
                        .map(|&z| -> [f64; 3] { [x, y, z] })
                        .collect()
                })
                .collect()
        })
        .collect()
}

fn grid_common_multiple(grid: &[u32; 3]) -> f64 {
    grid.into_iter().fold(1, |res, &i| res * i) as f64 * 2.0
}

fn hash_kpt_key(kpt: &[f64; 3], common_multiple: f64) -> u32 {
    kpt.iter()
        .map(|&f| f * common_multiple)
        .reduce(|acc, e| acc + e)
        .unwrap()
        .round()
        .abs() as u32
}

fn update_translational_hashmap(
    hash_tab: &mut HashMap<u32, [f64; 3]>,
    kpt: &[f64; 3],
    common_multiple: f64,
) {
    // Get the hash key for the kpt.
    let sum = hash_kpt_key(kpt, common_multiple);
    // Check if this key has already been occupied.
    let check = hash_tab.get(&sum);
    match check {
        None => {
            hash_tab.insert(sum, *kpt);
        }
        // Deal with conflict.
        Some(previous) => {
            let first_diff = kpt[0] - previous[0];
            let sec_diff = kpt[1] - previous[1];
            // If occupied, we want the Rx to be positive or zero, and Ry to be positive if Rx is zero.
            if first_diff > 0.0 || (first_diff == 0.0 && sec_diff > 0.0) {
                hash_tab.insert(sum, *kpt);
            } else {
                ()
            }
        }
    }
}

fn kpt_coeff_upper(grid: &[u32; 3]) -> Vec<[i32; 3]> {
    let x_nums = fractional_num_upper(grid[0]);
    let y_nums = fractional_num_upper(grid[1]);
    let z_nums = fractional_num_upper(grid[2]);
    x_nums
        .iter()
        .flat_map(|&x| -> Vec<[i32; 3]> {
            y_nums
                .iter()
                .flat_map(|&y| -> Vec<[i32; 3]> {
                    z_nums.iter().map(|&z| -> [i32; 3] { [x, y, z] }).collect()
                })
                .collect()
        })
        .collect()
}

pub fn filter_translational_sym(grid: &[u32; 3], upper_coeffs: &[[i32; 3]]) -> Vec<[f64; 3]> {
    let d2d1 = (grid[1] * grid[2]) as i32;
    let d1 = grid[2] as i32;
    upper_coeffs
        .iter()
        .filter_map(|upper_coeff| {
            let decimal = upper_coeff[0] * d2d1 + upper_coeff[1] * d1 + upper_coeff[2];
            if decimal >= 0 {
                let frac: Vec<f64> = upper_coeff
                    .iter()
                    .enumerate()
                    .map(|(i, &v)| v as f64 / (2.0 * grid[i] as f64))
                    .collect();
                let frac: [f64; 3] = frac.try_into().unwrap();
                Some(frac)
            } else {
                None
            }
        })
        .collect()
}

pub fn irreducible_kpts(grid: &[u32; 3]) -> Vec<[f64; 3]> {
    let combos = kpt_coeff_upper(grid);
    filter_translational_sym(grid, &combos)
}

pub fn weighted_kpts<T: DataFormat>(kpts: &[[f64; 3]]) -> Vec<KPoint<T>> {
    let origin = [0_f64, 0_f64, 0_f64];
    if let Some(origin_idx) = kpts.iter().position(|&kpt| kpt == origin) {
        let base_weight = 1_f64 / ((kpts.len() * 2 - 1) as f64);
        kpts.iter()
            .enumerate()
            .map(|(i, &kpt)| {
                let [a, b, c] = kpt;
                let weight = if i != origin_idx {
                    base_weight * 2_f64
                } else {
                    base_weight
                };
                KPoint::new([a, b, c, weight])
            })
            .collect()
    } else {
        let base_weight = 1_f64 / (kpts.len() as f64);
        kpts.iter()
            .map(|&kpt| {
                let [a, b, c] = kpt;
                KPoint::new([a, b, c, base_weight])
            })
            .collect()
    }
}

fn fractional_num_upper(size: u32) -> Vec<i32> {
    let size = size as i32;
    (1..=size).into_iter().map(|i| 2 * i - size - 1).collect()
}

#[cfg(test)]
mod test {
    use crate::data::{format::cell::Cell, param::KPoint};

    use super::{irreducible_kpts, weighted_kpts};

    #[test]
    fn hashing() {
        let grid: [u32; 3] = [4, 5, 1];
        let mut irreducible_kpts = irreducible_kpts(&grid);
        irreducible_kpts.reverse();
        let weighted_kpts: Vec<KPoint<Cell>> = weighted_kpts(&irreducible_kpts);
        for kpt in weighted_kpts.iter() {
            let [x, y, z, w] = kpt.content();
            println!("{:7.3}{:7.3}{:7.3}{:7.3}", x, y, z, w);
        }
    }
}
