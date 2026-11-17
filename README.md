# Natuleadan® Aurora™

> Reusable, accessible UI components for Rust.

## What it is

Aurora is a styled component layer built on Native Shell (NSH). One component model, one theme and one set of behaviors render natively across desktop, mobile, web, terminal and embedded surfaces.

## What it provides

It provides a semantic theme with presets, a catalog of typed components, a class-composition utility, a design-system linter and a CLI that copies component source into a project so you own what you ship.

## Architecture

Dependencies point one way: tokens and class composition feed primitives and components, the registry and linter sit above them, and the CLI and showcase sit at the top of the graph.

## Repository layout

The workspace holds library crates under `crates/`, the showcase under `apps/`, and product documentation under `docs/`. All source lives in the development branch until release.

## Getting started

Clone `https://github.com/natuleadan/aurora.git`. This branch holds only the
project baseline; the source code, build manifests and full validation suite
live in the `dev` branch. Switch to `dev` to build and run the project.

## Development

Work happens on short-lived branches merged into `dev`. Commits follow
`type(scope): description` with a body of at least three bullets; commitlint
enforces the policy locally and in CI. The `main` branch is protected and holds
only this minimal baseline until the first release.

## Status

Active development. The component catalog and headless UI test matrix are in place; the first published version will be 0.1.0.

## License

Apache-2.0. See [LICENSE](LICENSE).

## Trademark

Natuleadan® Aurora™ is a trademark of Natuleadan. Other constellation
repositories: Pulsar, Aurora, Pleaides, Andromeda and Orion.
