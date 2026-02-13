#!/usr/bin/env bash
set -e

while read dir lc name; do
  [ -z "$dir" ] && continue
  [[ "$dir" == \#* ]] && continue

  base="problems/$dir/src/bin"
  file="$base/${lc}-${name}.rs"

  mkdir -p "$base"

  if [ ! -f "$file" ]; then
    echo "// LeetCode ${lc#lc}: ${name//-/ }" > "$file"
    echo "" >> "$file"
    echo "pub struct Solution {}" >> "$file"
    echo "" >> "$file"
    echo "impl Solution {" >> "$file"
    echo "}" >> "$file"
  fi
done < neetcode250.txt
