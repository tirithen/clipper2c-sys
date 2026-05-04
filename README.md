# clipper2c-sys

[![crate.io](https://img.shields.io/crates/v/clipper2c-sys.svg)](https://crates.io/crates/clipper2c-sys)
[![docs.rs](https://docs.rs/clipper2c-sys/badge.svg)](https://docs.rs/clipper2c-sys)

Rust FFI bindings to [Clipper2](https://github.com/AngusJohnson/Clipper2),
Angus Johnson's C++ library for 2D polygon clipping, offsetting,
Minkowski sums, and polyline simplification.

> **Looking for a safe, idiomatic API?** Use the higher-level
> [`clipper2`](https://crates.io/crates/clipper2) crate. This `-sys` crate
> exposes the raw, unsafe C ABI and is intended for crates that wrap or
> extend it.

This crate is a thin Rust FFI wrapper around Clipper2, the canonical C++
implementation. Clipper2 is vendored verbatim into this repository so
builds stay reproducible without depending on a system install or
network access. The C ABI bridge started as a fork of
[songhuaixu/clipper2c](https://github.com/songhuaixu/clipper2c) (Boost
Software License 1.0, preserved in `clipper2c/LICENSE`) and has since
been substantially rewritten — most notably for zero-copy.

## What's exposed

- Boolean operations on polygons: intersection, union, difference, XOR
- Polygon offsetting / inflation / deflation, with square, bevel, round, or miter joins and butt, square, round, or joined ends
- Minkowski sum and difference
- Rectangular clipping for paths and polylines
- Polyline simplification: Ramer–Douglas–Peucker, near-equal stripping, collinear trimming
- Point-in-polygon, polygon area, axis-aligned bounds
- Hierarchical (PolyTree) results that preserve solid/hole nesting
- Integer (`_64`, `i64`) and floating-point (`_D`, `f64`) coordinate variants — see the [crate-level docs](https://docs.rs/clipper2c-sys) for the precision/range tradeoff

## Typical use cases

- **CAD / CNC / 3D-printing slicers** — toolpath offsetting, pocketing, infill generation, contour boolean operations
- **GIS / mapping** — polygon overlay, buffer zones, vector tile clipping
- **Vector graphics & rendering** — path stroking via offset, SVG-style clipping, tessellation pre-pass
- **Game development** — visibility polygons, navigation mesh boolean operations, collision-region merging
- **Robotics / motion planning** — Minkowski-sum configuration spaces, swept-area computation
- **Computational geometry research** — exact-arithmetic boolean operations on integer-grid polygons

## Versioning

The crate's SemVer tracks the Rust FFI surface. The bundled Clipper2
version is recorded directly in upstream's [`clipper.version.h`](clipper2c/vendor/Clipper2/CPP/Clipper2Lib/include/clipper2/clipper.version.h)
— that file is the single source of truth for which Clipper2 release
this crate is built against.

```toml
clipper2c-sys = "0.1"
```

## Building

The crate compiles the vendored Clipper2 C++ source through
[`cc`](https://crates.io/crates/cc) and therefore needs a working
**C++17** toolchain on the build host:

| Platform | Toolchain |
|----------|-----------|
| Linux    | `g++ ≥ 7` or `clang++ ≥ 5` (most distros' default) |
| macOS    | Xcode Command Line Tools (`xcode-select --install`) |
| Windows  | MSVC build tools (Visual Studio Build Tools 2019+) or MSYS2 with `g++` |

Minimum Rust: **1.85** (edition 2024).

Optional features:

- `serde` — derives `Serialize` / `Deserialize` on `ClipperPoint64`.
- `generate-bindings` — regenerates the FFI bindings via `bindgen` at build time (otherwise the pre-generated `generated/bindings.rs` is used). Requires `libclang` on the host.

WebAssembly: `wasm32-unknown-unknown` builds via the WASI SDK toolchain — see `scripts/wasm-check.sh`.

## Status

This crate is pre-1.0; expect breaking changes between minor versions.

Suggestions on how the API can be simplified, or direct code
contributions, are welcome — see
[CONTRIBUTING.md](https://github.com/tirithen/clipper2c-sys/blob/main/CONTRIBUTING.md)
for more details.

## License

Licensed under either of [Apache License, Version 2.0](https://github.com/tirithen/clipper2c-sys/blob/main/LICENSE-APACHE.md)
or [MIT license](https://github.com/tirithen/clipper2c-sys/blob/main/LICENSE-MIT.md)
at your option.

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in clipper2c-sys by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
