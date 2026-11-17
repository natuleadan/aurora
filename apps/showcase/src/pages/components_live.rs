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
}

fn field() -> impl IntoElement {
    let value = signal(String::new());
    aurora_components::input(value, "Type here")
}

fn bar() -> impl IntoElement {
    let value = signal(40.0_f32);
    aurora_components::progress(value)
}
