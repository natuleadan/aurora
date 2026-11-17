# Quality

## Local parity with CI

Every check that runs in CI runs locally first, without omissions.

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo --config 'resolver.min-publish-age="7 days"' check --workspace
nsh check
nsh test
aurora-lint
pnpm validate
```

`pnpm validate` is the single entry point and fails if any check fails. Use
pnpm and cargo only; never npm or npx. Checks emit `PASS`, `FAIL`, or `SKIP`
and exit non-zero on failure.

## Design-system linter

`aurora-lint` is an original Rust crate built on a Rust syntax parser. It
applies design-system contracts to the code and produces agent-facing
diagnostics.

| Rule | Detects | Suggested fix |
|---|---|---|
| `no_restyle` | Disallowed styling on a component | Use the component's size or variant |
| `no_raw_colors` | Literal colors | Use a theme token |
| `no_arbitrary_values` | Arbitrary dimensions | Use the spacing scale |
| `no_inline_styles` | Raw styles bypassing the system | Use typed helpers |
| `no_unknown_classes` | Unsupported classes | Use a class from the valid vocabulary |
| `require_static_classes` | Dynamically built classes | Use literals or a static table |

Diagnostics have the form `FAIL <file>:<line> <rule> — <problem>. Use <fix>
(<location>).` Contracts are configured per component and per project. The
automatic fix only applies safe transformations. The showcase is linted too,
with explicit overrides.

## Class composition

`aurora-cn` joins class fragments conditionally and resolves conflicts against
the vocabulary the engine supports. It has no dependencies, exposes a macro and
a function, and generates its conflict table from the engine's supported
classes. It is allocation-light on the common path and has tests for padding,
margin, color, radius, display, and variant conflicts.

## Tests

Tests are written in Rust. Unit tests live next to the code; integration tests
live in `tests/`. The engine's headless runner executes them; it is not a test
language. UI tests assert on the DOM, never on screenshots.
