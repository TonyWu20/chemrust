use std::collections::HashMap;

use itertools::Itertools;
use kd_tree::KdIndexTree;
use nalgebra::Point3;

use super::geometry::{
    Circle, CircleIntersectChecker, CircleIntersectResult, Intersect, Sphere, SphereIntersectResult,
};

pub trait CheckStage {}

#[derive(Default)]
pub struct Ready;
pub struct SphereStage {
    spheres: Vec<Sphere>,
}

impl SphereStage {
    pub fn new(coords: &[Point3<f64>], radius: f64) -> Self {
        Self {
            spheres: coords
                .iter()
                .map(|&center| Sphere::new(center, radius))
                .collect(),
        }
    }
    pub fn get_sphere(&self, id: usize) -> Option<&Sphere> {
        self.spheres.get(id)
    }
}

#[derive(Default)]
pub struct CircleStage {
    sphere_sites: HashMap<usize, Sphere>,
    sphere_cut_points: Vec<CoordinationPoint>,
    circles: Vec<BondingCircle>,
}

#[derive(Debug, Clone, Copy)]
pub struct BondingCircle {
    pub circle: Circle,
    pub connecting_atoms: [usize; 2],
}

impl CircleStage {
    pub fn new(
        sphere_sites: HashMap<usize, Sphere>,
        sphere_cut_points: Vec<CoordinationPoint>,
        circles: Vec<BondingCircle>,
    ) -> Self {
        Self {
            sphere_sites,
            circles,
            sphere_cut_points,
        }
    }
    pub fn get_circle(&self, id: usize) -> Option<&BondingCircle> {
        self.circles.get(id)
    }
}

#[derive(Debug, Clone)]
pub struct PointStage {
    sphere_sites: HashMap<usize, Sphere>,
    circles: Vec<BondingCircle>,
    cut_points: Vec<CoordinationPoint>,
    multi_cn_points: Vec<CoordinationPoint>,
}

impl PointStage {
    pub fn report_spheres(&self) -> String {
        let texts: Vec<String> = self
            .sphere_sites
            .iter()
            .map(|(i, s)| {
                format!(
                    "Sphere Site at {}, radius: {}, atom_id: {}",
                    s.center,
                    s.radius,
                    i + 1
                )
            })
            .collect();
        texts.join("\n")
    }

    pub fn sphere_sites(&self) -> &HashMap<usize, Sphere> {
        &self.sphere_sites
    }

    pub fn circles(&self) -> &[BondingCircle] {
        self.circles.as_ref()
    }

    pub fn cut_points(&self) -> &[CoordinationPoint] {
        self.cut_points.as_ref()
    }

    pub fn multi_cn_points(&self) -> &[CoordinationPoint] {
        self.multi_cn_points.as_ref()
    }
}

macro_rules! impl_check_stage {
    ($($x: ty),*) => {
        $(impl CheckStage for $x {})*
    };
}

impl_check_stage!(Ready, SphereStage, CircleStage, PointStage);

#[derive(Debug, Clone)]
pub struct CoordinationPoint {
    coord: Point3<f64>,
    connecting_atom_ids: Vec<usize>,
    cn: u32,
}

impl CoordinationPoint {
    pub fn merge_with(self, rhs: Self) -> Option<CoordinationPoint> {
        if self.coord == rhs.coord {
            let new_connecting_atom_ids = vec![self.connecting_atom_ids, rhs.connecting_atom_ids];
            let mut new_connecting_atom_ids = new_connecting_atom_ids.concat();
            new_connecting_atom_ids.sort();
            new_connecting_atom_ids.dedup();
            let new_cn = new_connecting_atom_ids.len();
            Some(CoordinationPoint {
                coord: self.coord,
                connecting_atom_ids: new_connecting_atom_ids,
                cn: new_cn as u32,
            })
        } else {
            None
        }
    }

    pub fn connecting_atom_ids(&self) -> &[usize] {
        self.connecting_atom_ids.as_ref()
    }

    pub fn cn(&self) -> u32 {
        self.cn
    }

    pub fn coord(&self) -> Point3<f64> {
        self.coord
    }
}

