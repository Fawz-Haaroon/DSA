#!/bin/bash

while read dir lc name; do
  [ -z "$dir" ] && continue

  base="problems/$dir/src/bin"
  file="$base/${lc}-${name}.rs"

  mkdir -p "$base"

  if [ ! -f "$file" ]; then
    echo "// LeetCode ${lc#lc}: ${name//-/ }" > "$file"
  fi
done < neetcode150.txt
