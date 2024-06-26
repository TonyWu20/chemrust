use chemrust_core::data::Atom;
use nalgebra::Vector3;

use super::{BondingCircle, BondingSphere, CoordinationPoint};
pub trait Visualize {
    type Output;
    fn draw_with_atoms(&self) -> Self::Output;
    fn draw_with_element(&self, element_symbol: &str) -> Self::Output;
}

impl Visualize for BondingSphere {
    type Output = Vec<Atom>;
    fn draw_with_atoms(&self) -> Self::Output {
        let center = self.sphere.center;
        let radius = self.sphere.radius;
        let z_shift = Vector3::z_axis().scale(radius);
        let report_coord = center + z_shift;
        vec![Atom::new_builder()
            .with_index(0)
            .with_symbol("He")
            .with_coord(&report_coord)
            .ready()
            .build()]
    }
    fn draw_with_element(&self, element_symbol: &str) -> Self::Output {
        let center = self.sphere.center;
        let radius = self.sphere.radius;
        let z_shift = Vector3::z_axis().scale(radius);
        let report_coord = center + z_shift;
        vec![Atom::new_builder()
            .with_index(0)
            .with_symbol(element_symbol)
            .with_coord(&report_coord)
            .ready()
            .build()]
    }
}

impl Visualize for BondingCircle {
    type Output = Vec<Atom>;
    fn draw_with_atoms(&self) -> Self::Output {
        let center = self.circle().center;
        let z_axis = Vector3::z_axis();
        let z_shift = self.circle().radius;
        let repr_coord = center + z_axis.scale(z_shift);
        vec![Atom::new_builder()
            .with_index(0)
            .with_symbol("Ne")
            .with_coord(&repr_coord)
            .ready()
            .build()]
    }
    fn draw_with_element(&self, element_symbol: &str) -> Self::Output {
        let center = self.circle().center;
        let z_axis = Vector3::z_axis();
        let z_shift = self.circle().radius;
        let repr_coord = center + z_axis.scale(z_shift);
        vec![Atom::new_builder()
            .with_index(0)
            .with_symbol(element_symbol)
            .with_coord(&repr_coord)
            .ready()
            .build()]
    }
}

impl Visualize for CoordinationPoint {
    type Output = Vec<Atom>;
    fn draw_with_atoms(&self) -> Self::Output {
        let symbol = match self.connecting_atom_ids().len() {
            3 => "Cu",
            4 => "Fe",
            5 => "W",
            6 => "Pt",
            _ => "Na",
        };
        vec![Atom::new_builder()
            .with_index(0)
            .with_symbol(symbol)
            .with_coord(&self.coord)
            .ready()
            .build()]
    }
    fn draw_with_element(&self, element_symbol: &str) -> Self::Output {
        vec![Atom::new_builder()
            .with_index(0)
            .with_symbol(element_symbol)
            .with_coord(&self.coord)
            .ready()
            .build()]
    }
}
