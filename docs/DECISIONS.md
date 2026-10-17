# Decision Log

Session decisions affecting the repo. One entry per decision, newest first.

## 2026-10-18: K-point settings layer + castep-kpoint-gen app crate

- `chemrust-kpoint-gen` now exports a curated 8-item public API at the
  crate root: `KPoint`, `IrreducibleKpt`, `MPGrid`, `MPSpacingParam`,
  `SymmetryOperation`, `reduce_kpoints`, plus the new settings layer
  `KPointSampling` and `KPointSet`. Modules stay private.
- `apply_rotation_to_kpt` is `pub(crate)` (used by kpoint tests across
  modules); `is_irreducible` is private. `#![allow(unused)]` in
  monkhorst_pack and the unused `Spacing` alias are removed.
- D1: the code-agnostic settings layer lives in kpoint-gen, not in
  castep-cell-io and not in a new core crate.
- D2: the CASTEP app-adapter is a new standalone crate
  `castep-kpoint-gen` at `~/programming/castep-kpoint-gen`, sibling of
  `castep-cell-io`. Name follows the `castep-*` family.
- D3: kpoint-gen takes no dependency on castep-cell-io. This reverses
  the draft plan that would have added a `castep` module there.
- D4: KPOINT_LIST weights are normalized to sum 1 in the adapter.
- D5: symmetry comes from a CASTEP space-group number (programmatic)
  or a `SYMMETRY_OPS` block. castep-cell-io has no space-group-number
  type, so the number path is programmatic.
- Conventions recorded in `docs/KPT-SETTINGS-LAYER.md`: 2pi spacing
  scaling for `KPOINTS_MP_SPACING` (ReciprocalCellVectors is 2pi,
  CASTEP is crystallographic 1/ang), MP offset as BZ fractions with
  `offset_i * 2 * n_i` conversion (unverified assumption),
  `hall_symbol` off-by-one (zero-indexed table vs 1-indexed CASTEP
  numbers), `core_position_set` as full group element list, rotation
  only (non-symmorphic translations ignored), Ang-only lattice units.
- `castep-kpoint-gen` deps: path deps on `castep-cell-io/castep_cell_io`,
  `chemrust/chemrust-kpoint-gen`, `chemrust/chemrust-geometry`;
  `castep-cell-fmt` is dev-only.
- Affected: `chemrust-kpoint-gen/src/{lib.rs, settings.rs,
  functions/mod.rs, monkhorst_pack/mod.rs, monkhorst_pack/mp_spacing.rs}`,
  new `docs/KPT-SETTINGS-LAYER.md`, new crate
  `~/programming/castep-kpoint-gen`, `docs/DECISIONS.md`.
- Verified: chemrust workspace 24 tests (16 geometry + 8 kpoint-gen),
  0 warnings. `castep-kpoint-gen` 17 tests including a KPOINT_LIST
  write-out and re-parse round trip. `cargo doc` for kpoint-gen lists
  exactly the 8 public items.
- Open: `castep-kpoint-gen` has no git repo yet. KPOINT_MP_OFFSET
  unit assumption unverified against CASTEP.

## 2026-10-07: Prune the seven stale non-member crate dirs

- Removed `cell_reader`, `chemrust-cerius2-io`, `chemrust-misctools`,
  `chemrust-parser`, `chemrust-scanner`, `chemrust-settings`, `mount_scanner`.
- None of the seven is a workspace member. The workspace keeps only
  `chemrust-geometry` and `chemrust-kpoint-gen`.
- All seven were fully git-tracked with no uncommitted files. Nothing is
  lost. History stays reachable via git.
- CI runs `cargo build` and `cargo test` at the root. It never touched the
  seven dirs, so the workflow needs no change.
- `docs/REWRITE_PLAN.md` still names them. That document is historical
  context and stays as is.
- Supersession confirmed: `mount_scanner` and `chemrust-scanner` were
  superseded by the sibling repo `../chemrust-nasl`.
  - `chemrust-scanner` -> `chemrust-nasl` (core). Same geometry
    primitives/intersections, refactored coordination-sites module,
    `circle_check`/`sphere_check` algorithms. Same `SAC_GDY_V.cell` fixture.
  - `mount_scanner` -> `chemrust-nasl-app` (binary `rhino`). `arg_parser.rs`
    and `kpoint_quality.rs` are byte-identical. Same `example_task.yaml`
    schema. Seeding (pseudopotential copy) carries the old Fast/Full/Post
    run modes.
  - Old dirs last touched 2024-05-27. The nasl repo ran from 2024-04-29 to
    2025-04-17 and kept evolving. The prune is safe.
- `notes/` (kept `pr-reviews/` content per the 2026-10-01 docs move) is not
  a crate dir and was not pruned. Open: prune it too or keep it.
- Root `README.md` still lists `chemrust-core` under Member. Stale label.
  Open: update it to the current member names.
- Verified: `cargo build` and `cargo test --workspace` pass.
  20 tests pass (16 geometry, 4 kpoint-gen).
- Affected: 7 deleted dirs, `docs/DECISIONS.md`.


## 2026-10-07: chemrust-geometry manifest metadata filled; MIT confirmed

- `chemrust-geometry/Cargo.toml` now sets `homepage.workspace = true` and
  `repository.workspace = true` (inherit from the root).
- `documentation = "https://docs.rs/chemrust-geometry"` is set on the member.
  The workspace has no `documentation` key to inherit.
- License stays MIT via `license.workspace = true`. No change needed.
- Verified: `cargo metadata` shows `license: MIT`, `homepage`, `repository`,
  and `documentation` all resolved. `cargo publish --dry-run` no longer prints
  the metadata warning.
- Open question (undecided): should `chemrust-geometry` leave the current
  workspace-style repo and get its own repo?
- Recommendation: keep it in the workspace for now. `chemrust-kpoint-gen`
  path-depends on `chemrust-geometry` and uses its public API. A repo split
  would force kpoint-gen to track published versions of geometry mid-development.
  `cargo publish` works from inside a workspace, so publishing is not blocked.
- Affected: `chemrust-geometry/Cargo.toml`, `docs/DECISIONS.md`.


## 2026-10-07: chemrust-geometry publish-ready; dev-deps move off local paths

- `chemrust-geometry` is ready for `cargo publish`.
- Dev-deps `castep-cell-io` 0.7.0 and `castep-cell-fmt` 0.3.0 drop their
  `path = "../../castep-cell-io/..."` keys. They now resolve from crates.io.
- The local-path pin (2026-10-01) existed because registry 0.2.1 lacked
  `ToCellFile`. `castep-cell-fmt` 0.3.0 is now published on crates.io, so
  the registry version carries `ToCellFile` and the example builds.
- Added `chemrust-geometry/README.md`. It documents the core types, the
  transform pipeline, the slab generators, and the CASTEP example.
- `cargo publish` refuses path dependencies, so this is the blocker fix.
- Affected: `chemrust-geometry/Cargo.toml`, `chemrust-geometry/README.md`.
- Verified: `cargo publish --dry-run --allow-dirty` packages 13 files and
  verifies the build. `cargo build --example cu111_co` compiles from the
  registry versions. `cargo test -p chemrust-geometry` passes 16 tests.
- Known warning: `manifest has no documentation, homepage or repository`.
  The manifest inherits `edition`, `authors`, `license` from the workspace
  but not `repository` or `homepage`.


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
