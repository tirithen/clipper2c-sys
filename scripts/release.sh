#!/usr/bin/env bash
# Usage: scripts/release.sh [patch|minor|major]   (default: patch)

set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

cargo deny check
cargo semver-checks check-release
cargo test
cargo release "${1:-patch}" --execute --no-verify
