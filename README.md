# Natuleadan® Aurora™

**Reusable, accessible UI components for Rust.**

`aurora` is a styled component layer built on Native Shell (NSH). One component
model, one theme, and one set of behaviors for iOS, Android, macOS, Windows,
Linux, the web, the terminal, and GPUI.

> Status: batch 0 in progress. The showcase renders on the fixed port 23202.

## What it provides

- A semantic theme with light and dark modes, a derived radius scale, and
  exportable presets.
- A catalog of components with typed variants and platform-native behavior.
- A small class-composition utility for conditional class strings.
- A design-system linter that explains violations to coding agents.
- A CLI that scaffolds a project and copies component source into it, so you
  own the code you ship.
- A showcase application that runs as a website and as a native app on every
  supported platform.

## Layout

| Path | Purpose |
|---|---|
| `apps/showcase/` | The showcase application (website and cross-platform gallery). |
| `crates/aurora-tokens/` | Theme tokens, presets, spacing, typography. |
| `crates/aurora-primitives/` | Shared styling and focus primitives. |
| `crates/aurora-components/` | The component catalog. |
| `crates/aurora-cn/` | Class composition and conflict resolution. |
| `crates/aurora-lint/` | Design-system linter. |
| `crates/aurora-registry/` | Component manifests and dependency graph. |
| `crates/aurora-cli/` | The `aurora` command-line tool. |
| `docs/`, `themes/`, `registry/` | Documentation and data files. |
| `tools/` | Local validation entry point. |

Dependencies point one way only: tokens and class composition feed primitives
and components; the registry and linter sit above them; the CLI and the
showcase sit at the top.

## Development

Requires Rust 1.100 or newer, NSH 0.5.1, and pnpm.

```sh
pnpm validate
nsh run apps/showcase --target web --port 23202
```

The showcase is served on the fixed port **23202**. Run the same checks locally
that CI runs before opening a pull request; see [CONTRIBUTING.md](CONTRIBUTING.md).

## Documentation

- [Architecture](docs/ARCHITECTURE.md)
- [Components](docs/COMPONENTS.md)
- [Theming](docs/THEMING.md)
- [CLI](docs/CLI.md)
- [Showcase](docs/SHOWCASE.md)
- [Quality](docs/QUALITY.md)

## License

MIT. See [LICENSE](LICENSE).
