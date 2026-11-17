# Theming

## Principle

The theme belongs to the application. `aurora` defines the token vocabulary
and the scales; components only consume tokens.

## Semantic tokens

`background`, `foreground`, `card`, `card-foreground`, `popover`,
`popover-foreground`, `primary`, `primary-foreground`, `secondary`,
`secondary-foreground`, `muted`, `muted-foreground`, `accent`,
`accent-foreground`, `destructive`, `destructive-foreground`, `border`,
`input`, `ring`, `chart-1` through `chart-5`, and the sidebar family.

Every token exists in light and dark mode. Dark mode overrides the same names;
there is no parallel palette.

## Radius scale

A single base radius generates the derived scale: `sm = 0.6x`, `md = 0.8x`,
`lg = 1x`, `xl = 1.4x`, `2xl = 1.8x`, `3xl = 2.2x`, `4xl = 2.6x`. Changing
the base updates the whole scale.

## Presets

Presets combine a base color family, a radius, and typography. They are JSON
files loaded through the application configuration and exposed to components
by the engine's theme selector.

## Verification

- Radius derivation tests.
- Light and dark coverage for every token.
- Minimum contrast checks for foreground and background pairs.
- Live theme switching in the showcase without rebuilding.
- The linter rejects literal colors.
