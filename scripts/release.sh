#!/usr/bin/env bash
# Usage: scripts/release.sh
#
# The bump level is computed from conventional commit history since
# the last tag (via 'git cliff --bumped-version' and the [bump]
# section of cliff.toml). No manual version selection.

set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

cargo deny check
cargo semver-checks check-release
cargo test
scripts/wasm-check.sh

next=$(git cliff --bumped-version 2>/dev/null)
if [[ -z "$next" ]]; then
    echo "release.sh: no release-worthy conventional commits since last tag" >&2
    exit 0
fi

cargo release "${next#v}" --execute --no-verify
