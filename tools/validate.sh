#!/usr/bin/env bash
set -uo pipefail

pass=0
fail=0

run() {
  local name="$1"
  shift
  local output
  if output=$("$@" 2>&1); then
    echo "PASS $name"
    pass=$((pass + 1))
  else
    echo "FAIL $name"
    printf '%s\n' "$output" | tail -40
    fail=$((fail + 1))
  fi
}

run "cargo fmt" cargo fmt --all -- --check
run "cargo clippy" cargo clippy --workspace --all-targets -- -D warnings
run "cargo test" cargo test --workspace
run "cargo check" cargo --config 'resolver.min-publish-age="7 days"' check --workspace
run "nsh check" nsh check apps/showcase
run "nsh test" nsh test apps/showcase
run "file size" bash tools/check-size.sh
run "generalization" bash tools/check-general.sh

echo "TOTAL PASS=$pass FAIL=$fail"
[ "$fail" -eq 0 ]
