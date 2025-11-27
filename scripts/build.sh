#!/usr/bin/env bash
set -euo pipefail

echo "=== Arxia Build ==="
cargo build --workspace
echo "=== Build complete ==="
