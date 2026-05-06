# Plan: chemrust-core Rewrite + Cu(111)+CO CASTEP Input Generator

## Context

The user wants to build CASTEP `.cell`/`.param` input files programmatically from Rust, replacing the proprietary Windows-only Materials Studio GUI. The target system is Cu(111)+CO (4-layer slab, 2x2 surface cell, CO atop, ~12 A vacuum) as specified in `topics/neural-field-hamiltonian/cu111-co-castep-plan.md`.

Three existing Rust projects form the toolchain:
- **castep-cell-io** (v0.5.0, production-ready): Complete CASTEP file I/O (`CellDocument`, `ParamDocument`, builders, serialization)
- **crystallographic-group** (v0.3.1, stable): Space group symmetry operators from Hall symbols
- **chemrust** (incomplete): Intended central computational chemistry toolchain

The chemrust `refactor` branch has good type designs but is incomplete. The `main` branch has dead code and duplicate implementations.

## Design Principles (from discussion)

1. **chemrust-core = crystal geometry toolkit, not a universal IR.** Format-specific data lives in format-specific types. Core provides geometry operations + a convenience `Structure` data bag.
2. **Concrete types, no traits unless they earn their keep.** The `Transform` trait is the one exception — it unifies geometric operations (rotation, supercell, vacuum) under one composable abstraction, enabling lazy matrix composition.
3. **Always fractional coordinates internally.** Natural for periodic systems, independent of cell parameters.
4. **Single `Structure` struct.** Molecules (cell=None + pbc=[F,F,F]), crystals (cell + pbc=[T,T,T]), slabs (cell + pbc=[T,T,F]).
5. **Struct-of-arrays layout.** Separate `Vec`s for species, coords, tags, labels. Coordinate transforms operate on one array in one pass.
6. **Lazy matrix composition.** `transform()` queues a 4x4 augmented matrix internally. `apply()` forces composition into a single matrix and applies it to cell + coords. Multiple transforms compose via matrix multiplication without touching coordinates.
7. **Chainable API.** All methods consume `self` and return `Self`.
8. **Format conversion is downstream code, not in core.** chemrust-core has zero format dependencies.
9. **crystallographic-group for symmetry.** `Structure.space_group: Option<SpaceGroupHallSymbol>`.

## Step 1: Clean Slate

Delete everything in `chemrust-core/src/` and rebuild from scratch.

**Keep as-is (other crates in workspace):**
- `chemrust-kpoint-gen` (refactor branch) — MP grid + symmetry reduction
- `chemrust-cerius2-io` (refactor branch)
- `chemrust-parser`, `chemrust-scanner`, `mount_scanner`, `cell_reader`

**Remove from workspace:**
- `chemrust-formats` — superseded by castep-cell-io
- `chemrust-settings` — duplicate k-point code

## Step 2: New chemrust-core Module Structure

```
chemrust-core/src/
  lib.rs              # crate docs, re-exports
  structure.rs        # Structure struct (SoA layout, chainable API)
  lattice.rs          # LatticeVectors, CellConstants
  transform.rs        # Transform trait, SurfaceRotation, Supercell, VacuumGap
  coords.rs           # FractionalCoord newtype
  slab.rs             # fcc_bulk(), surface_rotation_matrix(), fcc_layers()
  error.rs            # Error type
```

### `lattice.rs`

```rust
use nalgebra::Matrix3;
use crystallographic_group::database::CrystalSystem;

#[derive(Debug, Clone, Copy)]
pub struct CellConstants { pub a: f64, pub b: f64, pub c: f64, pub alpha: f64, pub beta: f64, pub gamma: f64 }

#[derive(Debug, Clone, Copy)]
pub struct LatticeVectors(Matrix3<f64>);

impl LatticeVectors {
    pub fn new(tensor: Matrix3<f64>) -> Self;
    pub fn from_constants(c: CellConstants) -> Self;
    pub fn tensor(&self) -> &Matrix3<f64>;
    pub fn cell_volume(&self) -> f64;
    pub fn metric_tensor(&self) -> Matrix3<f64>;
    pub fn reciprocal(&self) -> Matrix3<f64>;
    pub fn crystal_system(&self) -> CrystalSystem;
    pub fn lengths(&self) -> (f64, f64, f64);
    pub fn angles(&self) -> (f64, f64, f64);
}
```

### `structure.rs`

