#!/usr/bin/env bash
set -e

for d in problems/*/; do
  if [ -f "$d/Cargo.toml" ]; then
    echo "Cleaning $d"
    (cd "$d" && cargo clean)
  fi
done
