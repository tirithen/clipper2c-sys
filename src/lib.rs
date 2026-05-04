//! Raw Rust FFI bindings to the [Clipper2](https://github.com/AngusJohnson/Clipper2)
//! C++ polygon clipping and offsetting library by Angus Johnson — the canonical
//! implementation. The library's C++ source is vendored into this crate; see
//! the repository [README](https://github.com/tirithen/clipper2c-sys#clipper2c-sys)
//! for the rationale and credit details.
//!
//! # You probably want [`clipper2`](https://crates.io/crates/clipper2) instead
//!
//! Almost every item here is `unsafe`, requires manual memory management, and
//! exposes raw C pointer types. The high-level [`clipper2`](https://crates.io/crates/clipper2)
//! crate wraps these with a safe Rust API and is the recommended way to do
//! polygon clipping from Rust. Reach for `clipper2c-sys` directly only when
//! you need to interop with code that already operates on the C ABI.
//!
//! # Memory model
//!
//! All non-trivial types in this crate are *opaque handles*. The C++ side
//! owns the storage; Rust code only sees `*mut ClipperX` pointers and never
//! reads or writes struct fields directly. The lifecycle of every opaque
//! object is:
//!
//! 1. Ask `clipper_X_size()` how many bytes the C++ object needs.
//! 2. Allocate that many bytes via [`clipper_allocate`].
//! 3. Pass the buffer to a constructor like `clipper_X(mem)`. Internally
//!    this is C++ placement-new — no extra allocation occurs.
//! 4. Use the resulting `*mut ClipperX` as a handle to all `clipper_X_*`
//!    operations.
//! 5. Release with `clipper_delete_X(handle)` (runs the C++ destructor and
//!    frees the [`clipper_allocate`] buffer) or `clipper_destruct_X(handle)`
//!    (destructor only — caller frees the buffer themselves through the
//!    matching allocator).
//!
//! Memory from [`clipper_allocate`] must be released through the matching
//! `clipper_delete_X` (or destructed and freed via the same allocator).
//! Mixing with `libc::free` or Rust's allocator is undefined behaviour.
//!
//! # `_64` (`i64`) vs `_D` (`f64`) variants
//!
//! Every clipping type comes in two flavours that look superficially
//! similar but use **different Rust numeric types**:
//!
//! | Suffix | Rust coordinate type | Upstream C++ type | Example |
//! |--------|----------------------|-------------------|---------|
//! | `_64`  | `i64` (signed 64-bit integer) | `int64_t`         | [`ClipperPoint64`] (`x: i64, y: i64`) |
//! | `_D`   | `f64` (64-bit IEEE-754 float) | `double`          | [`ClipperPointD`] (`x: f64, y: f64`)  |
//!
//! ## Why both exist
//!
//! The clipping engine is implemented **exclusively on `int64_t`**.
//! Integer arithmetic and comparisons are exact, so the engine's
//! handling of coincident edges and shared vertices is deterministic.
//! Floating-point rounding around bit-different-but-mathematically-equal
//! values would otherwise produce different topology depending on input
//! order.
//!
//! - **`_64` types are the engine's native interface.** Your `i64`
//!   coordinates pass through unchanged. No transformation, no rounding.
//! - **`_D` types are a convenience wrapper for `f64` data.** On input,
//!   each coordinate is multiplied by a scale factor
//!   `s = 2^⌈log₂(10^precision)⌉` (default `precision = 2` → `s = 128`),
//!   rounded to `i64`, fed to the engine. Outputs are divided by `s` and
//!   returned as `f64`. The round-trip quantises results to a grid of
//!   step `1/s ≈ 10^-precision`.
//!
//! ## Which preserves your data
//!
//! - **`_64` round-trips bit-exactly** (same `i64` values out as in).
//! - **`_D` does not** — output values are rounded to the integer grid
//!   defined by the `precision` setting. The round-trip is *not* the
//!   identity. This is benign for typical CAD or vector-graphics use,
//!   but matters if you depend on invariants like "feeding the same
//!   shape back must give bit-identical coordinates".
//!
//! ## Other tradeoffs
//!
//! - **Precision floor on `_D`.** Quantisation step is `~10^-precision`
//!   (default `~0.01`). Max supported `precision` is 8 decimal digits
//!   (`CLIPPER2_MAX_DEC_PRECISION`).
//! - **Range.** Integer coordinates must satisfy `|c| ≤ INT64_MAX/4`
//!   (≈ 2.3 × 10¹⁸). For `_D`, the same bound applies *after* scaling,
//!   so high-magnitude values at high precision can hit a range error
//!   well before the raw `f64` runs out of bits.
//! - **Performance.** `_D` adds a multiply on every input coordinate
//!   and a divide on every output. For hot-path work, prefer `_64` and
//!   pre-scale once if your data sits on a known integer grid.
//!
//! Reach for `_D` when your geometry is already `f64` and the
//! quantisation is acceptable. Reach for `_64` when your data fits an
//! integer grid or when bit-exact preservation matters.
//!
//! # Cargo features
//!
//! - `serde` (off by default) — derives `Serialize`/`Deserialize` on
//!   [`ClipperPoint64`].
//! - `generate-bindings` — regenerate the Rust FFI bindings from the C
//!   headers at build time. Off by default; the pre-generated
//!   `generated/bindings.rs` is used otherwise.
//! - `update-bindings` — like `generate-bindings`, but writes the result
//!   back into `generated/bindings.rs` so the regen output is committed.
//!   Use `scripts/regenerate-bindings.sh` to invoke this with the right
//!   `LIBCLANG_PATH` handling.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::unreadable_literal)]
#![allow(deref_nullptr)]

#[cfg(test)]
mod test;

#[cfg(all(not(feature = "update-bindings"), feature = "generate-bindings"))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(any(feature = "update-bindings", not(feature = "generate-bindings")))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/generated/bindings.rs"
));
