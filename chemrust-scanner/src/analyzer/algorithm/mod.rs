use itertools::Itertools;
use kd_tree::KdIndexTree;
use nalgebra::Point3;

use super::geometry::{
    CircleIntersectChecker, CircleIntersectResult, Intersect, SphereIntersectResult,
};

mod coordination_sites;
mod stages;
pub use coordination_sites::*;
pub use stages::*;

pub struct IntersectChecker<'a, T: CheckStage> {
    coords: &'a [Point3<f64>],
    coords_kdtree: KdIndexTree<'a, Point3<f64>>,
    state: T,
}

impl<'a, T: CheckStage> IntersectChecker<'a, T> {
    pub fn new(coords: &'a [Point3<f64>]) -> IntersectChecker<Ready> {
        let coords_kdtree = KdIndexTree::build_by_ordered_float(coords);
        IntersectChecker {
            coords,
            coords_kdtree,
            state: Ready::default(),
        }
    }
}

impl<'a> IntersectChecker<'a, Ready> {
    pub fn start_with_radius(self, radius: f64) -> IntersectChecker<'a, SphereStage> {
        IntersectChecker {
            coords: self.coords,
            coords_kdtree: self.coords_kdtree,
            state: SphereStage::new(&self.coords, radius),
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
        self.coords.iter().enumerate().for_each(|(i, p)| {
            let found = self.coords_kdtree.nearests(p, 2);
            let found_id = found.get(1).unwrap().item;
            let found_sphere = self.state.get_sphere(*found_id).unwrap();
            let this_sphere = self.state.get_sphere(i).unwrap();
            let intersect_result = this_sphere.intersects(&found_sphere);
            match intersect_result {
                SphereIntersectResult::Zero => spheres.push(BondingSphere::new(*this_sphere, i)),
                SphereIntersectResult::SinglePoint(p) => {
                    points_only_sites.push(CoordinationPoint::new(p, vec![i, *found_id], 2))
                }
                SphereIntersectResult::Circle(c) => {
                    circles.push(BondingCircle::new(c, [i, *found_id]))
                }
                _ => (),
            }
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
        let circle_centers: Vec<Point3<f64>> = self
            .state
            .circles
            .iter()
            .map(|c| c.circle().center)
            .collect();
        let circles_kdtree = KdIndexTree::build_by_ordered_float(&circle_centers);
        let mut pure_circles = Vec::new();
        let mut points_only_sites: Vec<CoordinationPoint> = Vec::new();
        self.state.circles.iter().for_each(|bond_circle| {
            let center = bond_circle.circle().center;
            let found = circles_kdtree.nearests(&center, 2);
            let found_id = found.get(1).unwrap().item;
            let found_circle = self.state.get_circle(*found_id).unwrap();
            let res =
                CircleIntersectChecker::new(&bond_circle.circle(), &found_circle.circle()).check();
            match res {
                CircleIntersectResult::Zero => pure_circles.push(*bond_circle),
                CircleIntersectResult::Single(p) => {
                    let point = Self::join_circle_points(&p, bond_circle, found_circle);
                    points_only_sites.push(point);
                }
                CircleIntersectResult::Double(points) => {
                    let point_1 = Self::join_circle_points(&points.0, bond_circle, found_circle);
                    let point_2 = Self::join_circle_points(&points.1, bond_circle, found_circle);
                    points_only_sites.push(point_1);
                    points_only_sites.push(point_2);
                }
                _ => (),
            }
        });
        let dedup_point_only = Self::analyze_points(&mut points_only_sites);
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
        points.sort_by(|a, b| {
            a.coord()
                .x
                .partial_cmp(&b.coord().x)
                .unwrap_or_else(|| panic!("Comparing {} to {}", a.coord(), b.coord()))
        });
        let point_xyzs: Vec<Point3<f64>> = points.iter().map(|cp| cp.coord()).collect();
        let dedup_points = points.iter().dedup_by_with_count(|a, b| {
            (a.coord().x - b.coord().x).abs() < 1e-6
                && (a.coord().y - b.coord().y).abs() < 1e-6
                && (a.coord().z - b.coord().z).abs() < 1e-6
        });
        let point_kdtree = KdIndexTree::build_by_ordered_float(&point_xyzs);
        dedup_points
            .into_iter()
            .map(|(_, p)| {
                let this_coord = p.coord();
                let found = point_kdtree.within_radius(&this_coord, 0.1);
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
            .collect()
    }
}

impl<'a> IntersectChecker<'a, PointStage> {
    pub fn report(&self) -> &PointStage {
        &self.state
    }
}
