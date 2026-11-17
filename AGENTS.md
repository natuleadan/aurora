# AGENTS.md — Aurora

Technical orientation for agents and contributors. The README describes the
product; this file explains how to build, verify and work in the repository.

## Language and content

1. English across source, docs, file names, output and commits.
2. Non-obvious code may carry a short comment when it aids understanding.
   Comments are written in English; longer explanation belongs in `docs/`.

## Layout

| Path | Purpose |
|---|---|
| `apps/showcase/` | The Rust application used as the website and cross-platform gallery. |
| `crates/` | Library crates, one concern per crate. |
| `docs/` | Product documentation. |
| `tools/` | Validation entry points. |

Dependencies point one way: foundational crates feed components; the CLI and
the showcase sit at the top. Lower layers never import higher layers.

## Rules

1. Components consume theme tokens; they never hardcode colors or dimensions.
2. Tests are written in Rust. The engine runner is a headless runner only.
3. UI changes are verified through the DOM, never through screenshots.
4. The development port is fixed at 23202.
5. Files stay within 500 lines. Split by concern when they grow.
6. Never print secrets. `.env.example` holds placeholders only.
7. Use pnpm and cargo. Never npm or npx.
8. Do not publish without explicit approval.

## Validation

Run the complete local suite before any change is considered done:

```sh
pnpm validate
```

It runs formatting, linting, tests, type checks, an engine check and repository
hygiene checks. It is the same command CI runs. Checks emit `PASS`, `FAIL` or
`SKIP` and fail the build when any check fails.

## Git

Never initialize git in a parent directory. Work on `feat/`, `fix/` or `docs/`
branches, open pull requests against `dev`, and consolidate into `main` only
after review. Never rewrite published history.

Commit format: `type(scope): description`, scope in English, 3–7 words, a body
with at least three bullets formatted as `- **Title**: body`, with one newline
between bullets. Author: `leojara95`. Never add a `Co-authored-by` trailer.
