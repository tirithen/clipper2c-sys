#!/usr/bin/env bash
# Usage: scripts/release.sh [--execute]
# Default is dry run. Version is inferred from conventional commits.

set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

# cargo-semver-checks fires the bindgen path under --all-features;
# an ESP / xtensa cross-toolchain libclang on PATH asserts on a
# pointer-width mismatch. Force the system libclang for this run.
unset LIBCLANG_PATH

case "${1:-}" in
    --execute) execute_flag="--execute" ;;
    "")        execute_flag="" ;;
    *) echo "release.sh: unknown arg '$1' (use --execute to release)" >&2; exit 2 ;;
esac

cargo deny check
cargo semver-checks check-release
cargo test
scripts/wasm-check.sh

next=$(git cliff --bumped-version 2>/dev/null)
if [[ -z "$next" ]]; then
    echo "release.sh: no release-worthy conventional commits since last tag" >&2
    exit 0
fi

# --no-publish in dry mode: cargo-release does not bump Cargo.toml in
# dry mode, so its dry-mode publish would package the previous
# version and emit a misleading "already exists on crates.io" warning.
if [[ -n "$execute_flag" ]]; then
    cargo release "${next#v}" $execute_flag --no-verify
else
    cargo release "${next#v}" --no-verify --no-publish
fi
