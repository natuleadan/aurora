use nsh_core::prelude::*;

use crate::ui;

static ACTION: &[(&str, &str, &str)] = &[
    ("button", "Action", "Ready"),
    ("button-group", "Action", "Ready"),
    ("toggle", "Action", "Ready"),
    ("toggle-group", "Action", "Ready"),
    ("checkbox", "Forms", "Ready"),
    ("switch", "Forms", "Ready"),
    ("radio-group", "Forms", "Ready"),
];

static FORMS: &[(&str, &str, &str)] = &[
    ("input", "Forms", "Ready"),
    ("textarea", "Forms", "Ready"),
    ("label", "Forms", "Ready"),
    ("number-input", "Forms", "Ready"),
    ("otp-input", "Forms", "Ready"),
    ("select", "Forms", "Ready"),
    ("slider", "Forms", "Ready"),
    ("time-field", "Forms", "Ready"),
    ("date-picker", "Forms", "Ready"),
    ("color-picker", "Forms", "Ready"),
    ("calendar", "Forms", "Ready"),
    ("combobox", "Forms", "Ready"),
    ("keyboard", "Forms", "Ready"),
    ("text-selection", "Forms", "Ready"),
];

static OVERLAYS: &[(&str, &str, &str)] = &[
    ("dialog", "Overlays", "Ready"),
    ("alert-dialog", "Overlays", "Ready"),
    ("popover", "Overlays", "Ready"),
    ("tooltip", "Overlays", "Ready"),
    ("sheet", "Overlays", "Ready"),
    ("toast", "Overlays", "Ready"),
    ("hover-card", "Overlays", "Ready"),
    ("context-menu", "Overlays", "Ready"),
    ("menu", "Overlays", "Ready"),
    ("link", "Navigation", "Ready"),
    ("nav-stack", "Navigation", "Ready"),
    ("toolbar", "Navigation", "Ready"),
];

static DATA: &[(&str, &str, &str)] = &[
    ("table", "Data", "Ready"),
    ("data-table", "Data", "Ready"),
    ("chart", "Data", "Ready"),
    ("plot", "Data", "Ready"),
    ("tree", "Data", "Ready"),
    ("diff-view", "Data", "Ready"),
    ("rich-editor", "Data", "Ready"),
    ("terminal", "Data", "Ready"),
    ("text-view", "Data", "Ready"),
    ("avatar", "Data", "Ready"),
    ("badge", "Data", "Ready"),
    ("pagination", "Navigation", "Ready"),
    ("breadcrumb", "Navigation", "Ready"),
];

static STRUCTURE: &[(&str, &str, &str)] = &[
    ("card", "Layout", "Ready"),
    ("accordion", "Layout", "Ready"),
    ("collapsible", "Layout", "Ready"),
    ("tabs", "Layout", "Ready"),
    ("separator", "Layout", "Ready"),
    ("skeleton", "Layout", "Ready"),
    ("resizable", "Layout", "Ready"),
    ("dock", "Layout", "Ready"),
    ("scrollbar", "Layout", "Ready"),
    ("safe-area", "Layout", "Ready"),
    ("carousel", "Layout", "Ready"),
    ("command", "Navigation", "Ready"),
    ("editor", "Data", "Ready"),
    ("emoji-picker", "Forms", "Ready"),
    ("alert", "Feedback", "Ready"),
    ("progress", "Feedback", "Ready"),
    ("kbd", "Feedback", "Ready"),
];

static PRIMITIVES: &[(&str, &str, &str)] = &[
    ("view", "Primitives", "Ready"),
    ("text", "Primitives", "Ready"),
    ("image", "Primitives", "Ready"),
    ("scroll", "Primitives", "Ready"),
    ("list", "Primitives", "Ready"),
    ("canvas", "Primitives", "Ready"),
    ("shader", "Primitives", "Ready"),
    ("web-view", "Primitives", "Ready"),
    ("video", "Primitives", "Ready"),
    ("overlay", "Primitives", "Ready"),
    ("control-flow", "Primitives", "Ready"),
];

pub fn table() -> impl IntoElement {
    let mut body = view().flex_1().gap_1().child(header());
    for (name, category, status) in ACTION
        .iter()
        .chain(FORMS.iter())
        .chain(OVERLAYS.iter())
        .chain(DATA.iter())
        .chain(STRUCTURE.iter())
        .chain(PRIMITIVES.iter())
    {
        body = body.child(row(name, category, status));
    }
    body
}

fn header() -> impl IntoElement {
    view()
        .flex_row()
        .items_center()
        .gap_3()
        .pb_2()
        .border_b_1()
        .border_color(move || nsh_base::theme().border)
        .child(cell("Component", true))
        .child(cell("Category", true))
        .child(cell("Status", true))
}

fn row(name: &'static str, category: &'static str, status: &'static str) -> impl IntoElement {
    view()
        .flex_row()
        .items_center()
        .gap_3()
        .py_2()
        .border_b_1()
        .border_color(move || nsh_base::theme().border)
        .child(cell(name, false))
        .child(cell(category, false))
        .child(cell(status, false))
}

fn cell(label: &'static str, strong: bool) -> impl IntoElement {
    text(label)
        .text_size(px(13.0))
        .font_weight(if strong {
            FontWeight::BOLD
        } else {
            FontWeight::NORMAL
        })
        .text_color(move || {
            if strong {
                nsh_base::theme().foreground
            } else {
                nsh_base::theme().muted_foreground
            }
        })
}

pub fn summary() -> impl IntoElement {
    ui::section_title(
        "Component catalog",
        "Every component, grouped by the job it does.",
    )
}
