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
//! # Integer (`_64`) vs decimal (`_D`) variants
//!
//! Every clipping type comes in two flavours, e.g. [`ClipperPath64`] and
//! [`ClipperPathD`]. The clipping engine itself is **only integer**: all
//! boolean math runs on `int64_t`. The `_D` variants are a thin scaling
//! wrapper — they multiply decimal input by a power-of-two scale factor
//! sized to the chosen `precision` argument, run the integer engine, then
//! divide back out.
//!
//! Tradeoffs:
//!
//! - **Precision.** `_64` is exact. `_D` quantises to roughly
//!   `10^-precision` (default `precision = 2`, i.e. ~0.01 units). The
//!   maximum supported precision is 8 decimal digits.
//! - **Range.** Integer coordinates must satisfy `|c| ≤ INT64_MAX/4`
//!   (≈ 2.3 × 10¹⁸). For `_D` the same bound applies *after* scaling, so
//!   high-magnitude values at high precision can hit a range error.
//! - **Performance.** `_D` adds per-point multiply-on-input and
//!   divide-on-output. For large inputs prefer `_64` and pre-scale once
//!   if your data has a known integer grid.
//!
//! Reach for `_D` for convenience when your geometry is already
//! floating-point and the quantisation is acceptable. Reach for `_64`
//! whenever your data fits a regular grid or when precision matters.
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
