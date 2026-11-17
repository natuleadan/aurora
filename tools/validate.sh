#!/usr/bin/env bash
set -uo pipefail

pass=0
fail=0

run() {
  local name="$1"
  shift
  if timeout 300 "$@" >/dev/null 2>&1; then
    echo "PASS $name"
    pass=$((pass + 1))
  else
    echo "FAIL $name"
    fail=$((fail + 1))
  fi
}

run "cargo fmt" cargo fmt --all -- --check
run "cargo clippy" cargo clippy --workspace --all-targets -- -D warnings
run "cargo test" cargo test --workspace
run "cargo check" cargo --config 'resolver.min-publish-age="7 days"' check --workspace
run "nsh check" nsh check apps/showcase
run "nsh test" nsh test apps/showcase
run "aurora-lint" cargo run -q -p aurora-lint -- apps/showcase/src/lib.rs apps/showcase/src/router.rs apps/showcase/src/shell.rs apps/showcase/src/ui.rs apps/showcase/src/pages/*.rs crates/aurora-tokens/src/*.rs crates/aurora-cn/src/*.rs crates/aurora-registry/src/*.rs crates/aurora-components/src/*.rs crates/aurora-cli/src/*.rs
run "file size" bash tools/check-size.sh
run "generalization" bash tools/check-general.sh
run "notices" node helpers/tools/third-party-notices.mjs --check

echo "TOTAL PASS=$pass FAIL=$fail"
[ "$fail" -eq 0 ]
