use nsh_core::prelude::*;

use crate::ui;

pub fn live() -> impl IntoElement {
    view()
        .gap_3()
        .child(ui::section_title(
            "Live controls",
            "Real components with native behavior and token-driven styling.",
        ))
        .child(field())
        .child(aurora_components::checkbox("Accept the terms"))
        .child(aurora_components::switch("Notifications"))
        .child(bar())
        .child(aurora_components::tabs(&TABS))
        .child(aurora_components::accordion(&ITEMS))
}

static TABS: [(&str, &str, &str); 3] = [
    (
        "overview",
        "Overview",
        "A short summary of the current project.",
    ),
    ("activity", "Activity", "Recent changes and events."),
    ("settings", "Settings", "Options for this project."),
];

static ITEMS: [(&str, &str, &str); 2] = [
    (
        "shipping",
        "When does it ship?",
        "Orders placed before 2 pm leave the same day.",
    ),
    (
        "returns",
        "How do I return it?",
        "Print the label from your account.",
    ),
];

fn field() -> impl IntoElement {
    let value = signal(String::new());
    aurora_components::input(value, "Type here")
}

fn bar() -> impl IntoElement {
    let value = signal(40.0_f32);
    aurora_components::progress(value)
}
