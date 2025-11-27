#!/usr/bin/env bash
set -euo pipefail

# Neutralize file timestamps to a fixed date.
echo "=== Metadata Cleanup ==="

echo "Checking for confidential files..."
FOUND=$(find . -name "*.docx" -o -name "*.pdf" -o -name "*CLAUDE*" -o -name "*audit*v[0-9]*" 2>/dev/null | head -5)
if [ -n "$FOUND" ]; then
    echo "ERROR: Confidential files found:"
    echo "$FOUND"
    exit 1
fi

echo "Checking for machine paths..."
PATHS=$(grep -ri "C:\\Users\|/home/\|/Users/" \
    --include="*.rs" --include="*.md" --include="*.toml" \
    --include="*.yml" --include="*.yaml" . 2>/dev/null | head -5)
if [ -n "$PATHS" ]; then
    echo "ERROR: Machine paths found:"
    echo "$PATHS"
    exit 1
fi

echo "Neutralizing timestamps..."
find . -not -path './.git/*' -exec touch -t 202601010000 {} \; 2>/dev/null || true

echo "=== Metadata cleanup complete ==="
