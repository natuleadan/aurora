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
        .child(aurora_components::dialog_trigger(
            "Open dialog",
            "A native dialog",
            "It follows each platform's presentation and dismissal.",
        ))
        .child(aurora_components::hint(
            "Hover me",
            "A token-styled tooltip",
        ))
        .child(aurora_components::radio_group(&PLANS))
        .child(ui::section_title(
            "More components",
            "Wrappers over native behavior and tokens.",
        ))
        .child(aurora_components::toggle("Bold"))
        .child(aurora_components::collapsible(
            "Details",
            "Extra information appears here.",
        ))
        .child(aurora_components::separator())
        .child(aurora_components::skeleton(220.0, 14.0))
        .child(aurora_components::label("A form label"))
        .child(aurora_components::alert(
            "Heads up",
            "This is an alert surface.",
        ))
        .child(aurora_components::avatar("AR"))
        .child(aurora_components::kbd("Cmd K"))
        .child(aurora_components::breadcrumb(&[
            "Home",
            "Docs",
            "Components",
        ]))
        .child(aurora_components::table(&ROWS))
        .child(aurora_components::pagination(&PAGES))
}

static ROWS: [(&str, &str); 3] = [
    ("Left label", "Right value"),
    ("Second row", "42"),
    ("Third row", "Active"),
];

static PAGES: [&str; 4] = ["1", "2", "3", "..."];

static PLANS: [(&str, &str); 3] = [("basic", "Basic"), ("pro", "Pro"), ("team", "Team")];

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
