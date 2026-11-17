# CLI

## Commands

| Command | Purpose |
|---|---|
| `aurora init` | Create project configuration, theme, and minimum dependencies. |
| `aurora add <component>` | Copy a canonical component source into the project and rewrite imports. |
| `aurora list` | List available components and blocks with their status. |
| `aurora theme <preset>` | Apply or export a theme preset. |
| `aurora doctor` | Check toolchain, pinned versions, and configuration. |
| `aurora build` | Produce a publishable registry with manifests and checksums. |

## Distribution model

Core crates are normal dependencies. Each component is copied into the
consumer project so the user owns the code they ship. The CLI resolves
dependencies between components and refuses to overwrite local changes
without confirmation.

The registry is local and offline-first. Manifests list name, files,
dependencies, version, and checksum.

## Manifest example

```json
{
  "name": "button",
  "version": "0.1.0",
  "files": ["button.rs"],
  "dependencies": ["aurora-primitives", "aurora-cn"],
  "category": "action"
}
```

## Rules

The CLI does not execute fetched code. Generated files use English and contain
no absolute paths. Every file operation reports `PASS` or `SKIP`.
