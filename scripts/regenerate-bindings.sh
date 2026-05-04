#!/usr/bin/env bash
# Usage: scripts/regenerate-bindings.sh
#
# Regenerate generated/bindings.rs from clipper2c/include/. Run after
# editing the C headers (e.g. adding doc comments) or bumping the
# bundled Clipper2 version.

set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

# bindgen uses libclang via libclang-sys, which honours LIBCLANG_PATH.
# An ESP / xtensa / other cross-toolchain libclang on the developer's
# machine will assert with a pointer_width mismatch against host rustc.
# Force the system libclang for this invocation.
unset LIBCLANG_PATH

exec cargo build --features update-bindings
