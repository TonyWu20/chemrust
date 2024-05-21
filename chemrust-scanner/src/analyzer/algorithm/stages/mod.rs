use chemrust_core::data::{Atom, BasicLatticeModel};
use nalgebra::Point3;

use crate::analyzer::geometry::Sphere;

use super::{BondingCircle, BondingSphere, CoordinationPoint, Visualize};

pub trait CheckStage {}
#[derive(Default)]
pub struct Ready;
pub struct SphereStage {
    spheres: Vec<Sphere>,
    radius: f64,
}

impl SphereStage {
    pub fn new(coords: &[Point3<f64>], radius: f64) -> Self {
        Self {
            spheres: coords
                .iter()
                .map(|&center| Sphere::new(center, radius))
                .collect(),
            radius,
        }
    }
    pub fn get_sphere(&self, id: usize) -> Option<&Sphere> {
        self.spheres.get(id)
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

#[derive(Default)]
pub struct CircleStage {
    pub sphere_sites: Vec<BondingSphere>,
    pub sphere_cut_points: Vec<CoordinationPoint>,
    pub circles: Vec<BondingCircle>,
}

impl CircleStage {
    pub fn new(
        sphere_sites: Vec<BondingSphere>,
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
    pub(crate) sphere_sites: Vec<BondingSphere>,
    pub(crate) circles: Vec<BondingCircle>,
    pub(crate) cut_points: Vec<CoordinationPoint>,
    pub(crate) multi_cn_points: Vec<CoordinationPoint>,
}

impl PointStage {
    pub fn new(
        sphere_sites: Vec<BondingSphere>,
        circles: Vec<BondingCircle>,
        cut_points: Vec<CoordinationPoint>,
        multi_cn_points: Vec<CoordinationPoint>,
    ) -> Self {
        Self {
            sphere_sites,
            circles,
            cut_points,
            multi_cn_points,
        }
    }

    pub fn sphere_sites(&self) -> &[BondingSphere] {
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

    pub fn multi_cn_points_mut(&mut self) -> &mut Vec<CoordinationPoint> {
        &mut self.multi_cn_points
    }
}

#[derive(Debug, Clone)]
pub struct FinalReport {
    pub(crate) sphere_sites: Vec<BondingSphere>,
    pub(crate) circles: Vec<BondingCircle>,
    pub(crate) cut_points: Vec<CoordinationPoint>,
    pub(crate) multi_cn_points: Vec<CoordinationPoint>,
}

impl FinalReport {
    pub fn new(
        sphere_sites: Vec<BondingSphere>,
        circles: Vec<BondingCircle>,
        cut_points: Vec<CoordinationPoint>,
        multi_cn_points: Vec<CoordinationPoint>,
    ) -> Self {
        Self {
            sphere_sites,
            circles,
            cut_points,
            multi_cn_points,
        }
    }

    pub fn sphere_sites(&self) -> &[BondingSphere] {
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

    pub fn report_spheres(&self) -> String {
        let texts: Vec<String> = self
            .sphere_sites
            .iter()
            .map(|s| {
                format!(
                    "Sphere Site at {}, radius: {}, atom_id: {}",
                    s.sphere().center,
                    s.sphere().radius,
                    s.locating_atom_id() + 1
                )
            })
            .collect();
        texts.join("\n")
    }

    pub fn generate_sphere_models(
        &self,
        lattice_model: &BasicLatticeModel,
        new_element_symbol: &str,
    ) -> Option<Vec<(String, BasicLatticeModel)>> {
        if !self.sphere_sites.is_empty() {
            Some(
                self.sphere_sites()
                    .iter()
                    .map(|sphere| {
                        let location = sphere.locating_atom_id();
                        let mut new_atoms = sphere.draw_with_element(new_element_symbol);
                        let mut new_lattice = lattice_model.clone();
                        let current_num = new_lattice.number_of_atoms();
                        new_atoms
                            .iter_mut()
                            .for_each(|atom| atom.set_index(current_num + atom.index()));
                        new_lattice.append_atom(&mut new_atoms);
                        let new_name = format!("single_atom_{}", location);
                        (new_name, new_lattice)
                    })
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn generate_circle_models(
        &self,
        lattice_model: &BasicLatticeModel,
        new_element_symbol: &str,
    ) -> Option<Vec<(String, BasicLatticeModel)>> {
        if !self.circles.is_empty() {
            Some(
                self.circles()
                    .iter()
                    .map(|circle| {
                        let location = circle.connecting_atoms();
                        let mut new_atoms = circle.draw_with_element(new_element_symbol);
                        let mut new_lattice = lattice_model.clone();
                        new_lattice.append_atom(&mut new_atoms);
                        let new_name = format!("double_atom_{}_{}", location[0], location[1]);
                        (new_name, new_lattice)
                    })
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn generate_cut_point_models(
        &self,
        lattice_model: &BasicLatticeModel,
        new_element_symbol: &str,
    ) -> Option<Vec<(String, BasicLatticeModel)>> {
        if !self.cut_points.is_empty() {
            Some(
                self.cut_points()
                    .iter()
                    .map(|cut_point| {
                        let location = cut_point
                            .connecting_atom_ids()
                            .iter()
                            .map(|i| format!("{i}"))
                            .collect::<Vec<String>>()
                            .join("_");
                        let mut new_atoms = cut_point.draw_with_element(new_element_symbol);
                        let mut new_lattice = lattice_model.clone();
                        new_lattice.append_atom(&mut new_atoms);
                        let new_name = format!(
                            "cn_{}_point_atom_{}",
                            cut_point.connecting_atom_ids().len(),
                            location
                        );
                        (new_name, new_lattice)
                    })
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn generate_multi_point_models(
        &self,
        lattice_model: &BasicLatticeModel,
        new_element_symbol: &str,
    ) -> Option<Vec<(String, BasicLatticeModel)>> {
        if !self.multi_cn_points.is_empty() {
            Some(
                self.multi_cn_points()
                    .iter()
                    .map(|pt| {
                        let location = pt
                            .connecting_atom_ids()
                            .iter()
                            .map(|i| format!("{i}"))
                            .collect::<Vec<String>>()
                            .join("_");
                        let mut new_atoms = pt.draw_with_element(new_element_symbol);
                        let mut new_lattice = lattice_model.clone();
                        new_lattice.append_atom(&mut new_atoms);
                        let new_name = format!(
                            "cn_{}_point_atom_{}",
                            pt.connecting_atom_ids().len(),
                            location
                        );
                        (new_name, new_lattice)
                    })
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn visualize_atoms(&self) -> Vec<Atom> {
        let spheres: Vec<Atom> = self.visualize_specific_sites(self.sphere_sites());
        let circles: Vec<Atom> = self.visualize_specific_sites(self.circles());
        let points = [self.cut_points(), self.multi_cn_points()].concat();
        let all_points: Vec<Atom> = self.visualize_specific_sites(&points);
        [spheres, circles, all_points].concat()
    }

    /// Export all possible sites of one kind at once.
    /// E.g., all `BondingSphere`, all `CoordinationPoint`
    pub fn visualize_specific_sites<T>(&self, coordination_sites: &[T]) -> Vec<Atom>
    where
        T: Visualize<Output = Vec<Atom>>,
    {
        coordination_sites
            .iter()
            .map(|site| site.draw_with_atoms())
            .collect::<Vec<Vec<Atom>>>()
            .concat()
    }
}

macro_rules! impl_check_stage {
    ($($x: ty),*) => {
        $(impl CheckStage for $x {})*
    };
}

impl_check_stage!(Ready, SphereStage, CircleStage, PointStage, FinalReport);
