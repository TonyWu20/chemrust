## PR Review: `main` (chemrust-core rewrite) → `main`

**Rating:** Request Changes

**Summary:** The `chemrust-core` rewrite delivers a sound crystal geometry toolkit with clean architecture (SoA, lazy transforms, chainable API) and a working end-to-end Cu(111)+CO pipeline. The core design decisions are validated. Three fixable issues remain: an unused dependency, a dead-code newtype not wired into `Structure`, and one bare `unwrap()` lacking a diagnostic message. No architectural rework is needed.

**Cross-Round Patterns:** None (first review)

**Deferred Improvements:** 11 items → `notes/pr-reviews/chemrust-core-rewrite/deferred.md`

**Axis Scores:**

- Plan & Spec: Partial — All planned types and methods delivered, but `bon = "3"` is unused and `FracCoord` newtype is dead code. Both are [Defect] items.
- Architecture: Pass — Single `Structure`, SoA layout, lazy composition, zero format deps in core, chainable API. All architectural principles upheld.
- Rust Style: Pass — Clean code, good error types, descriptive `expect()` messages where used, iterator chains, colocated tests. One bare `.unwrap()` flagged.
- Test Coverage: Partial — 13 tests on key paths, but gaps in composed transforms, vacuum gap cell-length, `from_constants`, and panic contracts.
