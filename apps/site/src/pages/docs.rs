use nsh_core::prelude::*;

pub fn docs() -> impl IntoElement {
    view()
        .p_6()
        .gap_4()
        .max_w(px(900.0))
        .child(
            text("Documentation")
                .text_size(px(28.0))
                .font_weight(FontWeight::BOLD),
        )
        .child(
            text("Aurora is structured into clean, decoupled layers following a strict one-way dependency model:")
                .text_size(px(14.0))
                .text_color(move || nsh_base::theme().muted_foreground),
        )
        .child(
            aurora_components::card(
                "Architecture: One-Way Layering",
                "tokens + cn -> primitives -> components -> registry / lint -> cli -> apps",
            ),
        )
        .child(
            aurora_components::card(
                "Zero-Runtime-Cost Tokens",
                "Theme tokens provide deterministic semantics for light and dark modes across GPUI, Web and Desktop.",
            ),
        )
}
