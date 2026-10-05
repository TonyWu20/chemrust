# Decision Log

Session decisions affecting the repo. One entry per decision, newest first.

## 2026-10-05: `crystallographic-group` 0.3.1 to 0.4.0

- `chemrust-geometry` and `chemrust-kpoint-gen` now use `crystallographic-group = "0.4.0"`.
- Geometry needed no code change. `database::SpaceGroupHallSymbol` and `database::CrystalSystem` keep their 0.3.1 paths.
- Kpoint-gen needed one test fix. `LookUpSpaceGroup` and `DEFAULT_SPACE_GROUP_SYMBOLS` are gone. The lookup is now `SpaceGroupTable::per_number().hall_symbol(20)`.
- `SeitzMatrix::rotation_part()` still returns `Matrix3<i32>`. The `SymmetryOperation` impl compiles unchanged.
- The lockfile now holds a single 0.4.0.
- Rationale: 0.4.0 is the current release. It adds `SpaceGroupNumber`, `SpaceGroupTable`, and the `SpaceGroup` type.
- Affected: `chemrust-geometry/Cargo.toml`, `chemrust-kpoint-gen/Cargo.toml`, `chemrust-kpoint-gen/src/lib.rs`, `docs/REWRITE_PLAN.md`, `Cargo.lock`.
- Verified: `cargo test --workspace` (16 geometry + 4 kpoint-gen pass). `cargo run --example cu111_co` writes 66 atoms.

## 2026-10-01: castep dev-deps point at the local sibling repo

- `chemrust-geometry` dev-deps now use the local path deps `castep-cell-io` 0.7.0
  and `castep-cell-fmt` 0.3.0 from `../castep-cell-io/`.
- Both crates resolve to the same source tree that `castep-cell-io` itself uses.
- Rationale: the example imports `castep_cell_fmt::ToCellFile`, and `CellDocument`
  (from `castep-cell-io`) implements it.
- The registry pin `castep-cell-fmt = "0.2.1"` created a second crate in the
  dependency graph. `to_cell_file()` did not resolve, so the example failed to build.
  One local 0.3.0 crate fixes it.
- Affected: `chemrust-geometry/Cargo.toml`.
- Verified: `cargo run --example cu111_co` writes `Cu111_CO.cell` and `.param`.
  The structure has 66 atoms: 64 Cu, 1 C, 1 O.
  The cell is axis-aligned: `a` on x, `b` in the xy plane, `c` on z.
  c is 18.261 A, that is 4 layers plus a 12 A vacuum. PBC is `[T,T,F]`.
  Workspace tests: 20 pass.

## 2026-10-01: `FracCoord` drops `DerefMut`, keeps `Deref`

- `FracCoord` keeps `Deref<Target = Point3<f64>>`. It is the mechanism the plan's
  semantic accessors (`.x`, `.y`, `.z`) and `Index`/`IndexMut` usage depend on.
- Removing it would force `.0.x` / `.0[0]` everywhere. It would break the plan's
  "indexing still works" claim.
- `DerefMut` is removed: unused in the codebase.
- Writes go through the `pub` tuple field `.0` (in `wrap()`) or whole-value
  assignment in `apply()` / `add_vacuum_gap()`.
- Affected: `chemrust-geometry/src/coords.rs`.

## 2026-10-01: docs convention moves from `notes/` to `docs/`

- Context docs and plans live in `docs/` at the repo root.
- `notes/` is no longer the docs convention. It keeps its existing `pr-reviews/`
  content for now.
- `IMPROVEMENT_PLAN.md` and `REWRITE_PLAN.md` move from `chemrust-geometry/` to
  `docs/` via `git mv`. History is preserved.
- Stale `chemrust-core` labels in the moved files and `CLAUDE.md` update to
  `chemrust-geometry`.
- Affected: `docs/IMPROVEMENT_PLAN.md`, `docs/REWRITE_PLAN.md`, `CLAUDE.md`.

## 2026-10-01: example uses semantic accessors

- `cu111_co_system()` now uses `.x` / `.y` / `.z` for coordinate access instead of
  `[0]` / `[1]` / `[2]`, per the plan's cosmetic note.
- Affected: `chemrust-geometry/examples/cu111_co.rs`.
