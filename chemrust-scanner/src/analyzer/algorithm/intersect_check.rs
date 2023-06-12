use std::collections::HashSet;

use crate::analyzer::geometry::{
    CircleIntersectChecker, CircleIntersectResult, Intersect, SphereIntersectResult,
};
use itertools::Itertools;
use kd_tree::KdIndexTree;
use nalgebra::Point3;

use super::{
    BondingCircle, BondingSphere, CheckStage, CircleStage, CoordinationPoint, PointStage, Ready,
    SphereStage,
};

pub struct IntersectChecker<'a, T: CheckStage> {
    coords: &'a [Point3<f64>],
    coords_kdtree: KdIndexTree<'a, Point3<f64>>,
    state: T,
}

impl<'a> IntersectChecker<'a, Ready> {
    pub fn new(coords: &'a [Point3<f64>]) -> Self {
        let coords_kdtree = KdIndexTree::build_by_ordered_float(coords);
        IntersectChecker {
            coords,
            coords_kdtree,
            state: Ready::default(),
        }
    }
    pub fn start_with_radius(self, radius: f64) -> IntersectChecker<'a, SphereStage> {
        IntersectChecker {
            coords: self.coords,
            coords_kdtree: self.coords_kdtree,
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
        self.coords.iter().enumerate().for_each(|(i, p)| {
            let found = self.coords_kdtree.within_radius(p, 2.0 * radius);
            found
                .iter()
                .skip(1)
                .filter(|found_id| -> bool {
                    let mut pair: [usize; 2] = [i, ***found_id];
                    pair.sort();
                    checked_pairs.insert(pair)
                })
                // If the pair atoms of found id and current id has been documented, the `insert` will return false, so the checked atom pairs will be skipped
                .for_each(|new_id| {
                    // the remain ids are new
                    let found_sphere = self.state.get_sphere(**new_id).unwrap();
                    let this_sphere = self.state.get_sphere(i).unwrap();
                    let intersect_result = this_sphere.intersects(found_sphere);
                    match intersect_result {
                        SphereIntersectResult::Zero => {
                            spheres.push(BondingSphere::new(*this_sphere, i))
                        }
                        SphereIntersectResult::SinglePoint(p) => {
                            points_only_sites.push(CoordinationPoint::new(p, vec![i, **new_id], 2))
                        }
                        SphereIntersectResult::Circle(c) => {
                            circles.push(BondingCircle::new(c, [i, **new_id]))
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
            coords_kdtree,
            state: _,
        } = self;
        IntersectChecker {
            coords,
            coords_kdtree,
            state: circle_stage,
        }
    }
}

impl<'a> IntersectChecker<'a, CircleStage> {
    /// Returns the analyze circle intersects of this [`IntersectChecker<CircleStage>`].
    fn join_circle_points(
        point: &Point3<f64>,
        circle_a: &BondingCircle,
        circle_b: &BondingCircle,
    ) -> CoordinationPoint {
        let atom_from_this = circle_a.connecting_atoms().to_vec();
        let atom_from_found = circle_b.connecting_atoms().to_vec();
        let connecting_atoms = vec![atom_from_this, atom_from_found];
        CoordinationPoint::new(*point, connecting_atoms.concat(), 4)
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
                        true => return false,
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
                            CircleIntersectResult::Zero => {
                                zero_intersect_count += 1;
                            }
                            CircleIntersectResult::Single(p) => {
                                let point =
                                    Self::join_circle_points(&p, now_bond_circle, bonding_circle);
                                points_only_sites.push(point);
                            }
                            CircleIntersectResult::Double(points) => {
                                let point_1 = Self::join_circle_points(
                                    &points.0,
                                    now_bond_circle,
                                    bonding_circle,
                                );
                                let point_2 = Self::join_circle_points(
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
        let dedup_point_only = if !points_only_sites.is_empty() {
            Self::analyze_points(&mut points_only_sites)
        } else {
            points_only_sites
        };
        let point_stage = PointStage::new(
            self.state.sphere_sites,
            pure_circles,
            self.state.sphere_cut_points,
            dedup_point_only,
        );
        IntersectChecker {
            coords: self.coords,
            coords_kdtree: self.coords_kdtree,
            state: point_stage,
        }
    }
    fn analyze_points(points: &mut [CoordinationPoint]) -> Vec<CoordinationPoint> {
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
        dbg!(point_xyzs.len());
        let dedup_points: Vec<(usize, &CoordinationPoint)> = points
            .iter()
            .dedup_by_with_count(|a, b| {
                (a.coord().x - b.coord().x).abs() < 1e-3
                    && (a.coord().y - b.coord().y).abs() < 1e-3
                    && (a.coord().z - b.coord().z).abs() < 1e-3
            })
            .collect();
        let point_kdtree = KdIndexTree::build_by_ordered_float(&point_xyzs);
        dbg!(dedup_points.len());
        let res: Vec<CoordinationPoint> = dedup_points
            .into_iter()
            .map(|(_, p)| {
                let this_coord = p.coord();
                let found = point_kdtree.within_radius(&this_coord, 0.00001);
                if found.len() == 1 {
                    p.clone()
                } else {
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
            .collect();
        dbg!(res.len());
        res
    }
}

impl<'a> IntersectChecker<'a, PointStage> {
    pub fn report(&self) -> &PointStage {
        &self.state
    }
}
