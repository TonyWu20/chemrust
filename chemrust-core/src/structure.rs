use castep_periodic_table::element::ElementSymbol;
use crystallographic_group::database::SpaceGroupHallSymbol;
use nalgebra::{Matrix4, Point3, Vector3};

use crate::error::Error;
use crate::lattice::LatticeVectors;
use crate::transform::Transform;

/// Struct-of-arrays. One struct for molecules, crystals, and slabs.
///
/// # Coordinate convention
/// All atomic positions are stored in **fractional coordinates** internally.
/// Cartesian positions are computed on-the-fly: `cart = cell * frac`.
///
/// # Transform model
/// Use `.transform(T)` for cell-independent operations (rotation, supercell).
/// These queue a 4×4 augmented matrix lazily. Call `.apply()` to force it.
/// Cell-dependent operations (vacuum, layer replication) have their own methods.
#[derive(Debug, Clone)]
pub struct Structure {
    pub species: Vec<ElementSymbol>,
    /// Always fractional. Each entry is [x, y, z] in the current cell basis.
    pub frac_coords: Vec<[f64; 3]>,
    /// Periodic cell. `None` for isolated molecules.
    pub cell: Option<LatticeVectors>,
    /// Periodic boundary condition flags.
    pub pbc: [bool; 3],
    /// Integer tags per atom (layer index, constraint flags, etc.).
    pub tags: Vec<i32>,
    /// String labels per atom (CIF site labels, "C_atop", etc.).
    pub labels: Vec<Option<String>>,
    /// Known space group.
    pub space_group: Option<SpaceGroupHallSymbol>,

    /// Pending 4×4 augmented matrix for lazy composition.
    pending: Option<Matrix4<f64>>,
}

impl Structure {
    pub fn new(
        species: Vec<ElementSymbol>,
        frac_coords: Vec<[f64; 3]>,
        cell: Option<LatticeVectors>,
        pbc: [bool; 3],
        tags: Vec<i32>,
        labels: Vec<Option<String>>,
        space_group: Option<SpaceGroupHallSymbol>,
    ) -> Self {
        Self {
            species,
            frac_coords,
            cell,
            pbc,
            tags,
            labels,
            space_group,
            pending: None,
        }
    }

    pub fn num_atoms(&self) -> usize {
        self.species.len()
    }

    /// Cartesian coordinates. Returns `None` if the structure has no cell.
    pub fn cart_coords(&self) -> Option<Vec<[f64; 3]>> {
        let cell = self.cell.as_ref()?;
        Some(
            self.frac_coords
                .iter()
                .map(|&f| {
                    let p = cell.tensor() * Point3::new(f[0], f[1], f[2]);
                    [p.x, p.y, p.z]
                })
                .collect(),
        )
    }

    /// Compose a transform into the pending pipeline.
    ///
    /// Does NOT touch coordinates or cell yet. Call `.apply()` when ready.
    pub fn transform(mut self, t: impl Transform) -> Self {
        let m = t.matrix();
        self.pending = match self.pending {
            None => Some(m),
            Some(existing) => Some(m * existing), // compose: newest acts first
        };
        self
    }

    /// Force-apply the pending transform to cell + all frac_coords.
    ///
    /// This is a no-op if no transform is pending.
    pub fn apply(mut self) -> Self {
        let m = match self.pending.take() {
            None => return self,
            Some(m) => m,
        };

        // Extract the 3×3 linear part. The matrix operates on fractional
        // coords: x' = M * x. The cell transforms inversely: C' = C * M₃₃⁻¹.
        let m33 = m.fixed_view::<3, 3>(0, 0).into_owned();
        let m33_inv = m33
            .try_inverse()
            .expect("Transform matrix is singular");

        // Update cell: C' = C * M₃₃⁻¹
        if let Some(cell) = &mut self.cell {
            let new_tensor = cell.tensor() * m33_inv;
            *cell = LatticeVectors::new(new_tensor);
        }

        // For the translation part, apply to coords: x' = R*x + t
        let t_vec = Vector3::new(m[(0, 3)], m[(1, 3)], m[(2, 3)]);

        for coord in &mut self.frac_coords {
            let old = Point3::new(coord[0], coord[1], coord[2]);
            let new = m33 * old + t_vec;
            *coord = [new.x, new.y, new.z];
        }

        self
    }

    /// Wrap fractional coordinates to [0, 1) by subtracting integer parts.
    /// Forces any pending transform first.
    pub fn wrap_frac_coords(mut self) -> Self {
        self = self.apply();
        for coord in &mut self.frac_coords {
            coord[0] = coord[0] - coord[0].floor();
            coord[1] = coord[1] - coord[1].floor();
            coord[2] = coord[2] - coord[2].floor();
        }
        self
    }

