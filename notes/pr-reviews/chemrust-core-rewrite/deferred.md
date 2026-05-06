## Deferred Improvements: `chemrust-core-rewrite` — 2026-05-06

### Remove blanket `#![allow(dead_code)]` from lib.rs
**Source:** Round 1 review
**Rationale:** The blanket allow masks real dead code: `FracCoord` unused methods, `Error::InvalidLattice` never constructed, `cart_coords()` never called in tests, `from_constants()` and `crystal_system()` not exercised. Targeted `#[allow(dead_code)]` on intentional public-but-unused-in-crate items is preferable.
**Candidate for:** Phase 2 cleanup
**Precondition:** After `FracCoord` is wired into `Structure` (Issue 2 fix), re-evaluate which items are genuinely unused vs. simply not exercised in tests.

### Rename `SurfaceRotation` to `CubicSurfaceRotation`
**Source:** Round 1 review
**Rationale:** The `SurfaceRotation` struct hard-codes the cubic (hkl) surface matrix construction in `cubic_surface_matrix()`. The name implies it works for any crystal system. Either rename to convey cubic-only, or accept a lattice metric and dispatch to the correct rotation strategy.
**Candidate for:** Phase 2
**Precondition:** A non-cubic surface rotation strategy is needed (e.g., hexagonal or tetragonal).

### Review `crystal_system()` Trigonal detection logic
**Source:** Round 1 review
**Rationale:** The branch `(n_90 == 0 && eq(alpha, beta) && eq(beta, gamma)) || n_120 == 0` in the `a==b==c` case is fragile. The `|| n_120 == 0` arm catches any equal-length cell with no exactly-120° angle, including angle combinations that should fall through to Hexagonal.
**Candidate for:** Phase 2
**Precondition:** A user encounters misclassified cells or the function gains test coverage.

### Remove redundant `.apply()` in `cu111_4layer`
**Source:** Round 1 review
**Rationale:** `replicate_along_c(3)` already calls `self.apply()` internally (line 143 of structure.rs). The trailing `.apply()` on slab.rs line 61 hits the `None => return self` early-exit and is a pure no-op. Removing it clarifies the pipeline.
**Candidate for:** Phase 2 cleanup
**Precondition:** None — trivial fix.

### Restrict `Structure` field visibility
**Source:** Round 1 review
**Rationale:** All per-atom fields on `Structure` are `pub`, allowing unchecked mutation that can violate the lazy-transform invariant (coord/cell mismatch after direct field writes). Consider keeping fields read-only (public getters or `pub` for read, private for write).
**Candidate for:** Phase 2
**Precondition:** An external consumer writes to fields directly and causes a latent bug.

### Add `set_pbc()` chainable method to `Structure`
**Source:** Round 1 review
**Rationale:** `cu111_co_system` mutates `sys.pbc = [true, true, false]` as a direct field write, breaking the chainable API pattern. A `fn with_pbc(mut self, pbc: [bool; 3]) -> Self` method would maintain chainability.
**Candidate for:** Phase 2
**Precondition:** None — trivial addition.

### Extract shared matrix-application logic from `apply()` and `add_vacuum_gap()`
**Source:** Round 1 review
**Rationale:** Both methods extract a 3×3 block, invert it, update `cell = C * M⁻¹`, and transform fractional coordinates. The duplication is small (~10 lines) but would benefit from a `fn apply_matrix(&mut self, m: Matrix4<f64>)` private helper.
**Candidate for:** Phase 2 cleanup
**Precondition:** A third consumer of the pattern emerges (e.g., a `Strain` transform).

### Add composed-transform unit test
**Source:** Round 1 review
**Rationale:** `transform(A).transform(B).apply()` is tested only indirectly via `cu111_4layer`. No unit test verifies that `pending` correctly composes `B * A` (rightmost-first ordering).
**Candidate for:** Phase 2
**Precondition:** None — new test file is self-contained.

### Add vacuum-gap cell-length assertion to tests
**Source:** Round 1 review
**Rationale:** `add_vacuum_gap()` and `cu111_4layer()` are tested for atom counts but never assert the cell c-length actually grew by the gap amount.
**Candidate for:** Phase 2
**Precondition:** None — straightforward test addition.

### Add `from_constants()` correctness test
**Source:** Round 1 review
**Rationale:** The `CellConstants → LatticeVectors` conversion (triclinic angle convention) has no unit test.
**Candidate for:** Phase 2
**Precondition:** None — straightforward test addition.

### Add panic/contract tests for edge cases
**Source:** Round 1 review
**Rationale:** `replicate_along_c(0)`, `with_atoms()` with mismatched lengths, and `reciprocal()` on zero-volume cell should have `#[should_panic]` tests to document the contract.
**Candidate for:** Phase 2
**Precondition:** None — low effort, high documentation value.
