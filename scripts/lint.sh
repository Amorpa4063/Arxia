#!/usr/bin/env bash
set -euo pipefail

echo "=== Arxia Lint ==="
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
echo "=== Lint complete ==="
