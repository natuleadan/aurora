#!/usr/bin/env bash
set -uo pipefail

status=0
while IFS= read -r file; do
  lines=$(wc -l < "$file")
  if [ "$lines" -gt 500 ]; then
    echo "FAIL $file has $lines lines"
    status=1
  fi
done < <(find . \
  -type d \( -name target -o -name node_modules -o -name .git -o -name .nsh \) -prune -o \
  -type f \( -name '*.rs' -o -name '*.md' -o -name '*.sh' \) -print)

exit "$status"