#[derive(Debug, Clone)]
pub enum MountingSite {
    Single(Sphere),
    DoubleCircle(BondingCircle),
    Point(CoordinationPoint),
}

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
        let spheres: Vec<Sphere> = self
            .coords
            .iter()
            .map(|&xyz| Sphere::new(xyz, radius))
            .collect();
        IntersectChecker {
            coords: self.coords,
            coords_kdtree: self.coords_kdtree,
            state: SphereStage { spheres },
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
    fn analyze_sphere_intersects(&self) -> HashMap<usize, MountingSite> {
        let results: Vec<(usize, MountingSite)> = self
            .coords
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let found = self.coords_kdtree.nearests(p, 2);
                let found_id = found.get(1).unwrap().item;
                let found_sphere = self.state.get_sphere(*found_id).unwrap();
                let this_sphere = self.state.get_sphere(i).unwrap();
                let intersect_result = this_sphere.intersects(&found_sphere);
                match intersect_result {
                    SphereIntersectResult::Zero => Some((i, MountingSite::Single(*this_sphere))),
                    SphereIntersectResult::SinglePoint(p) => Some((
                        i,
                        MountingSite::Point(CoordinationPoint {
                            coord: p,
                            connecting_atom_ids: vec![i, *found_id],
                            cn: 2,
                        }),
                    )),
                    SphereIntersectResult::Circle(c) => Some((
                        i,
                        MountingSite::DoubleCircle(BondingCircle {
                            circle: c,
                            connecting_atoms: [i, *found_id],
                        }),
                    )),
                    _ => None,
                }
            })
            .filter_map(|res| res)
            .collect();
        HashMap::from_iter(results)
    }
    /// Transition to `CircleStage` by collecting pure spheres, single points,
    /// and circles from current stage intersect results.
    /// # Caution: This function consumes self, copy data to create a new `IntersectChecker`
    pub fn check_spheres(self) -> IntersectChecker<'a, CircleStage> {
        let first_mounting_sites = self.analyze_sphere_intersects();
        let mut circles: Vec<BondingCircle> = Vec::new();
        let mut spheres_map: HashMap<usize, Sphere> = HashMap::new();
        let mut points_only_sites: Vec<CoordinationPoint> = Vec::new();
        first_mounting_sites.iter().for_each(|res| match res {
            (_, MountingSite::DoubleCircle(c)) => circles.push(*c),
            (i, MountingSite::Single(sphere)) => {
                spheres_map.insert(*i, *sphere);
            }
            (_, MountingSite::Point(p)) => points_only_sites.push(p.clone()),
        });
        let Self {
            coords,
            coords_kdtree,
            state: _,
        } = self;
        IntersectChecker {
            coords,
            coords_kdtree,
            state: CircleStage::new(spheres_map, points_only_sites, circles),
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
        let atom_from_this = circle_a.connecting_atoms.to_vec();
        let atom_from_found = circle_b.connecting_atoms.to_vec();
        let connecting_atoms = vec![atom_from_this, atom_from_found];
        CoordinationPoint {
            coord: *point,
            connecting_atom_ids: connecting_atoms.concat(),
            cn: 4,
        }
    }
    pub fn analyze_circle_intersects(self) -> IntersectChecker<'a, PointStage> {
        let circle_centers: Vec<Point3<f64>> =
            self.state.circles.iter().map(|c| c.circle.center).collect();
        let circles_kdtree = KdIndexTree::build_by_ordered_float(&circle_centers);
        let mut pure_circles = Vec::new();
        let mut points_only_sites: Vec<CoordinationPoint> = Vec::new();
        self.state.circles.iter().for_each(|bond_circle| {
            let center = bond_circle.circle.center;
            let found = circles_kdtree.nearests(&center, 2);
            let found_id = found.get(1).unwrap().item;
            let found_circle = self.state.get_circle(*found_id).unwrap();
            let res =
                CircleIntersectChecker::new(&bond_circle.circle, &found_circle.circle).check();
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
        let point_stage = PointStage {
            sphere_sites: self.state.sphere_sites,
            circles: pure_circles,
            cut_points: self.state.sphere_cut_points,
            multi_cn_points: dedup_point_only,
        };
        IntersectChecker {
            coords: self.coords,
            coords_kdtree: self.coords_kdtree,
            state: point_stage,
        }
    }
    fn analyze_points(points: &mut [CoordinationPoint]) -> Vec<CoordinationPoint> {
        points.sort_by(|a, b| {
            a.coord
                .x
                .partial_cmp(&b.coord.x)
                .unwrap_or_else(|| panic!("Comparing {} to {}", a.coord, b.coord))
        });
        let point_xyzs: Vec<Point3<f64>> = points.iter().map(|cp| cp.coord).collect();
        let dedup_points = points.iter().dedup_by_with_count(|a, b| {
            (a.coord.x - b.coord.x).abs() < 1e-6
                && (a.coord.y - b.coord.y).abs() < 1e-6
                && (a.coord.z - b.coord.z).abs() < 1e-6
        });
        dedup_points
            .clone()
            .into_iter()
            .for_each(|(i, p)| println!("{}, {}", i, p.coord));
        let point_kdtree = KdIndexTree::build_by_ordered_float(&point_xyzs);
        dedup_points
            .into_iter()
            .map(|(_, p)| {
                let this_coord = p.coord;
                let found = point_kdtree.within_radius(&this_coord, 0.1);
                if found.len() == 1 {
                    p.clone()
                } else {
                    let total_connected_atoms_vec: Vec<Vec<usize>> = found
                        .iter()
                        .map(|&&i| points[i].connecting_atom_ids.clone())
                        .collect();
                    let mut total_atoms = total_connected_atoms_vec.concat();
                    total_atoms.sort();
                    total_atoms.dedup();
                    let cn = total_atoms.len() as u32;
                    CoordinationPoint {
                        coord: p.coord,
                        connecting_atom_ids: total_atoms,
                        cn,
                    }
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
