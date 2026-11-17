# Architecture

## Layers

The rendering engine supplies native primitives and unstyled base components:
behavior, focus, accessibility, and platform-native interaction. `aurora` is
the styled layer above them. It owns tokens, variants, and composition; it does
not reimplement the engine.

```
engine primitives  →  engine base components  →  aurora  →  showcase
```

## Crates

| Crate | Responsibility |
|---|---|
| `aurora-tokens` | Colors, radius, typography, spacing, light and dark modes, presets. |
| `aurora-primitives` | Shared styling, focus, and state helpers. |
| `aurora-components` | The component catalog, one module per component. |
| `aurora-cn` | Conditional class joining and conflict resolution. |
| `aurora-lint` | AST-based design-system rules and agent-facing diagnostics. |
| `aurora-registry` | Manifests, dependency graph, and canonical sources. |
| `aurora-cli` | Project scaffolding and component installation. |
| `aurora-test-support` | Rust helpers for headless tests. |

## Dependency direction

Dependencies point one way:

```
tokens → primitives → { components, cn } → { lint, registry } → cli → showcase
```

A lower layer never imports a higher layer. The CLI copies canonical component
sources into a consumer crate and rewrites import paths; there is no second
copy to maintain.

## Constraints

- Components consume tokens; they never hardcode colors or dimensions.
- One component model across all targets, with platform-native interaction.
- Files are at most 500 lines.
- Tests are written in Rust.
- The showcase uses the fixed development port 23202.
