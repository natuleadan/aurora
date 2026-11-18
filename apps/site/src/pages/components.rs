use nsh_core::prelude::*;

pub fn components() -> impl IntoElement {
    view()
        .p_6()
        .gap_4()
        .max_w(px(900.0))
        .child(
            text("Component Catalog")
                .text_size(px(28.0))
                .font_weight(FontWeight::BOLD),
        )
        .child(
            text("Typed, headless-ready Rust components that conform 1:1 to semantic interaction rules.")
                .text_size(px(14.0))
                .text_color(move || nsh_base::theme().muted_foreground),
        )
        .child(
            view()
                .flex_row()
                .gap_3()
                .child(aurora_components::badge("Button"))
                .child(aurora_components::badge("Badge"))
                .child(aurora_components::badge("Card"))
                .child(aurora_components::badge("Input"))
                .child(aurora_components::badge("Dialog"))
                .child(aurora_components::badge("Select")),
        )
        .child(
            aurora_components::card(
                "Headless Verification",
                "Every component is verified through headless interaction tests without screenshot flakiness.",
            ),
        )
}
