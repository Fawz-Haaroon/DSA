#!/usr/bin/env bash
set -e

echo "Cleaning problem crates..."
for d in problems/*/; do
  if [ -f "$d/Cargo.toml" ]; then
    echo "  → $d"
    (cd "$d" && cargo clean)
  fi
done

echo
echo "Cleaning tool crates..."
for d in tools/*/; do
  if [ -f "$d/Cargo.toml" ]; then
    echo "  → $d"
    (cd "$d" && cargo clean)
  fi
done

echo
echo "Done."

