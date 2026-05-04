# clipper2c-sys

[![crate.io](https://img.shields.io/crates/v/clipper2c-sys.svg)](https://crates.io/crates/clipper2c-sys)
[![docs.rs](https://docs.rs/clipper2c-sys/badge.svg)](https://docs.rs/clipper2c-sys)

Exposing unsafe FFI functions from the Clipper2 library for Rust.

**NOTE:** This crate is primarily intended to be used by the high level crate [clipper2](https://crates.io/crates/clipper2). If you just want to use Clipper2 from Rust that is probably the crate that you should try instead.

This crate is a thin Rust FFI wrapper around [Clipper2](https://github.com/AngusJohnson/Clipper2),
Angus Johnson's polygon clipping and offsetting library — the canonical C++
implementation. Clipper2 is vendored verbatim into this repository so builds
stay reproducible without depending on a system install or network access.
The C ABI bridge started as a fork of [songhuaixu/clipper2c](https://github.com/songhuaixu/clipper2c)
(Boost Software License 1.0, preserved in `clipper2c/LICENSE`) and has since
been substantially rewritten — most notably for zero-copy.

Compile with cargo feature `generate-bindings` to generate bindings at build
time.

## Versioning

The crate's SemVer tracks the Rust FFI surface. The bundled Clipper2
version is recorded directly in upstream's [`clipper.version.h`](clipper2c/vendor/Clipper2/CPP/Clipper2Lib/include/clipper2/clipper.version.h)
— that file is the single source of truth for which Clipper2 release
this crate is built against.

```toml
clipper2c-sys = "0.1"
```

## Early days

This project is in a super early stage. Expect breaking changes now and then.

Please also feel free to come with suggestions on how the API can be simplified
or send code contributions directly. See
[CONTRIBUTING.md](https://github.com/tirithen/clipper2c-sys/blob/main/CONTRIBUTING.md)
for more details.

## License

Licensed under either of [Apache License, Version 2.0](https://github.com/tirithen/clipper2c-sys/blob/main/LICENSE-APACHE.md)
or [MIT license](https://github.com/tirithen/clipper2c-sys/blob/main/LICENSE-MIT.md)
at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Serde by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