```rust
use castep_periodic_table::element::ElementSymbol;
use crystallographic_group::database::SpaceGroupHallSymbol;
use nalgebra::Matrix4;

/// Struct-of-arrays. One struct for molecules, crystals, and slabs.
#[derive(Debug, Clone)]
pub struct Structure {
    pub species: Vec<ElementSymbol>,
    pub frac_coords: Vec<[f64; 3]>,       // always fractional
    pub cell: Option<LatticeVectors>,
    pub pbc: [bool; 3],
    pub tags: Vec<i32>,
    pub labels: Vec<Option<String>>,
    pub space_group: Option<SpaceGroupHallSymbol>,
    pending_matrix: Option<Matrix4<f64>>,  // lazy composition
}

impl Structure {
    pub fn num_atoms(&self) -> usize;

    /// Queue a transform. Does NOT touch coordinates.
    /// Composes: pending = T * existing_pending.
    fn transform(mut self, t: impl Transform) -> Self;

    /// Force-apply pending matrix to cell + all frac_coords.
    /// Returns self with pending cleared.
    fn apply(mut self) -> Self;

    /// Wrap frac_coords to [0,1). Forces apply first.
    fn wrap_frac_coords(self) -> Self;

    /// Replicate atoms along c-axis into N layers. Forces apply first.
    /// Tags each layer k with tag=k.
    fn replicate_along_c(self, n_layers: usize) -> Self;

    /// Add a block of atoms in one shot.
    fn with_atoms(self, species: Vec<ElementSymbol>, coords: Vec<[f64;3]>,
                  tags: Vec<i32>, labels: Vec<Option<String>>) -> Self;
}
```

### `transform.rs`

```rust
use nalgebra::Matrix4;

/// A geometric operation expressed as a 4x4 augmented matrix
/// operating on fractional coordinates: x' = M * x.
/// Cell basis transforms as: C' = C * M_inv_3x3
pub trait Transform {
    fn matrix(&self) -> Matrix4<f64>;
}

/// Rotate lattice so (hkl) plane becomes the a-b plane.
pub struct SurfaceRotation { h: i32, k: i32, l: i32 }
impl Transform for SurfaceRotation { /* returns R_inv in 3x3 block */ }

/// Diagonal supercell scaling.
pub struct Supercell { nx: usize, ny: usize, nz: usize }
impl Transform for Supercell { /* returns diag(1/nx, 1/ny, 1/nz) in 3x3 block */ }

/// Stretch c-axis to add vacuum. Only valid after surface rotation.
pub struct VacuumGap { gap_ang: f64 }
impl Transform for VacuumGap { /* returns diag(1, 1, c_old/c_new) in 3x3 block */ }
```

### `slab.rs`

```rust
/// Build FCC bulk (conventional cell, 4 atoms, Fm-3m, a in Angstrom).
pub fn fcc_bulk(a: f64, species: ElementSymbol) -> Structure;
```

No `cleave_slab` free function — the pipeline is `fcc_bulk().transform(SurfaceRotation).transform(Supercell).apply().replicate_along_c(4)`.

### Draft: End-to-End Cu(111)+CO Pipeline

```rust
use chemrust_core::{
    Structure, LatticeVectors,
    transform::{SurfaceRotation, Supercell, VacuumGap, Transform},
    slab::fcc_bulk,
};
use castep_cell_io::{
    CellDocument, ParamDocument, PositionsFrac, PositionFracEntry, Species as CastepSpecies,
    cell::{
        lattice_param::LatticeCart, species::Species,
        bz_sampling_kpoints::KpointsMpGrid, symmetry::SymmetryGenerate,
    },
    param::{
        general::{GeneralParams, Task},
        exchange_correlation::{ExchangeCorrelationParams, XcFunctional, SpinPolarized},
        basis_set::{BasisSetParams, CutoffEnergy},
        electronic::{ElectronicParams, FixOccupancy},
        electronic_minimisation::{ElectronicMinimisationParams, MaxScfCycles},
    },
    units::EnergyUnit,
};
use castep_cell_fmt::{ToCellFile, format::to_string_many_spaced};
use castep_periodic_table::element::ElementSymbol;

fn main() -> anyhow::Result<()> {
    // ── The pipeline: 8 chained calls ───────────────────────────
    let cu111_co = fcc_bulk(3.615, ElementSymbol::Cu)
        .transform(SurfaceRotation::new(1, 1, 1))       // rotate [111] -> c-axis
        .transform(Supercell::new(2, 2, 1))             // 2x2 surface cell
        .apply()                                         // * force: need coords for layer sort
        .replicate_along_c(4)                            // ABCA, 4 layers, tag by layer
        .transform(VacuumGap::new(12.0))                // stretch c by 12 A
        .apply()                                         // * force: need final cell for heights
        .with_atoms(                                     // CO one-shot
            vec![ElementSymbol::C, ElementSymbol::O],
            vec![
                [0.5, 0.5, top_surface_z + 1.9 / c_len],
                [0.5, 0.5, top_surface_z + 3.05 / c_len],
            ],
            vec![-1, -1],
            vec![Some("C".into()), Some("O".into())],
        )
        .wrap_frac_coords();                             // normalize to [0,1)

    // ── CASTEP conversion (downstream, inline) ──────────────────
    let cell = cu111_co.cell.unwrap();
    let tensor = cell.tensor();

    let cell_doc = CellDocument::builder()
        .lattice(LatticeCart {
            unit: None,
            a: tensor.column(0).into(),
            b: tensor.column(1).into(),
            c: tensor.column(2).into(),
        })
        .positions(PositionsFrac {
            positions: cu111_co.species.iter()
                .zip(cu111_co.frac_coords.iter())
                .map(|(&sp, &coord)| PositionFracEntry {
                    species: Species::Symbol(sp.symbol().to_string()),
                    coord, spin: None, mixture: None,
                })
                .collect(),
        })
        .kpoints_mp_grid(KpointsMpGrid([4, 4, 1]))
        .symmetry_generate(SymmetryGenerate)
        .build()?;

    let param_doc = ParamDocument::builder()
        .general(GeneralParams { task: Some(Task::SinglePointEnergy), ..Default::default() })
        .exchange_correlation(ExchangeCorrelationParams {
            xc_functional: Some(XcFunctional::Pbe),
            spin_polarized: Some(SpinPolarized(false)),
            ..Default::default()
        })
        .basis_set(BasisSetParams {
            cutoff_energy: Some(CutoffEnergy { value: 400.0, unit: EnergyUnit::ElectronVolt }),
            ..Default::default()
        })
        .electronic(ElectronicParams { fix_occupancy: Some(FixOccupancy(true)), ..Default::default() })
        .electronic_minimisation(ElectronicMinimisationParams {
            max_scf_cycles: Some(MaxScfCycles(200)), ..Default::default()
        })
        .build();

    let cell_text = to_string_many_spaced(&cell_doc.to_cell_file());
    let param_text = to_string_many_spaced(&param_doc.to_cell_file());
    fs::write("Cu111_CO.cell", &cell_text)?;
    fs::write("Cu111_CO.param", &param_text)?;
    Ok(())
}
```

