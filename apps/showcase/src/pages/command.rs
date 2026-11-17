use nsh_core::prelude::*;

use crate::ui;

static COMMANDS: [(&str, &str); 6] = [
    ("Components", "Components"),
    ("Blocks", "Blocks"),
    ("Charts", "Charts"),
    ("Directory", "Directory"),
    ("Typeset", "Typeset"),
    ("New project", "Create"),
];

pub fn command(route: Signal<&'static str>) -> impl IntoElement {
    let query = signal(String::new());
    let mut results = view().gap_1();
    for (label, target) in COMMANDS {
        results = results.child(when(
            move || {
                let needle = query.get().to_lowercase();
                needle.is_empty() || label.to_lowercase().contains(&needle)
            },
            move || ui::button(label, ui::ButtonStyle::Ghost, move || route.set(target)),
        ));
    }
    view()
        .flex_1()
        .gap_4()
        .child(ui::section_title(
            "Command palette",
            "Search and jump to a section.",
        ))
        .child(
            nsh_base::input()
                .value(query)
                .on_change(move |text: &str| query.set(text.to_string()))
                .placeholder("Type a command...")
                .p_3()
                .border_1()
                .border_color(move || nsh_base::theme().border)
                .rounded(px(8.0)),
        )
        .child(results)
}
