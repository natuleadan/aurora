# Components

## Naming

Component APIs use plain UI vocabulary and a Rust `au_` prefix, for example
`au_button()` and `au_card()`. Each component lives in one module under
`crates/aurora-components/src/`, with a demo, a documentation page, and tests.

## Families

| Family | Components |
|---|---|
| Action | button, button-group, badge, toggle, toggle-group, kbd |
| Containers | card, item, separator, aspect-ratio, collapsible, accordion, scroll-area, resizable, sidebar, table, tabs |
| Forms | input, textarea, label, checkbox, switch, radio-group, select, native-select, combobox, field, form, input-group, input-otp, slider, calendar, date-picker, questionnaire |
| Overlays | dialog, alert-dialog, drawer, sheet, popover, tooltip, hover-card, dropdown-menu, context-menu, menubar, command |
| Data and feedback | alert, avatar, progress, skeleton, spinner, empty, marker, toast, message, message-scroller, chart, data-table, bubble, attachment |
| Navigation | breadcrumb, navigation-menu, pagination, direction |

## Completion criteria

A component is complete when it has:

1. The expected variants and states: default, secondary, outline, ghost,
   destructive, link; small, medium, and large; disabled, loading, selected,
   and expanded.
2. Token-driven styling with no literal colors or dimensions.
3. Accessible behavior delegated to the engine's base components.
4. A clean linter result under documented contracts.
5. An interactive demo, a documentation page, and Rust tests.
6. Web, terminal, and at least one native desktop target verified.

## Platform adaptation

Phones use sheets and bottom menus; desktops use popups and visible focus;
the web emits semantic markup; the terminal uses the character grid. A
feature present on one surface is expected on all surfaces with the
interaction style appropriate to each.
