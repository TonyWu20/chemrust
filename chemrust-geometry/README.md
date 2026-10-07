# chemrust-geometry

Crystal geometry toolkit for computational materials science.

One `Structure` type covers molecules, crystals, and slabs.
All atomic positions are fractional. Cartesian positions are computed
on demand. The crate has zero format dependencies. Format output
lives in downstream crates such as `castep-cell-io`.

## Core types

| Type | Role |
|------|------|
| `Structure` | Struct-of-arrays container: species, fractional coords, cell, PBC flags, tags, labels, space group |
| `LatticeVectors` | 3x3 Cartesian cell. Columns are the a, b, c vectors in Angstrom |
| `CellConstants` | Lattice constants a, b, c and angles in radians |
| `FracCoord` | Fractional coordinate newtype wrapping `nalgebra::Point3<f64>` |
| `ReciprocalCellVectors` | Reciprocal lattice vectors, 2pi convention |
| `Error` | Crate error type. `NoCell` means the structure lacks a cell |

## Coordinate convention

- Molecule: `cell = None`, `pbc = [F, F, F]`
- Crystal: `cell = Some`, `pbc = [T, T, T]`
- Slab: `cell = Some`, `pbc = [T, T, F]`

## Transform pipeline

Cell-independent operations implement the `Transform` trait.
Each call queues a 4x4 augmented matrix. Call `apply()` to push the
pending matrix through the cell and all coordinates.

```rust
use chemrust_geometry::slab::fcc_bulk;
use chemrust_geometry::transform::{SurfaceRotation, Supercell};
use chemrust_geometry::ElementSymbol;

let slab = fcc_bulk(3.615, ElementSymbol::Cu)
    .transform(SurfaceRotation::new(1, 1, 1))
    .transform(Supercell::new(2, 2, 1))
    .apply();
```

`SurfaceRotation(h, k, l)` rotates a cubic conventional cell so the
(hkl) plane normal aligns with c. For (111) the in-plane cell is a
hexagonal 60 degree rhombus.

Cell-dependent operations are methods on `Structure`.

| Method | Effect |
|--------|--------|
| `supercell(nx, ny, nz)` | Scales the cell and replicates every atom |
| `replicate_along_c(n)` | Stacks n layers along c. Tags each layer 0 to n-1 |
| `add_vacuum_gap(angstrom)` | Extends c by a vacuum gap of the given length |
| `align_axes()` | Rotates the cell so c is on z and a, b lie in the xy plane |
| `with_atoms(species, coords, tags, labels)` | Appends atoms. Useful for adsorbates and dopants |
| `wrap_frac_coords()` | Wraps all fractional coordinates to [0, 1) |

## Slab generators

| Function | Output |
|----------|--------|
| `fcc_bulk(a, species)` | FCC conventional cell with 4 atoms |
| `cu111_4layer(a)` | Cu(111) slab: 2x2 surface cell, 4 layers, 12 A vacuum, 64 Cu atoms |

## Example: Cu(111)+CO CASTEP input

The example builds a Cu(111) slab with a CO adsorbate and writes
CASTEP `.cell` and `.param` files.

```sh
cargo run --example cu111_co
```

The example uses the dev-dependencies `castep-cell-io`,
`castep-cell-fmt`, and `anyhow`.

## License

MIT
