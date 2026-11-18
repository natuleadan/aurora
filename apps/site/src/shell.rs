use nsh_core::prelude::*;

use crate::router;

pub fn header(route: Signal<&'static str>) -> impl IntoElement {
    view()
        .px_6()
        .py_4()
        .border_b(px(1.0))
        .border_color(move || nsh_base::theme().border)
        .flex_row()
        .items_center()
        .justify_between()
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_3()
                .child(
                    text("Aurora")
                        .text_size(px(18.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(move || nsh_base::theme().foreground),
                )
                .child(aurora_components::badge("Design Standard")),
        )
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_4()
                .child(nav_link("Home", route))
                .child(nav_link("Docs", route))
                .child(nav_link("Components", route))
                .child(nav_link("AIElements", route))
                .child(nav_link("CLI", route))
                .child(nav_link("Theming", route)),
        )
}

fn nav_link(target: &'static str, route: Signal<&'static str>) -> impl IntoElement {
    let is_active = move || route.get() == target;
    nsh_base::button()
        .on_press(move |_: &()| router::navigate(target))
        .child(text(target).text_size(px(14.0)).text_color(move || {
            if is_active() {
                nsh_base::theme().primary
            } else {
                nsh_base::theme().muted_foreground
            }
        }))
}

pub fn footer() -> impl IntoElement {
    view()
        .px_6()
        .py_6()
        .border_t(px(1.0))
        .border_color(move || nsh_base::theme().border)
        .flex_row()
        .items_center()
        .justify_between()
        .child(
            text("Natuleadan® Aurora™ — Reusable Rust Design Standard")
                .text_size(px(12.0))
                .text_color(move || nsh_base::theme().muted_foreground),
        )
        .child(
            text("Built on Native Shell (NSH)")
                .text_size(px(12.0))
                .text_color(move || nsh_base::theme().muted_foreground),
        )
}
