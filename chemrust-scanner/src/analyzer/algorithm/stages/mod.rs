use chemrust_core::data::Atom;
use nalgebra::Point3;

use crate::analyzer::geometry::Sphere;

use super::{BondingCircle, BondingSphere, CoordinationPoint, Visualize};

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
    sphere_sites: Vec<BondingSphere>,
    circles: Vec<BondingCircle>,
    cut_points: Vec<CoordinationPoint>,
    multi_cn_points: Vec<CoordinationPoint>,
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
    pub fn visualize_atoms(&self) -> Vec<Atom> {
        let spheres: Vec<Vec<Atom>> = self
            .sphere_sites()
            .iter()
            .map(|s| s.draw_with_atoms())
            .collect();
        let circles: Vec<Vec<Atom>> = self.circles().iter().map(|c| c.draw_with_atoms()).collect();
        let sphere_cut_points: Vec<Atom> = self
            .cut_points()
            .iter()
            .map(|p| p.draw_with_atoms())
            .collect();
        let multi_points: Vec<Atom> = self
            .multi_cn_points()
            .iter()
            .map(|p| p.draw_with_atoms())
            .collect();
        let total_atoms = vec![
            spheres.concat(),
            circles.concat(),
            sphere_cut_points,
            multi_points,
        ];
        total_atoms.concat()
    }
}

macro_rules! impl_check_stage {
    ($($x: ty),*) => {
        $(impl CheckStage for $x {})*
    };
}

impl_check_stage!(Ready, SphereStage, CircleStage, PointStage);
