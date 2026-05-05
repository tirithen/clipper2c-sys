# Contributing to clipper2c-sys

Thanks for your interest. Issues, suggestions, and pull requests are
all welcome.

## Scope

This crate is the raw FFI surface over Clipper2's C ABI. The
high-level safe Rust API lives in
[`clipper2`](https://crates.io/crates/clipper2). If a contribution is
about ergonomics or safety on top of these bindings, it usually
belongs there. Contributions to this crate are typically:

- exposing additional items from the Clipper2 C ABI that the bindings
  do not yet cover,
- fixing memory-safety, lifetime, or correctness bugs in the FFI
  glue,
- updating the vendored Clipper2 source to a newer upstream release.

## Local checks before opening a PR

```sh
cargo fmt
cargo test
scripts/wasm-check.sh    # downloads WASI SDK on first run, into .tmp/
```

CI runs the same `cargo test` on Linux, macOS, and Windows, plus the
`wasm32-unknown-unknown` build.

If you change the C/C++ headers or `clipper2c.h`, regenerate the
checked-in bindings:

```sh
scripts/regenerate-bindings.sh
```

## Commit messages

This repo uses [Conventional Commits](https://www.conventionalcommits.org/).
The release tooling derives the next version and the CHANGELOG entry
directly from commit messages, so the type prefix matters:

- `feat:` — adds a new item to the public surface (patch bump on 0.x)
- `fix:` — bug fix (patch bump)
- `perf:` — performance improvement
- `docs:`, `test:`, `refactor:`, `chore:`, `ci:`, `build:`, `style:`
  — kept out of the changelog
- `BREAKING CHANGE:` footer — bumps the minor on 0.x (e.g. 0.1.6 →
  0.2.0). Use this for MSRV bumps, removed bindings, or signature
  changes.

Do not edit `Cargo.toml`'s `version` field or `CHANGELOG.md` by hand.
Those are produced by `scripts/release.sh` and the cargo-release /
git-cliff configuration in `release.toml` and `cliff.toml`.

## Licensing

By submitting a contribution you agree that it is dual-licensed
under [Apache-2.0](LICENSE-APACHE.md) and [MIT](LICENSE-MIT.md), as
stated at the bottom of the README. The vendored Clipper2 C++ source
under `clipper2c/vendor/Clipper2/` retains its own Boost Software
License 1.0 and is not affected.
