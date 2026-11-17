# AGENTS.md — Aurora

Technical orientation for agents and contributors. The README describes the
product; this file explains how to build, verify and work in the repository.

## Overview

Aurora is a reusable component library written in Rust on a native UI engine.
The same components render on desktop, mobile, web, terminal and GPUI. The
workspace holds the library crates, a command line tool and a showcase app that
is also the project's website.

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

It runs formatting, clippy with `-D warnings`, the full workspace test suite,
the engine checks and repository hygiene checks. Checks emit `PASS`, `FAIL` or
`SKIP` and fail on any error. The same command runs in CI.

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
- Commit format: `type(scope): description`, scope in English, 3–7 words, a body
  with at least three bullets `- **Title**: body` and one newline between them.
  Author `leojara95`; never add a `Co-authored-by` trailer.
