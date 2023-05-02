use chemrust_core::data::Atom;
use nalgebra::{Rotation3, Vector3};

use super::{BondingCircle, BondingSphere, CoordinationPoint};
pub trait Visualize {
    type Output;
    fn draw_with_atoms(&self) -> Self::Output;
}

impl Visualize for BondingSphere {
    type Output = Vec<Atom>;
    fn draw_with_atoms(&self) -> Self::Output {
        let center = self.sphere.center;
        let radius = self.sphere.radius;
        let x_shift = Vector3::x_axis().scale(radius);
        let y_shift = Vector3::y_axis().scale(radius);
        let z_shift = Vector3::z_axis().scale(radius);
        let repr_coords = vec![
            center + x_shift,
            center - x_shift,
            center + y_shift,
            center - y_shift,
            center + z_shift,
            center - z_shift,
        ];
        repr_coords
            .iter()
            .enumerate()
            .map(|(i, coord)| {
                Atom::new_builder()
                    .with_index(i)
                    .with_symbol("H")
                    .with_atomic_number(0)
                    .with_coord(coord)
                    .finish()
                    .unwrap()
                    .build()
            })
            .collect()
    }
}

impl Visualize for BondingCircle {
    type Output = Vec<Atom>;
    fn draw_with_atoms(&self) -> Self::Output {
        let center = self.circle().center;
        let normal = self.circle().normal;
        let z_axis = Vector3::z_axis();
        let rotation = Rotation3::rotation_between(&z_axis, &normal).unwrap();
        let x_axis = rotation * Vector3::x_axis();
        let y_axis = rotation * Vector3::y_axis();
        let x_shift = x_axis.scale(self.circle().radius);
        let y_shift = y_axis.scale(self.circle().radius);
        [
            center + x_shift,
            center - x_shift,
            center + y_shift,
            center - y_shift,
        ]
        .into_iter()
        .enumerate()
        .map(|(i, coord)| {
            Atom::new_builder()
                .with_index(i)
                .with_symbol("O")
                .with_atomic_number(8)
                .with_coord(&coord)
                .finish()
                .unwrap()
                .build()
        })
        .collect()
    }
}

impl Visualize for CoordinationPoint {
    type Output = Atom;
    fn draw_with_atoms(&self) -> Self::Output {
        Atom::new_builder()
            .with_index(0)
            .with_symbol("Nd")
            .with_atomic_number(60)
            .with_coord(&self.coord)
            .finish()
            .unwrap()
            .build()
    }
}
