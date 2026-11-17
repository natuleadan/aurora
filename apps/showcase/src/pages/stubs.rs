use nsh_core::prelude::*;

use crate::ui;

pub fn installation() -> impl IntoElement {
    page(
        "Installation",
        "Set up a project and start adding components.",
        &[
            (
                "01 · Scaffold",
                "Create a Rust application with the engine template.",
            ),
            (
                "02 · Add the theme",
                "Copy a theme file beside the app metadata.",
            ),
            (
                "03 · Add components",
                "Install a component with the command line tool.",
            ),
        ],
    )
}

pub fn components_index() -> impl IntoElement {
    view()
        .flex_1()
        .gap_4()
        .child(ui::section_title(
            "Components",
            "Every control in one place, grouped by the job it does.",
        ))
        .child(ui::card(
            "Action",
            "Button, button group, toggle and toggle group.",
        ))
        .child(ui::card(
            "Forms",
            "Input, checkbox, switch, select and slider.",
        ))
        .child(ui::card(
            "Overlays",
            "Dialog, menu, popover, tooltip and toast.",
        ))
        .child(ui::card(
            "Data",
            "Table, chart, badge, avatar and progress.",
        ))
        .child(super::components_live::live())
}

pub fn directory() -> impl IntoElement {
    page(
        "Directory",
        "Find projects and resources built with the library.",
        &[
            ("Showcase", "The live gallery in this application."),
            ("Themes", "Presets exported from the designer."),
            ("Community", "Packages and examples from other authors."),
        ],
    )
}

pub fn blocks() -> impl IntoElement {
    view()
        .flex_1()
        .gap_4()
        .child(ui::section_title(
            "Blocks",
            "Ready-made compositions you can copy into a project.",
        ))
        .child(ui::card(
            "Dashboard",
            "A sidebar, summary cards, a chart and a data table.",
        ))
        .child(ui::card(
            "Authentication",
            "Sign in and sign up screens with validation.",
        ))
        .child(ui::card(
            "Settings",
            "A profile form with sections and toggles.",
        ))
        .child(ui::section_title(
            "Preview",
            "A block assembled from live components.",
        ))
        .child(super::components_live::live())
}

pub fn charts() -> impl IntoElement {
    page(
        "Area Charts",
        "Charts drawn natively from your own rows.",
        &[
            ("Area", "A filled series over time."),
            ("Stacked", "Several series sharing one axis."),
            ("Tooltip", "Values on hover or focus."),
        ],
    )
}

pub fn typeset() -> impl IntoElement {
    view()
        .flex_1()
        .gap_4()
        .child(ui::section_title(
            "Typeset",
            "The typographic scale of the theme.",
        ))
        .child(
            text("Display heading")
                .text_size(px(40.0))
                .font_weight(FontWeight::BOLD),
        )
        .child(
            text("Section heading")
                .text_size(px(24.0))
                .font_weight(FontWeight::BOLD),
        )
        .child(text("Body text at the default size.").text_size(px(16.0)))
        .child(
            text("Muted caption text for secondary information.")
                .text_size(px(13.0))
                .text_color(move || nsh_base::theme().muted_foreground),
        )
}

fn page(
    title: &'static str,
    subtitle: &'static str,
    rows: &[(&'static str, &'static str)],
) -> impl IntoElement {
    let mut content = view()
        .flex_1()
        .gap_4()
        .child(ui::section_title(title, subtitle));
    for (label, body) in rows {
        content = content.child(ui::card(label, body));
    }
    content
}
