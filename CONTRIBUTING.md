# Contributing

Thanks for helping improve `aurora`.

## Before you start

- Read [AGENTS.md](AGENTS.md). It defines the layout, the one-way dependency
  rule, the language rule, and the validation suite.
- Work in English, including comments and commit messages.
- Do not bring in external code, assets, or names from projects studied for
  inspiration. Reimplement behavior with original work.

## Branches and commits

Create a branch from `dev`: `feat/<topic>`, `fix/<topic>`, or `docs/<topic>`.
Open the pull request against `dev`.

Commit messages follow this format:

```text
type(scope): three to seven words here
- **First Point**: what changed and why.
- **Second Point**: what changed and why.
- **Third Point**: what changed and why.
```

Use exactly one line break between bullets. Author is `leojara95`; never add a
`Co-authored-by` trailer.

## Checks

Before pushing, run the full local suite:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
nsh check
nsh test
aurora-lint
pnpm validate
```

Do not skip a check because a binary is missing. Use `SKIP` only with a stated
reason. UI changes must be checked through the DOM, not screenshots.

## Tests

Write tests in Rust next to the code they cover. Run the showcase on the fixed
port 23202 when changing visual behavior:

```sh
nsh run apps/showcase --target web --port 23202
```

## What a good change looks like

- A component change includes its demo, its documentation page, and tests.
- A theme change includes light and dark token coverage and contrast checks.
- A new linter rule includes examples of both a violation and a compliant fix.
- No file exceeds 500 lines.

## Repository conventions

- Dependencies point one way: tokens and class composition feed primitives and
  components; the linter and registry sit above them; the CLI and showcase sit
  at the top.
- Tests are written in Rust. The engine runner executes them headlessly.
- UI changes are verified through the DOM, never through screenshots.
- The development port is fixed at 23202.
- Reference material is inspiration only: it is never imported, copied or
  mentioned anywhere, including its child projects and derivatives.
- English everywhere, including comments and commit messages.
