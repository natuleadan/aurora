# AGENTS.md

Technical instructions for agents and contributors. The README describes the
product; this file explains how to build, verify and work in the repository.

## Repository layout

| Path | Purpose |
|---|---|
| `crates/aurora-tokens` | Theme tokens, radius scale and presets |
| `crates/aurora-primitives` | Styling and focus helpers |
| `crates/aurora-components` | The component catalog |
| `crates/aurora-cn` | Conditional class composition |
| `crates/aurora-lint` | Design-system lint rules |
| `crates/aurora-registry` | Component manifests and dependency graph |
| `crates/aurora-cli` | Project scaffolding and component installation |
| `apps/showcase/` | Rust application: website and cross-platform gallery |
| `docs/` | Product documentation |
| `tools/` | Validation entry points |

Dependencies point one way: tokens and class composition feed primitives and
components; the registry and linter sit above them; the CLI and showcase sit at
the top.

## Development

Use `pnpm` (never `npm` or `npx`). Run the single gate before pushing:

```sh
pnpm validate
```

`pnpm validate` is the single gate before pushing. It runs `cargo fmt --all --
--check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test
--workspace`, a Cargo check with the `min-publish-age` cooldown, the engine
checks, the design-system linter, and file-size and generalization checks.
Checks emit `PASS`, `FAIL` or `SKIP` and fail the build when any check fails.
The same command runs in CI. Never skip a check because a binary is missing.

The showcase runs on the fixed port 23202:

```sh
nsh run apps/showcase --target web --port 23202
```

The web target needs `llvm-tools` installed and its `llvm-ar` exposed through
`AR_wasm32_unknown_unknown`.

## Conventions

- English across source, docs, file names, output and commits.
- Comments, when needed, explain non-obvious code; longer explanation belongs
  in `docs/`.
- Source files stay within 500 lines.
- Components consume theme tokens instead of hardcoded colors or dimensions.
- Tests are written in Rust; UI is checked through the DOM, not screenshots.
- Never print secrets; `.env.example` holds placeholders only.
- Use short-lived branches (`feat/...`, `fix/...`, `docs/...`), open a pull
  request against `dev` and consolidate into `main` after review. Never rewrite
  published history.

## Commits

Format: `type(scope): description`, lowercase, in English.

- Header: 3 to 7 words counting `type(scope):`, maximum 100 characters, no
  trailing period, blank line after it.
- Body: at least three bullets `- **Title**: body`, each body with 3 to 7 words.
- Allowed scopes: `client`, `server`, `boxer`, `components`, `tokens`, `cli`,
  `showcase`, `tools`, `docs`, `ci`, `workflow`, `hooks`, `release`, `license`,
  `deps`.

## Releases

Versions derive from conventional commits since the last `v*` tag: `fix`,
`perf` and `revert` bump patch; `feat` bumps minor; a breaking change bumps
major; `docs`, `ci`, `chore`, `test`, `style` and `refactor` do not trigger a
release. Before a functional PR toward `main`, update `package.json` and
`CHANGELOG.md` and verify them with the release check.