    /// Replicate all atoms along c-axis into N layers.
    ///
    /// This forces any pending transform first. Each layer k gets tag=k.
    /// This is a cell-dependent, non-linear operation.
    pub fn replicate_along_c(mut self, n_layers: usize) -> Self {
        assert!(n_layers >= 1, "n_layers must be at least 1");
        self = self.apply();

        let n_atoms = self.num_atoms();
        let total = n_atoms * n_layers;

        // Pre-allocate
        let mut new_species = Vec::with_capacity(total);
        let mut new_coords = Vec::with_capacity(total);
        let mut new_tags = Vec::with_capacity(total);
        let mut new_labels = Vec::with_capacity(total);

        for k in 0..n_layers {
            let dz = k as f64 / n_layers as f64;
            new_species.extend_from_slice(&self.species);
            new_labels.extend_from_slice(&self.labels);
            new_tags.extend(std::iter::repeat(k as i32).take(n_atoms));
            new_coords.extend(self.frac_coords.iter().map(|c| {
                [c[0], c[1], c[2] + dz]
            }));
        }

        self.species = new_species;
        self.frac_coords = new_coords;
        self.tags = new_tags;
        self.labels = new_labels;
        self
    }

    /// Add a block of atoms in one shot.
    ///
    /// Useful for adsorbates, dopants, etc. All arrays must have the same length.
    pub fn with_atoms(
        mut self,
        species: Vec<ElementSymbol>,
        frac_coords: Vec<[f64; 3]>,
        tags: Vec<i32>,
        labels: Vec<Option<String>>,
    ) -> Self {
        let n = species.len();
        assert!(frac_coords.len() == n, "frac_coords length mismatch");
        assert!(tags.len() == n, "tags length mismatch");
        assert!(labels.len() == n, "labels length mismatch");

        self.species.extend(species);
        self.frac_coords.extend(frac_coords);
        self.tags.extend(tags);
        self.labels.extend(labels);
        self
    }

    /// Add vacuum gap along the c-axis (out-of-plane direction).
    ///
    /// Forces any pending transform first. Stretches the cell by `gap_ang`
    /// Angstrom along c and scales fractional coordinates accordingly.
    pub fn add_vacuum_gap(mut self, gap_ang: f64) -> Self {
        self = self.apply();
        if let Some(cell) = &self.cell {
            let m = crate::slab::vacuum_gap_matrix(cell, gap_ang);
            let m33 = m.fixed_view::<3, 3>(0, 0).into_owned();
            let m33_inv = m33.try_inverse().expect("Vacuum matrix is singular");
            let new_tensor = cell.tensor() * m33_inv;
            self.cell = Some(LatticeVectors::new(new_tensor));
            for coord in &mut self.frac_coords {
                let old = nalgebra::Point3::new(coord[0], coord[1], coord[2]);
                let new = m33 * old;
                *coord = [new.x, new.y, new.z];
            }
        }
        self
    }

    /// Ensure a cell exists, return error otherwise.
    pub fn require_cell(&self) -> Result<&LatticeVectors, Error> {
        self.cell.as_ref().ok_or(Error::NoCell)
    }
}

#[cfg(test)]
mod tests {
    use crate::transform::Supercell;

    use super::*;

    fn simple_structure() -> Structure {
        use castep_periodic_table::element::ElementSymbol;
        Structure::new(
            vec![ElementSymbol::H],
            vec![[0.0, 0.0, 0.0]],
            Some(LatticeVectors::new(nalgebra::Matrix3::identity() * 10.0)),
            [true, true, true],
            vec![0],
            vec![None],
            None,
        )
    }

    #[test]
    fn num_atoms() {
        let s = simple_structure();
        assert_eq!(s.num_atoms(), 1);
    }

    #[test]
    fn supercell_replicates_coords() {
        let s = simple_structure()
            .transform(Supercell::new(2, 2, 1))
            .apply();
        // Coord should halve: [0,0,0] / 2 = [0,0,0] in frac
        assert_eq!(s.frac_coords[0], [0.0, 0.0, 0.0]);
        // Cell should double in a and b
        let (a, b, _) = s.cell.unwrap().lengths();
        assert!((a - 20.0).abs() < 1e-10);
        assert!((b - 20.0).abs() < 1e-10);
    }

    #[test]
    fn replicate_along_c() {
        let s = simple_structure()
            .replicate_along_c(4);
        assert_eq!(s.num_atoms(), 4);
        assert_eq!(s.tags, vec![0, 1, 2, 3]);
        assert!((s.frac_coords[3][2] - 0.75).abs() < 1e-10);
    }

    #[test]
    fn with_atoms_one_shot() {
        let s = simple_structure()
            .with_atoms(
                vec![ElementSymbol::O],
                vec![[0.5, 0.5, 0.5]],
                vec![-1],
                vec![Some("ads".into())],
            );
        assert_eq!(s.num_atoms(), 2);
    }

    #[test]
    fn wrap_frac_coords() {
        let s = Structure::new(
            vec![ElementSymbol::H],
            vec![[-0.1, 1.2, 2.5]],
            Some(LatticeVectors::new(nalgebra::Matrix3::identity() * 10.0)),
            [true, true, true],
            vec![0],
            vec![None],
            None,
        );
        let s = s.wrap_frac_coords();
        assert!((s.frac_coords[0][0] - 0.9).abs() < 1e-10);
        assert!((s.frac_coords[0][1] - 0.2).abs() < 1e-10);
        assert!((s.frac_coords[0][2] - 0.5).abs() < 1e-10);
    }
}
