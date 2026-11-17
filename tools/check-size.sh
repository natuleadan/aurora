#!/usr/bin/env bash
set -uo pipefail

status=0
while IFS= read -r file; do
  lines=$(wc -l < "$file")
  if [ "$lines" -gt 500 ]; then
    echo "FAIL $file has $lines lines"
    status=1
  fi
done < <(find . -type f \
  \( -name '*.rs' -o -name '*.md' -o -name '*.sh' \) \
  -not -path './target/*' \
  -not -path './apps/showcase/target/*' \
  -not -path './apps/showcase/.nsh/*' \
  -not -path './.git/*' \
  -not -path './node_modules/*')

exit "$status"
