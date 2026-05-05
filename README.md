# clipper2c-sys

[![crate.io](https://img.shields.io/crates/v/clipper2c-sys.svg)](https://crates.io/crates/clipper2c-sys)
[![docs.rs](https://docs.rs/clipper2c-sys/badge.svg)](https://docs.rs/clipper2c-sys)

Rust FFI bindings to [Clipper2](https://github.com/AngusJohnson/Clipper2),
Angus Johnson's C++ library for 2D polygon clipping and offsetting.

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

- Polygon boolean operations — intersection, union, difference, XOR — via the `Clipper64` / `ClipperD` engine and `ClipperClipType`
- Polygon offsetting / inflation / deflation via `ClipperClipperOffset`, with corner and endpoint styles configurable through the `ClipperJoinType` and `ClipperEndType` enums (square / bevel / round / miter joins; polygon / joined / butt / square / round ends)
- Minkowski sum and difference (`clipper_path*_minkowski_{sum,diff}`) for sweeping an arbitrary polygon kernel along a path or polygon set — single-path and multi-path, both `_64` and `_D` — when the kernel isn't a circle and a plain offset won't do
- Path simplification (`clipper_*_simplify`)
- Point-in-polygon test
- Polygon area
- Hierarchical (PolyTree) results that preserve solid/hole nesting
- Integer (`_64`, `i64`) and floating-point (`_D`, `f64`) coordinate variants — see the [crate-level docs](https://docs.rs/clipper2c-sys) for the precision/range tradeoff

The Rust bindings cover the subset of Clipper2's C ABI that the
higher-level [`clipper2`](https://crates.io/crates/clipper2) crate
consumes. Other Clipper2 capabilities — rectangular clipping, the
named simplification variants (RDP, near-equal stripping, collinear
trimming), bounds, and SVG I/O — exist in the vendored C source but
are not currently allowlisted in the Rust bindings.

## Typical use cases

- **CAD / CNC / 3D-printing slicers** — toolpath offsetting (including drag-knife and other non-circular tool footprints via Minkowski sum), pocketing, infill generation, contour boolean operations
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
**C++17** toolchain on the build host. CI builds and runs the test
suite against the default toolchains shipped with each
GitHub-hosted runner:

| Runner         | Default C++ toolchain          |
|----------------|--------------------------------|
| ubuntu-latest  | system `g++` / `clang++`       |
| macos-latest   | Xcode-shipped `clang++`        |
| windows-latest | MSVC (Visual Studio Build Tools) |

`cc` auto-detects whichever of these is available. Older C++17-capable
toolchains are likely to work but are not regularly tested.

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
