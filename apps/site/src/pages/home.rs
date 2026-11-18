use nsh_core::prelude::*;

use crate::router;

pub fn home() -> impl IntoElement {
    view()
        .p_6()
        .gap_4()
        .max_w(px(900.0))
        .child(
            view()
                .gap_2()
                .child(
                    text("Natuleadan® Aurora™")
                        .text_size(px(32.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(move || nsh_base::theme().foreground),
                )
                .child(
                    text(
                        "A reusable, platform-independent design standard built entirely in Rust.",
                    )
                    .text_size(px(18.0))
                    .text_color(move || nsh_base::theme().muted_foreground),
                ),
        )
        .child(
            view()
                .flex_row()
                .gap_3()
                .child(aurora_components::button("Get Started", || {
                    router::navigate("Docs");
                }))
                .child(aurora_components::button("Browse Components", || {
                    router::navigate("Components");
                })),
        )
        .child(
            view()
                .gap_3()
                .p_4()
                .rounded(px(8.0))
                .border_1()
                .border_color(move || nsh_base::theme().border)
                .child(
                    text("Key Capabilities")
                        .text_size(px(16.0))
                        .font_weight(FontWeight::BOLD),
                )
                .child(aurora_components::badge("100% Rust Native"))
                .child(aurora_components::badge("Decoupled Design Primitives"))
                .child(aurora_components::badge("20 AI Elements for Agent Systems")),
        )
}