### Design Notes

1. **The pipeline is 8 core lines.** `fcc_bulk → transform → transform → apply → replicate → transform → apply → with_atoms → wrap`. Everything else is CASTEP boilerplate (inline, downstream).

2. **`apply()` boundaries are explicit.** Two calls, at exactly the points where the downstream operation needs real coordinates (layer replication needs z-sorted atoms; adsorbate needs final c-length for Angstrom→fractional conversion). This is a feature, not a bug — the lazy→eager transition is visible.

3. **Three `Transform` impls.** `SurfaceRotation`, `Supercell`, `VacuumGap`. This is the right number. New transforms (Strain, Shear) would implement the same trait. Trait earns its keep by enabling lazy composition: three `transform()` calls compile into one matrix multiply.

4. **Struct-of-arrays pays off.** `transform()` only touches `frac_coords` + `cell`. `replicate_along_c` duplicates across all arrays. `with_atoms` appends to all arrays. No per-atom struct allocations, no struct field access during transforms.

5. **FCC layer identification lives in `slab.rs`.** The `fcc_layers()` function identifies which atoms belong to which (111) layer in an FCC conventional cell, supporting the `replicate_along_c` step.

## Step 3: Cu(111)+CO Example

Example binary: `chemrust-core/examples/cu111_co.rs`

Runnable via `cargo run --example cu111_co`. Produces `Cu111_CO.cell` and `Cu111_CO.param`.

## Step 4: Workspace Cleanup

1. Merge `origin/refactor` into main (preserves chemrust-kpoint-gen, chemrust-cerius2-io)
2. Rewrite `chemrust-core/` per Step 2
3. Keep `chemrust-kpoint-gen`, `chemrust-cerius2-io`
4. Keep `chemrust-parser`, `chemrust-scanner`, `mount_scanner`, `cell_reader`
5. Remove `chemrust-formats`, `chemrust-settings` from workspace
6. Update workspace `Cargo.toml` members

## Step 5: Dependencies

**chemrust-core/Cargo.toml** (zero format deps):
```toml
[dependencies]
nalgebra = "0.33"
castep-periodic-table = "0.5"
crystallographic-group = "0.3"
```

**Example binary** (`[dev-dependencies]`):
```toml
castep-cell-io = "0.5"
castep-cell-fmt = "0.1"
anyhow = "1"
```

## Verification

1. `cargo build --workspace` succeeds
2. `cargo test --workspace` passes
3. `cargo run --example cu111_co` produces `Cu111_CO.cell` and `Cu111_CO.param`
4. Inspect: 4 layers x 4 Cu/layer = 16 Cu + C + O = 18 atoms, 4x4x1 k-points, PBE/400eV
5. Copy to CASTEP machine, run `castep Cu111_CO` — should converge
