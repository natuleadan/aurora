# Showcase

## Nature

The showcase is a Rust application built on the engine. The same sources are
published as a website and as a native application on every supported target.
There is no TypeScript and no template server.

## Local run

```sh
nsh run apps/showcase --target web --port 23202
```

The fixed development port is 23202. Visual verification uses the DOM, not
screenshots.

## Structure

```
apps/showcase/
├── nsh.json          name, id, appearance, theme, rust, root
├── theme.json        default theme
└── native/src/
    ├── lib.rs        module and asset registration
    ├── shell.rs      navigation, sidebar, responsive layout
    ├── ui/           palette, finishes, original icons, visual primitives
    ├── pages/        one page per route
    └── demos/        one demo per component
```

## Sections

Home; documentation pages; one index page per component; blocks with filters
and a full-screen preview; charts; typeset; a community directory; and a
visual preset builder.

Each component page shows a description, a live demo, the CLI command to add
it, a usage example, variants, and API details. Code samples are
generated from the real sources so they cannot go stale.

## Interactions

Navigation menu, global command search with shortcuts, light/dark/system
theme toggle with persistence, real interactive demos, copyable code snippets,
previous/next navigation, anchor links, full-screen block previews, and a
responsive layout that turns the sidebar into tabs or a stack on narrow
screens.

## Budget

Pages and demos load on demand. Bundle size and time to first interaction are
measured continuously; regressions need a justification.
