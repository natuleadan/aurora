# Natuleadan® Aurora™

> Reusable, accessible UI components for Rust.

Aurora is a styled component layer built on Native Shell (NSH). One component
model, one theme and one set of behaviors across desktop, mobile, web, terminal
and embedded surfaces.

## What it provides

- A semantic theme with light and dark modes and exportable presets.
- A catalog of typed, platform-native components.
- A class-composition utility and a design-system linter.
- A CLI that scaffolds a project and copies component source into it.

## Layout

| Path | Purpose |
|---|---|
| `apps/showcase/` | Website and cross-platform gallery |
| `crates/aurora-tokens/` | Theme tokens and presets |
| `crates/aurora-components/` | Component catalog |
| `crates/aurora-cli/` | Project scaffolding CLI |
| `tools/` | Local validation entry point |

## Development

Requires Rust 1.100 or newer, NSH and pnpm. Run the same checks locally that CI
runs before opening a pull request.

```sh
pnpm validate
```

## Documentation

- [Architecture](docs/ARCHITECTURE.md)
- [Components](docs/COMPONENTS.md)
- [Contributing](CONTRIBUTING.md)

## License

Apache-2.0. See [LICENSE](LICENSE).
