# chemrust — Design Principles & Coding Style

> These conventions apply when working with `chemrust-core` and related crates.
> They were established during the 2026-05-06 rewrite discussion.

## Crate Identity

`chemrust-core` is a **crystal geometry toolkit**, not a universal intermediate representation (IR).
- Format-specific data (spin, species_pot, k-points) lives in format-specific types (e.g., `castep-cell-io`'s `CellDocument`).
- Core provides geometry operations + a convenience `Structure` data bag.
- Format conversion is **downstream code**: the application/example wires core types to output formats inline.

## Type Design

- **Concrete types, no traits unless they earn their keep.** The one exception is the `Transform` trait, which unifies geometric operations under one composable abstraction enabling lazy matrix composition.
- **Struct-of-arrays (SoA) layout.** All per-atom data lives in separate `Vec`s (species, frac_coords, tags, labels). Coordinate transforms operate on one array in one pass.
- **Single `Structure` struct** for molecules, crystals, and slabs — like ASE's `Atoms` model:
  - Molecule: `cell=None, pbc=[F,F,F]`
  - Crystal: `cell=Some, pbc=[T,T,T]`
  - Slab: `cell=Some, pbc=[T,T,F]`
- **Always fractional coordinates internally.** Cartesian positions computed on-the-fly via `cell * frac_coords`.
- **Semantic newtypes** where they prevent type confusion: `FracCoord`, `MillerIndices`, `LayerIndex`.

## API Style

- **Chainable API.** All methods consume `self` and return `Self`. Enables builder-style pipelines.
- **Lazy matrix composition.** `transform()` queues a 4×4 augmented matrix. `apply()` forces the composition and applies it to cell + coords.
- **Iterator chains** over for-loops for atomic operations.
- **Builder pattern** (via `bon` crate) for structs with many optional fields.
- **Chained operators** over nested function calls.

## Architectural Boundaries

- `chemrust-core` has **zero format dependencies** — no `castep-cell-io`, no CIF, no XYZ.
- I/O crates (`castep-cell-io`, `chemrust-kpoint-gen`, etc.) live alongside core in the workspace.
- Format conversion examples live in `examples/` as `[dev-dependencies]`.

## What Belongs in Core vs External Crates

| In core | External crate |
|---------|---------------|
| `Structure`, `LatticeVectors`, `CellConstants` | `CellDocument` (castep-cell-io) |
| `Transform` trait, `SurfaceRotation`, `Supercell`, `VacuumGap` | `ParamDocument` (castep-cell-io) |
| `fcc_bulk()`, slab generation | Symmetry operators (crystallographic-group) |
| `FracCoord`, `MillerIndices`, `LayerIndex` | MP grid + k-point generation (chemrust-kpoint-gen) |
