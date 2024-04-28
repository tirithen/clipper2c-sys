# clipper2c-sys

[![crate.io](https://img.shields.io/crates/v/clipper2c-sys.svg)](https://crates.io/crates/clipper2c-sys)
[![docs.rs](https://docs.rs/clipper2c-sys/badge.svg)](https://docs.rs/clipper2c-sys)

Exposing unsafe FFI functions from the Clipper2 library for Rust.

**NOTE:** This crate is primarily intended to be used by the high level crate [clipper2](https://crates.io/crates/clipper2). If you just want to use Clipper2 from Rust that is probably the crate that you should try instead.

The create is a Rust wrapper around the C++ version of
[Clipper2](https://github.com/AngusJohnson/Clipper2) with the help of the [clipper2c-sys](https://github.com/songhuaixu/clipper2c) C wrapper.

Compile with cargo feature `generate-bindings` to generate bindings at build
time.

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
