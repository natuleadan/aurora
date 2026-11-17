#!/usr/bin/env bash
set -uo pipefail

status=0
if grep -rIn -E '/Users/|[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}' \
  --include='*.rs' --include='*.md' --include='*.toml' --include='*.json' \
  --include='*.sh' --exclude='check-general.sh' --exclude-dir=tools --exclude-dir=.git --exclude-dir=target --exclude-dir=.nsh \
  --exclude-dir=node_modules . >/dev/null 2>&1; then
  echo "FAIL public file contains a machine path or email"
  status=1
fi

exit "$status"
