use nsh_core::prelude::*;

pub fn cli() -> impl IntoElement {
    view()
        .p_6()
        .gap_4()
        .max_w(px(900.0))
        .child(
            text("Aurora CLI")
                .text_size(px(28.0))
                .font_weight(FontWeight::BOLD),
        )
        .child(
            text("Distribution and scaffolding tooling for integrating Aurora into projects.")
                .text_size(px(14.0))
                .text_color(move || nsh_base::theme().muted_foreground),
        )
        .child(aurora_components::card(
            "Commands",
            "aurora init, aurora add, aurora list, aurora theme, aurora doctor, aurora build.",
        ))
}
