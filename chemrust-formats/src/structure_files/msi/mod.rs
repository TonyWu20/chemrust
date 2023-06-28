use std::{fs::File, io::Write};

use chemrust_core::data::{lattice::LatticeVectors, Atom};
use nalgebra::{Matrix3, Rotation3, Unit, Vector3};

use crate::{Cell, ModelFormat};

use super::{FileExport, StructureFile};

#[derive(Debug, Clone, Copy, Default)]
/// A unit struct to mark `msi` format
pub struct Msi;

impl ModelFormat for Msi {}

impl FileExport for StructureFile<Msi> {
    fn write_to<P: AsRef<std::path::Path>>(&self, path: &P) -> Result<(), std::io::Error> {
        let text = self.export_msi();
        let mut f = File::create(path)?;
        write!(f, "{}", text)
    }
}
impl StructureFile<Msi> {
    fn atom_export(atom: &Atom) -> String {
        format!(
            r#"  ({item_id} Atom
    (A C ACL "{elm_id} {elm}")
    (A C Label "{elm}")
    (A D XYZ ({x:.12} {y:.12} {z:.12}))
    (A I Id {atom_id})
  )
"#,
            item_id = atom.index() + 2,
            elm_id = atom.atomic_number(),
            elm = atom.symbol(),
            x = atom.cartesian_coord().x,
            y = atom.cartesian_coord().y,
            z = atom.cartesian_coord().z,
            atom_id = atom.index() + 1
        )
    }
    fn rotate_to_standard_direction(&self) -> Option<Matrix3<f64>> {
        if let Some(vectors) = self.lattice_model.lattice_vectors() {
            let vector_b = vectors.data().column(1);
            let angle = Vector3::y_axis().angle(&vector_b);
            let rot_axis = Unit::new_normalize(vector_b.cross(&Vector3::y_axis()));
            let rot_mat = Rotation3::from_axis_angle(&rot_axis, angle);
            Some(*rot_mat.matrix())
        } else {
            None
        }
    }
    fn rotated_lattice_vector(&self) -> Option<LatticeVectors> {
        if let Some(vectors) = self.lattice_model.lattice_vectors() {
            let rotated_vectors = self.rotate_to_standard_direction().unwrap() * vectors.data();
            Some(LatticeVectors::new(rotated_vectors))
        } else {
            None
        }
    }
    fn lattice_vector_export(vec: &LatticeVectors) -> String {
        // Rotate to let B align with Y
        let vec_names = ["A3", "B3", "C3"];
        let lines: Vec<String> = vec
            .data()
            .column_iter()
            .zip(vec_names.iter())
            .map(|(col, name)| {
                format!(
                    "  (A D {} ({:.12} {:.12} {:.12}))\n",
                    name, col.x, col.y, col.z
                )
            })
            .collect();
        lines.concat()
    }
    pub fn export_msi(&self) -> String {
        let headers = if self.lattice_model.lattice_vectors().is_some() {
            let rotated_vectors = self.rotated_lattice_vector().unwrap();

            format!(
                r#"# MSI CERIUS2 DataModel File Version 4 0
(1 Model
  (A I CRY/DISPLAY (192 256))
  (A I PeriodicType 100)
  (A C SpaceGroup "1 1")
{}  (A D CRY/TOLERANCE 0.05)
"#,
                Self::lattice_vector_export(&rotated_vectors)
            )
        } else {
            "# MSI CERIUS2 DataModel File Version 4 0\n(1 Model\n".to_string()
        };
        let atoms: Vec<String> = match self.lattice_model.lattice_vectors() {
            Some(_) => {
                let rotated_matrix = self.rotate_to_standard_direction().unwrap();
                self.lattice_model
                    .atoms()
                    .iter()
                    .map(|atom| {
                        let rotated_coord = rotated_matrix * atom.cartesian_coord();
                        let rotated_atom = Atom::new_builder()
                            .with_coord(&rotated_coord)
                            .with_index(atom.index())
                            .with_atomic_number(atom.atomic_number())
                            .with_symbol(atom.symbol())
                            .ready()
                            .build();
                        Self::atom_export(&rotated_atom)
                    })
                    .collect()
            }
            None => self
                .lattice_model
                .atoms()
                .iter()
                .map(Self::atom_export)
                .collect(),
        };
        format!("{headers}{atoms_text})", atoms_text = atoms.concat())
    }
}

impl From<StructureFile<Cell>> for StructureFile<Msi> {
    fn from(value: StructureFile<Cell>) -> Self {
        StructureFile::<Msi>::new(value.lattice_model)
    }
}
