use nsh_core::prelude::*;

pub fn theming() -> impl IntoElement {
    view()
        .p_6()
        .gap_4()
        .max_w(px(900.0))
        .child(
            text("Semantic Theming")
                .text_size(px(28.0))
                .font_weight(FontWeight::BOLD),
        )
        .child(
            text("A unified token palette defining foreground, background, muted, border, ring and chart semantics.")
                .text_size(px(14.0))
                .text_color(move || nsh_base::theme().muted_foreground),
        )
        .child(
            aurora_components::card(
                "Theme Presets",
                "Exportable theme profiles conforming to OKLCH color spaces with WCAG contrast guarantees.",
            ),
        )
}
