pub mod blocks;
pub mod catalog;
pub mod command;
pub mod components_live;
pub mod create;
pub mod home;
pub mod login;
pub mod stubs;
pub mod testbed;

use nsh_core::prelude::*;

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
            text("This section is not available yet.")
                .text_size(px(14.0))
                .text_color(move || nsh_base::theme().muted_foreground),
        )
}
