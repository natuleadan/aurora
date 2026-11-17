use nsh_core::prelude::*;

use crate::ui;

pub fn live() -> impl IntoElement {
    view()
        .flex_1()
        .gap_4()
        .child(ui::section_title(
            "Live controls",
            "Real components with native behavior and token-driven styling.",
        ))
        .child(field())
        .child(check())
        .child(toggle())
        .child(bar())
}

fn field() -> impl IntoElement {
    let value = signal(String::new());
    nsh_base::input()
        .value(value)
        .on_change(move |text: &str| value.set(text.to_string()))
        .placeholder("Type here")
        .p_3()
        .border_1()
        .border_color(move || nsh_base::theme().border)
        .rounded(px(8.0))
}

fn check() -> impl IntoElement {
    let on = signal(false);
    nsh_base::checkbox()
        .checked(on)
        .on_change(move |next: &bool| on.set(*next))
        .flex_row()
        .items_center()
        .gap_2()
        .indicator_style(|s| {
            s.size(px(18.0))
                .items_center()
                .justify_center()
                .rounded(px(4.0))
                .border_2()
                .border_color(move || nsh_base::theme().border)
                .checked(|s| s.bg(move || nsh_base::theme().primary))
        })
        .indicator(text(move || if on.get() { "x" } else { "" }))
        .child(text("Accept the terms"))
}

fn toggle() -> impl IntoElement {
    let on = signal(false);
    nsh_base::switch()
        .checked(on)
        .on_change(move |next: &bool| on.set(*next))
        .flex_row()
        .items_center()
        .gap_3()
        .track_style(|s| {
            s.w(px(40.0))
                .p(px(2.0))
                .rounded(px(999.0))
                .bg(move || nsh_base::theme().muted)
                .checked(|s| s.bg(move || nsh_base::theme().primary))
        })
        .thumb_style(|s| {
            s.size(px(16.0))
                .rounded(px(999.0))
                .bg(move || nsh_base::theme().background)
                .checked(|s| s.translate_x(px(20.0)))
        })
        .child(text("Notifications"))
}

fn bar() -> impl IntoElement {
    let value = signal(40.0_f32);
    nsh_base::progress()
        .value(value)
        .gap_2()
        .track_style(|s| {
            s.h(px(8.0))
                .rounded(px(999.0))
                .overflow_hidden()
                .bg(move || nsh_base::theme().muted)
        })
        .indicator_style(|s| {
            s.h_full()
                .rounded(px(999.0))
                .bg(move || nsh_base::theme().primary)
        })
        .child(text(move || format!("{:.0}%", value.get())))
}
