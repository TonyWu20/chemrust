# Plan: Point3 internal + TransformMatrix refactor

## Context

The `chemrust-core` rewrite is working (all 17 tests pass, clean `align_axes` output), but two ergonomic improvements align it better with nalgebra:

1. **`FracCoord` currently wraps `[f64;3]`** — every matrix operation manually constructs a `Point3` via `Point3::new(coord[0], coord[1], coord[2])`, and results are unwrapped via `FracCoord::new(new.x, new.y, new.z)`. Wrapping `Point3<f64>` directly enables `m33 * coord.0` and eliminates these manual conversions.

2. **`Transform` trait returns raw `Matrix4<f64>`** — code manually extracts `m33` via `fixed_view::<3,3>(0,0)` and translation via `m[(0,3)], m[(1,3)], m[(2,3)]`. A decomposed `TransformMatrix { linear: Matrix3<f64>, translation: Vector3<f64> }` struct eliminates element-level access and simplifies composition.

## Target Files

| File | Nature of change |
|------|-----------------|
| `chemrust-core/src/coords.rs` | `FracCoord([f64;3])` → `FracCoord(Point3<f64>)`, update all methods |
| `chemrust-core/src/transform.rs` | New `TransformMatrix` struct; `Transform` trait returns it instead of `Matrix4`; update `SurfaceRotation`, `Supercell` impls |
| `chemrust-core/src/structure.rs` | `pending: Option<Matrix4<f64>>` → `Option<TransformMatrix>`; simplify `transform()`, `apply()`, `add_vacuum_gap()`; update tests |
| `chemrust-core/src/slab.rs` | `vacuum_gap_matrix()` returns `TransformMatrix`; update `cu111_4layer()`, `cu111_co_system()` call sites |
| `chemrust-core/examples/cu111_co.rs` | `coord.0` becomes `Point3<f64>`, need `[coord.x, coord.y, coord.z]` for `PositionFracEntry` |

## Step 1: `TransformMatrix` struct (transform.rs)

Define before changing the trait, since it's a leaf dependency:

```rust
#[derive(Debug, Clone, Copy)]
pub struct TransformMatrix {
    pub linear: Matrix3<f64>,
    pub translation: Vector3<f64>,
}

impl TransformMatrix {
    pub fn from_linear(linear: Matrix3<f64>) -> Self {
        Self { linear, translation: Vector3::zeros() }
    }

    pub fn compose(self, other: Self) -> Self {
        Self {
            linear: self.linear * other.linear,
            translation: self.linear * other.translation + self.translation,
        }
    }
}
```

Then update the trait:
```rust
pub trait Transform {
    fn matrix(&self) -> TransformMatrix;
}
```

Update `SurfaceRotation` and `Supercell` impls to return `TransformMatrix::from_linear(...)` instead of embedding 3×3 into 4×4.

Update 3 tests in transform.rs — access `.linear` instead of matrix elements.

## Step 2: `FracCoord` wraps `Point3<f64>` (coords.rs)

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FracCoord(pub Point3<f64>);
```

Removals:
- `Deref<Target=[f64;3]>` and `DerefMut` impls — Point3 provides its own `Index<usize>` and `IndexMut<usize>`
- `to_point()` method — trivial, `coord.0` is now `Point3<f64>` directly

Updates:
- `new(x,y,z)` → `Self(Point3::new(x, y, z))`
- `from_array(a)` → `Self(Point3::new(a[0], a[1], a[2]))`
- `into_array()` → `[self.0.x, self.0.y, self.0.z]`
- `wrap()` → use `.x`, `.y`, `.z` field access instead of `[0]`, `[1]`, `[2]`
- `From<[f64;3]>` → construct via Point3
- `Into<[f64;3]>` → same as into_array

Point3<f64> provides:
- `coord[0]`, `coord[1]`, `coord[2]` via `Index`/`IndexMut` — all existing indexing still works
- `coord.x`, `coord.y`, `coord.z` — semantic field access (use in new code)
- `m33 * coord.0` → `Point3<f64>` — direct matrix-point multiplication (the key win)

## Step 3: Simplify `structure.rs`

**`pending` field:**
```rust
pending: Option<TransformMatrix>,
```

**`transform()`** — Matrix4 multiplication → `TransformMatrix::compose()`:
```rust
pub fn transform(mut self, t: impl Transform) -> Self {
    let tm = t.matrix();
    self.pending = match self.pending.take() {
        None => Some(tm),
        Some(existing) => Some(tm.compose(existing)),
    };
    self
}
```

**`apply()`** — eliminate `fixed_view` extraction and manual `Point3::new`:
```rust
pub fn apply(mut self) -> Self {
    let tm = match self.pending.take() {
        None => return self,
        Some(tm) => tm,
    };
    let linear_inv = tm.linear.try_inverse().expect("Transform matrix is singular");
    if let Some(cell) = &mut self.cell {
        *cell = LatticeVectors::new(cell.tensor() * linear_inv);
    }
    for coord in &mut self.frac_coords {
        *coord = FracCoord(tm.linear * coord.0 + tm.translation);
    }
    self
}
```

Key win: `tm.linear * coord.0 + tm.translation` replaces `Point3::new(coord[0],coord[1],coord[2]) → m33 * old + t_vec → FracCoord::new(new.x, new.y, new.z)`.

**`cart_coords()`** — `f[0]` → `f.x` (or `f.0.x`):
```rust
.map(|&f| {
    let p = cell.tensor() * f.0;
    [p.x, p.y, p.z]
})
```

**`replicate_along_c()`** — `c[2]` → `c.z`:
```rust
FracCoord::new(c.x, c.y, c.z + dz)
```

**`add_vacuum_gap()`** — `vacuum_gap_matrix` returns `TransformMatrix`; eliminate `fixed_view`:
```rust
let tm = crate::slab::vacuum_gap_matrix(cell, gap_ang);
// ... same pattern as apply but from TransformMatrix directly
```

**`align_axes()`** — no change needed (uses Matrix3 directly, not Transform).

**Tests** — `FracCoord::new(x,y,z)` construction unchanged. Comparisons like `assert_eq!(s.frac_coords[0], FracCoord::new(0.0, 0.0, 0.0))` still work via `PartialEq`. Indexing `s.frac_coords[3][2]` still works via Point3's `Index`.

## Step 4: Simplify `slab.rs`

- `vacuum_gap_matrix()` returns `TransformMatrix` instead of `Matrix4<f64>`
- All `FracCoord::new(x,y,z)` calls unchanged
- `surf.frac_coords[i][2]` → `surf.frac_coords[i].z` (cosmetic, use semantic accessor)
- `cu111_co_system()`: `a[0]` → `a.x` for in-plane distance computation
- `with_atoms()` call sites: coordinates passed as `FracCoord::new(...)` — unchanged

## Step 5: Update example

`coord: coord.0` becomes `coord: [coord.x, coord.y, coord.z]` since `coord.0` is now `Point3<f64>` not `[f64;3]` when `PositionFracEntry.coord` expects `[f64; 3]`.

## Verification

1. `cargo check -p chemrust-core` — compiles
2. `cargo test -p chemrust-core` — all 13 pass
3. `cargo check -p chemrust-core --example cu111_co` — example compiles
4. `cargo run --example cu111_co` — produces valid, axis-aligned `Cu111_CO.cell`/`.param`
5. `cargo test --workspace` — all 17 pass, no regressions
