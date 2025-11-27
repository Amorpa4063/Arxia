#!/usr/bin/env bash
set -euo pipefail

echo "=== Arxia Test Suite ==="
cargo test --workspace
echo "=== All tests passed ==="
