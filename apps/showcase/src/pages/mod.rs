//! Showcase pages.

pub mod create;
pub mod home;

use nsh_core::prelude::*;

/// A section that has not been implemented yet in this batch.
pub fn placeholder(title: &'static str) -> impl IntoElement {
    view()
        .flex_1()
        .gap_3()
        .child(
            text(title)
                .text_size(px(28.0))
                .font_weight(FontWeight::BOLD),
        )
        .child(
            text("This section is being built in a later batch.")
                .text_size(px(14.0))
                .text_color(move || nsh_base::theme().muted_foreground),
        )
}
