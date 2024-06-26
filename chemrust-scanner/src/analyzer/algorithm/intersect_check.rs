use std::collections::HashSet;

use crate::analyzer::geometry::{
    CircleIntersectChecker, CircleIntersectResult, Intersect, SphereIntersectResult,
};
use itertools::Itertools;
use kd_tree::KdIndexTree;
use nalgebra::{distance, Point3};

use super::{
    BondingCircle, BondingSphere, CheckStage, CircleStage, CoordinationPoint, FinalReport,
    PointStage, Ready, SphereStage,
};

pub struct IntersectChecker<'a, T: CheckStage> {
    coords: &'a [Point3<f64>],
    selected_coords: &'a [Point3<f64>],
    coords_kdtree: KdIndexTree<'a, Point3<f64>>,
    bondlength: f64,
    state: T,
}

impl<'a> IntersectChecker<'a, Ready> {
    pub fn new(coords: &'a [Point3<f64>]) -> Self {
        let coords_kdtree = KdIndexTree::build_by_ordered_float(coords);
        IntersectChecker {
            coords,
            coords_kdtree,
            bondlength: 0.0,
            state: Ready,
            selected_coords: coords,
        }
    }
    pub fn set_check_atoms(self, to_check_atoms: &'a [Point3<f64>]) -> Self {
        IntersectChecker {
            selected_coords: to_check_atoms,
            ..self
        }
    }
    pub fn start_with_radius(self, radius: f64) -> IntersectChecker<'a, SphereStage> {
        IntersectChecker {
            coords: self.coords,
            coords_kdtree: self.coords_kdtree,
            selected_coords: self.selected_coords,
            bondlength: radius,
            state: SphereStage::new(self.coords, radius),
        }
    }
}

impl<'a> IntersectChecker<'a, SphereStage> {
    /// Returns the analyze sphere intersects of this [`IntersectChecker<SphereStage>`].
    /// The `SphereIntersectResult` is pattern-matched to corresponding `MountingSite` types.
    /// # Notes:
    /// - `SphereIntersectResult::Zero` => No sphere intersects, this sphere represent a single coordination site around the atom.
    /// - `SphereIntersectResult::SinglePoint` => Two spheres cut at one point. The `C.N.` of a point is at least two, considering later check of repetitions from other possibilities, e.g. many circles crossing at one point.
    /// - `SphereIntersectResult::Circle` => The two spheres intersect as a circle, every point on this circle satisfy bonding two atoms at the same time.
    fn analyze_sphere_intersects(&self) -> CircleStage {
        let mut circles: Vec<BondingCircle> = Vec::new();
        let mut spheres: Vec<BondingSphere> = Vec::new();
        let mut points_only_sites: Vec<CoordinationPoint> = Vec::new();
        let radius = self.state.radius();
        let mut checked_pairs: HashSet<[usize; 2]> = HashSet::new();
        self.selected_coords.iter().enumerate().for_each(|(_, p)| {
            let found = self.coords_kdtree.within_radius(p, 2.0 * radius);
            let original_id = self.coords.iter().position(|&op| *p == op).unwrap();
            let this_sphere = self.state.get_sphere(original_id).unwrap();
            found
                .iter()
                .filter(|found_id| -> bool {
                    if original_id == ***found_id {
                        return false;
                    }
                    let mut pair: [usize; 2] = [original_id, ***found_id];
                    pair.sort();
                    checked_pairs.insert(pair)
                })
                // If the pair atoms of found id and current id has been documented, the `insert` will return false, so the checked atom pairs will be skipped
                .for_each(|new_id| {
                    // the remain ids are new
                    let found_sphere = self.state.get_sphere(**new_id).unwrap();
                    let intersect_result = this_sphere.intersects(found_sphere);
                    match intersect_result {
                        SphereIntersectResult::Zero => {
                            spheres.push(BondingSphere::new(*this_sphere, original_id))
                        }
                        SphereIntersectResult::SinglePoint(p) => points_only_sites
                            .push(CoordinationPoint::new(p, vec![original_id, **new_id], 2)),
                        SphereIntersectResult::Circle(c) => {
                            circles.push(BondingCircle::new(c, [original_id, **new_id]))
                        }
                        _ => (),
                    }
                })
        });
        CircleStage::new(spheres, points_only_sites, circles)
    }
    /// Transition to `CircleStage` by collecting pure spheres, single points,
    /// and circles from current stage intersect results.
    /// # Caution: This function consumes self, copy data to create a new `IntersectChecker`
    pub fn check_spheres(self) -> IntersectChecker<'a, CircleStage> {
        let circle_stage = self.analyze_sphere_intersects();
        let Self {
            coords,
            selected_coords,
            coords_kdtree,
            bondlength,
            state: _,
        } = self;
        IntersectChecker {
            coords,
            coords_kdtree,
            selected_coords,
            bondlength,
            state: circle_stage,
        }
    }
}

