use castep_periodic_table::element::ElementSymbol;
use crystallographic_group::database::SpaceGroupHallSymbol;
use nalgebra::Matrix3;

use crate::coords::FracCoord;
use crate::error::Error;
use crate::lattice::LatticeVectors;
use crate::transform::{Transform, TransformMatrix};

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
    pub frac_coords: Vec<FracCoord>,
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

    /// Pending transform (linear + translation) for lazy composition.
    pending: Option<TransformMatrix>,
}

impl Structure {
    pub fn new(
        species: Vec<ElementSymbol>,
        frac_coords: Vec<FracCoord>,
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
                    let p = cell.tensor() * f.0;
                    [p.x, p.y, p.z]
                })
                .collect(),
        )
    }

    /// Compose a transform into the pending pipeline.
    ///
    /// Does NOT touch coordinates or cell yet. Call `.apply()` when ready.
    pub fn transform(mut self, t: impl Transform) -> Self {
        let tm = t.matrix();
        self.pending = match self.pending.take() {
            None => Some(tm),
            Some(existing) => Some(tm.compose(existing)),
        };
        self
    }

    /// Force-apply the pending transform to cell + all frac_coords.
    ///
    /// This is a no-op if no transform is pending.
    pub fn apply(mut self) -> Self {
        let tm = match self.pending.take() {
            None => return self,
            Some(tm) => tm,
        };

        let linear_inv = tm.linear
            .try_inverse()
            .expect("Transform matrix is singular");

        // Update cell: C' = C * L⁻¹
        if let Some(cell) = &mut self.cell {
            *cell = LatticeVectors::new(cell.tensor() * linear_inv);
        }

        // Apply to coords: x' = L * x + t
        for coord in &mut self.frac_coords {
            *coord = FracCoord(tm.linear * coord.0 + tm.translation);
        }

        self
    }

    /// Wrap fractional coordinates to [0, 1) by subtracting integer parts.
    /// Forces any pending transform first.
    pub fn wrap_frac_coords(mut self) -> Self {
        self = self.apply();
        for coord in &mut self.frac_coords {
            coord.wrap();
        }
        self
    }

    /// Create an (nx, ny, nz) supercell: scale the cell and replicate all atoms.
    ///
    /// Unlike the [`Supercell`](crate::transform::Supercell) transform (which only
    /// rescales the cell and fractional coordinates), this method actually creates
    /// nx×ny×nz copies of every atom at the appropriate fractional offsets, so the
    /// in-plane atom density is preserved.
    ///
    /// Coordinates are wrapped to [0, 1) after replication.
    pub fn supercell(mut self, nx: usize, ny: usize, nz: usize) -> Self {
        assert!(nx >= 1 && ny >= 1 && nz >= 1, "supercell factors must be ≥ 1");
        self = self.apply();

        // Scale cell
        if let Some(cell) = &mut self.cell {
            let t = cell.tensor().clone();
            let scaled = nalgebra::Matrix3::from_columns(&[
                t.column(0) * nx as f64,
                t.column(1) * ny as f64,
                t.column(2) * nz as f64,
            ]);
            *cell = LatticeVectors::new(scaled);
        }

        let n = self.num_atoms();
        let total = n * nx * ny * nz;
        let mut new_species = Vec::with_capacity(total);
        let mut new_coords = Vec::with_capacity(total);
        let mut new_tags = Vec::with_capacity(total);
        let mut new_labels = Vec::with_capacity(total);

        // Rescale the base coordinates into the enlarged cell, then add the
        // translated copies. The base copy lands at f/n + 0; copy (i,j,k)
        // lands at f/n + i/nx + j/ny + k/nz. This matches the old
        // Supercell transform (which rescaled coords and the cell) plus the
        // missing atom replication.
        for i in 0..n {
            let f = self.frac_coords[i];
            let rx = f.x / nx as f64;
            let ry = f.y / ny as f64;
            let rz = f.z / nz as f64;
            for ix in 0..nx {
                for iy in 0..ny {
                    for iz in 0..nz {
                        let dx = ix as f64 / nx as f64;
                        let dy = iy as f64 / ny as f64;
                        let dz = iz as f64 / nz as f64;
                        new_species.push(self.species[i]);
                        new_coords.push(FracCoord::new(
                            rx + dx, ry + dy, rz + dz,
                        ));
                        new_tags.push(self.tags[i]);
                        new_labels.push(self.labels[i].clone());
                    }
                }
            }
        }

        self.species = new_species;
        self.frac_coords = new_coords;
        self.tags = new_tags;
        self.labels = new_labels;

        // Wrap to [0, 1)
        for c in &mut self.frac_coords {
            c.wrap();
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
                FracCoord::new(c.x, c.y, c.z + dz)
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
        frac_coords: Vec<FracCoord>,
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
            let tm = crate::slab::vacuum_gap_matrix(cell, gap_ang);
            let linear_inv = tm.linear.try_inverse().expect("Vacuum matrix is singular");
            let new_tensor = cell.tensor() * linear_inv;
            self.cell = Some(LatticeVectors::new(new_tensor));
            for coord in &mut self.frac_coords {
                *coord = FracCoord(tm.linear * coord.0);
            }
        }
        self
    }

    /// Align lattice vectors to Cartesian axes.
    ///
    /// After transforms like SurfaceRotation, lattice vectors may not be
    /// axis-aligned. This rotates the entire system so that:
    /// - c points along the z-axis (surface normal)
    /// - a lies in the xy-plane
    /// - b lies in the xy-plane, perpendicular to a
    ///
    /// This is a pure rotation in Cartesian space — fractional coordinates
    /// and interatomic distances are preserved.
    pub fn align_axes(mut self) -> Self {
        self = self.apply();
        if let Some(cell) = &mut self.cell {
            let tensor = cell.tensor();
            let a = tensor.column(0);
            let _b = tensor.column(1);
            let c = tensor.column(2);

            // Gram-Schmidt orthonormalization of the lattice vectors
            let c_hat = c.normalize();
            let a_perp = a - a.dot(&c_hat) * c_hat;
            let a_len = a_perp.norm();
            if a_len < 1e-14 {
                return self; // already axis-aligned or 1D degenerate
            }
            let a_hat = a_perp.normalize();
            let b_hat = c_hat.cross(&a_hat);

            // Rotation matrix: rows are the new axis-aligned basis.
            // Applying R rotates cell vectors to align with Cartesian axes.
            let r = Matrix3::from_rows(&[a_hat.transpose(), b_hat.transpose(), c_hat.transpose()]);
            *cell = LatticeVectors::new(r * tensor);
            // Fractional coords unchanged: rotation applies uniformly to
            // cell and atom positions, so f' = (R*C)⁻¹ * R*C * f = f.
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
            vec![FracCoord::new(0.0, 0.0, 0.0)],
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
        assert_eq!(s.frac_coords[0], FracCoord::new(0.0, 0.0, 0.0));
        // Cell should double in a and b
        let (a, b, _) = s.cell.unwrap().lengths();
        assert!((a - 20.0).abs() < 1e-10);
        assert!((b - 20.0).abs() < 1e-10);
    }

    #[test]
    fn supercell_method_replicates_atoms() {
        let s = simple_structure()
            .clone()
            .supercell(2, 2, 1);
        // One atom becomes four: (0,0), (0.5,0), (0,0.5), (0.5,0.5).
        assert_eq!(s.num_atoms(), 4);
        let (a, b, c) = s.cell.unwrap().lengths();
        assert!((a - 20.0).abs() < 1e-10);
        assert!((b - 20.0).abs() < 1e-10);
        assert!((c - 10.0).abs() < 1e-10, "nz=1 keeps c");
        let mut xs: Vec<(f64, f64)> = s
            .frac_coords
            .iter()
            .map(|f| (f.x, f.y))
            .collect();
        xs.sort_by(|p, q| p.0.total_cmp(&q.0).then(p.1.total_cmp(&q.1)));
        let expect: Vec<(f64, f64)> =
            vec![(0.0, 0.0), (0.0, 0.5), (0.5, 0.0), (0.5, 0.5)]
                .into_iter()
                .map(|(x, y)| (x, y))
                .collect();
        assert_eq!(xs, expect);
    }

    #[test]
    fn supercell_method_preserves_physical_positions() {
        // An atom at frac 0.25 in a 10 A cell must stay at 2.5 A from the
        // corner after a 2x2x1 supercell: base copy at 0.25/2 = 0.125 of the
        // 20 A cell, copy at 0.125 + 0.5.
        let s = Structure::new(
            vec![ElementSymbol::H],
            vec![FracCoord::new(0.25, 0.25, 0.5)],
            Some(LatticeVectors::new(nalgebra::Matrix3::identity() * 10.0)),
            [true, true, true],
            vec![0],
            vec![None],
            None,
        )
        .supercell(2, 2, 1);
        assert_eq!(s.num_atoms(), 4);
        let c = s.cell.unwrap();
        let mut xs: Vec<f64> = s
            .frac_coords
            .iter()
            .map(|f| f.x * c.lengths().0)
            .collect();
        xs.sort_by(|a, b| a.total_cmp(b));
        // 2.5, 12.5, 2.5, 12.5
        assert_eq!(xs, vec![2.5, 2.5, 12.5, 12.5]);
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
                vec![FracCoord::new(0.5, 0.5, 0.5)],
                vec![-1],
                vec![Some("ads".into())],
            );
        assert_eq!(s.num_atoms(), 2);
    }

    #[test]
    fn wrap_frac_coords() {
        let s = Structure::new(
            vec![ElementSymbol::H],
            vec![FracCoord::new(-0.1, 1.2, 2.5)],
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