impl<'a> IntersectChecker<'a, CircleStage> {
    /// Returns the analyze circle intersects of this [`IntersectChecker<CircleStage>`].
    /// Bug when c2 contains c1
    fn join_circle_points(
        &self,
        point: &Point3<f64>,
        circle_a: &BondingCircle,
        circle_b: &BondingCircle,
    ) -> CoordinationPoint {
        let atom_from_this = circle_a.connecting_atoms().to_vec();
        let atom_from_found = circle_b.connecting_atoms().to_vec();
        let connecting_atoms = [atom_from_this, atom_from_found];
        let mut connecting_atoms = connecting_atoms.concat();
        connecting_atoms.sort();
        connecting_atoms.dedup();
        let real_connecting_atoms: Vec<usize> = connecting_atoms
            .into_iter()
            .filter(|&atom_id| {
                let distance = distance(point, self.coords.get(atom_id).unwrap());
                (distance - self.bondlength).abs() <= 1e-6
            })
            .collect();
        let coordination_number = real_connecting_atoms.len() as u32;
        CoordinationPoint::new(*point, real_connecting_atoms, coordination_number)
    }
    pub fn analyze_circle_intersects(self) -> IntersectChecker<'a, PointStage> {
        let mut pure_circles = Vec::new();
        let mut points_only_sites: Vec<CoordinationPoint> = Vec::new();
        let mut checked_pairs: HashSet<[usize; 2]> = HashSet::new();
        self.state
            .circles
            .iter()
            .enumerate()
            .for_each(|(id_main, now_bond_circle)| {
                let mut zero_intersect_count: usize = 0;
                let mut to_check: usize = 0;
                self.state
                    .circles
                    .iter()
                    .enumerate()
                    .filter(|&(id_sub, _)| match id_main == id_sub {
                        true => false,
                        false => {
                            let mut pair = [id_main, id_sub];
                            pair.sort();
                            checked_pairs.insert(pair)
                        }
                    })
                    .for_each(|(_, bonding_circle)| {
                        to_check += 1;
                        let res = CircleIntersectChecker::new(
                            &now_bond_circle.circle(),
                            &bonding_circle.circle(),
                        )
                        .check();
                        match res {
                            CircleIntersectResult::CoplanarZero => {
                                zero_intersect_count += 1;
                            }
                            CircleIntersectResult::NonCoplanarZero => {
                                zero_intersect_count += 1;
                            }
                            CircleIntersectResult::Single(p) => {
                                let point =
                                    self.join_circle_points(&p, now_bond_circle, bonding_circle);
                                points_only_sites.push(point);
                            }
                            CircleIntersectResult::Double(points) => {
                                let point_1 = self.join_circle_points(
                                    &points.0,
                                    now_bond_circle,
                                    bonding_circle,
                                );
                                let point_2 = self.join_circle_points(
                                    &points.1,
                                    now_bond_circle,
                                    bonding_circle,
                                );
                                points_only_sites.push(point_1);
                                points_only_sites.push(point_2);
                            }
                            _ => (),
                        }
                    });
                if zero_intersect_count == to_check {
                    pure_circles.push(*now_bond_circle)
                }
            });
        pure_circles.dedup_by(|a, b| {
            (a.circle().center.x - b.circle().center.x).abs() < 1e-6
                && (a.circle().center.y - b.circle().center.y).abs() < 1e-6
                && (a.circle().center.z - b.circle().center.z).abs() < 1e-6
        });
        let analzyed_circles = self.analyze_pure_circles(&pure_circles);
        let point_stage = PointStage::new(
            self.state.sphere_sites,
            analzyed_circles,
            self.state.sphere_cut_points,
            points_only_sites,
        );
        IntersectChecker {
            coords: self.coords,
            coords_kdtree: self.coords_kdtree,
            selected_coords: self.selected_coords,
            bondlength: self.bondlength,
            state: point_stage,
        }
    }
    /// 1. Get the nearests atoms around the circle center by `bondlength + circle radius``
    /// 2. Determine the longest possible distance from each atom to the circle
    /// 3. If the longest to circle distance of an atom is shorter than the bondlength, reject
    /// 4. How far should we iterate?
    fn analyze_pure_circles(&self, circles: &[BondingCircle]) -> Vec<BondingCircle> {
        circles
            .iter()
            .filter(|bc| {
                let center = bc.circle().center;

                let atoms_found = self
                    .coords_kdtree
                    .within_radius(&center, bc.circle().radius + self.bondlength);
                for &atom_id in atoms_found {
                    let atom_coord = self.coords.get(atom_id).unwrap();
                    let (min_distance, max_distance) =
                        bc.circle().point_to_circle_distances(atom_coord);
                    if self.bondlength - max_distance > 1e-6 {
                        if bc.connecting_atoms().contains(&atom_id) {
                            println!("{atom_id}, {min_distance}, {max_distance}")
                        }
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect()
    }
}

impl<'a> IntersectChecker<'a, PointStage> {
    pub fn analyze_points(self) -> IntersectChecker<'a, FinalReport> {
        let mut points = self.state.multi_cn_points().to_owned();
        let dedup_point_only = if !points.is_empty() {
            self.merge_points(&mut points)
        } else {
            points
        };
        let final_stage = FinalReport::new(
            self.state.sphere_sites,
            self.state.circles,
            self.state.cut_points,
            dedup_point_only,
        );
        IntersectChecker {
            coords: self.coords,
            coords_kdtree: self.coords_kdtree,
            selected_coords: self.selected_coords,
            bondlength: self.bondlength,
            state: final_stage,
        }
    }
    fn merge_points(&self, points: &mut [CoordinationPoint]) -> Vec<CoordinationPoint> {
        // Floor to clear meaningless digits in f64 for the ease of sort and deduplicate
        points.iter_mut().for_each(|p| {
            let floor_x = (p.coord().x * 1e5).floor() / 1e5;
            let floor_y = (p.coord().y * 1e5).floor() / 1e5;
            let floor_z = (p.coord().z * 1e5).floor() / 1e5;
            p.set_coord(Point3::new(floor_x, floor_y, floor_z))
        });
        points.sort_by(|a, b| {
            a.coord()
                .x
                .partial_cmp(&b.coord().x)
                .unwrap_or_else(|| panic!("Comparing {} to {}", a.coord(), b.coord()))
                .then(a.coord().y.partial_cmp(&b.coord().y).unwrap())
                .then(a.coord().z.partial_cmp(&b.coord().z).unwrap())
        });
        let point_xyzs: Vec<Point3<f64>> = points.iter().map(|cp| cp.coord()).collect();
        let dedup_points: Vec<(usize, &CoordinationPoint)> = points
            .iter()
            .dedup_by_with_count(|a, b| {
                (a.coord().x - b.coord().x).abs() < 1e-5
                    && (a.coord().y - b.coord().y).abs() < 1e-5
                    && (a.coord().z - b.coord().z).abs() < 1e-5
            })
            .collect();
        let point_kdtree = KdIndexTree::build_by_ordered_float(&point_xyzs);
        let res: Vec<CoordinationPoint> = dedup_points
            .into_iter()
            .map(|(_, p)| {
                let this_coord = p.coord();
                let found = point_kdtree.within_radius(&this_coord, 0.00001);
                if found.len() == 1 {
                    p.clone()
                } else {
                    // There are more than one result due to the floating point inaccuracy.
                    // Merge them into one coordination point result
                    let total_connected_atoms_vec: Vec<Vec<usize>> = found
                        .iter()
                        .map(|&&i| points[i].connecting_atom_ids().to_vec())
                        .collect();
                    let mut total_atoms = total_connected_atoms_vec.concat();
                    total_atoms.sort();
                    total_atoms.dedup();
                    let cn = total_atoms.len() as u32;
                    CoordinationPoint::new(p.coord(), total_atoms, cn)
                }
            })
            // New: conditional check to rule out situations that the found point actually has closer connections to atoms in the original lattice model,
            // besides the previously reported connected atoms
            .filter(|cp| {
                let this_coord = cp.coord();
                let found = self
                    .coords_kdtree
                    .within_radius(&this_coord, self.bondlength + 0.00000001);
                let mut found_cn: Vec<usize> = found.iter().map(|id| **id).collect();
                found_cn.sort();
                let mut cn: Vec<usize> = cp.connecting_atom_ids().to_vec();
                cn.sort();
                if found_cn == cn {
                    println!("found {:?}", found_cn);
                    println!("cn {:?}", cn);
                }
                // 0.0001 is the tolerance of floating point comparison.
                // After adding this, no more cases of `found.len() < cp.cn()` is reported
                found.len() == cp.cn() as usize && found_cn == cn
            })
            .collect();
        res
    }
}

impl<'a> IntersectChecker<'a, FinalReport> {
    pub fn report(&self) -> &FinalReport {
        &self.state
    }
}
