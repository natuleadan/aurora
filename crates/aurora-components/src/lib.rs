use nsh_core::prelude::*;

pub fn button(label: &'static str, on_press: impl Fn() + 'static) -> impl IntoElement {
    nsh_base::button()
        .on_press(move |_: &()| on_press())
        .px_4()
        .py_2()
        .rounded(px(8.0))
        .bg(move || nsh_base::theme().primary)
        .active(|s| s.opacity(0.8))
        .child(
            text(label)
                .text_size(px(13.0))
                .text_color(move || nsh_base::theme().primary_foreground),
        )
}

pub fn badge(label: &'static str) -> impl IntoElement {
    view()
        .px_3()
        .py_1()
        .rounded(px(999.0))
        .bg(move || nsh_base::theme().muted)
        .child(
            text(label)
                .text_size(px(12.0))
                .text_color(move || nsh_base::theme().muted_foreground),
        )
}

pub fn card(title: &'static str, body: &'static str) -> impl IntoElement {
    view()
        .p_4()
        .rounded(px(10.0))
        .border(px(1.0))
        .border_color(move || nsh_base::theme().border)
        .gap_2()
        .child(
            text(title)
                .text_size(px(16.0))
                .font_weight(FontWeight::BOLD),
        )
        .child(
            text(body)
                .text_size(px(13.0))
                .text_color(move || nsh_base::theme().muted_foreground),
        )
}

pub fn input(value: Signal<String>, placeholder: &'static str) -> impl IntoElement {
    nsh_base::input()
        .value(value)
        .on_change(move |text: &str| value.set(text.to_string()))
        .placeholder(placeholder)
        .p_3()
        .border_1()
        .border_color(move || nsh_base::theme().border)
        .rounded(px(8.0))
}

pub fn checkbox(label: &'static str) -> impl IntoElement {
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
        .child(text(label))
}

pub fn switch(label: &'static str) -> impl IntoElement {
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
        .child(text(label))
}

pub fn progress(value: Signal<f32>) -> impl IntoElement {
    nsh_base::progress()
        .value(value)
        .gap_2()
        .track_style(|s| {
            s.h(px(8.0))
                .rounded(px(999.0))
                .overflow_hidden()
                .bg(move || nsh_base::theme().muted)
        })
        .indicator_style(|s| s.h_full().bg(move || nsh_base::theme().primary))
        .child(text(move || format!("{:.0}%", value.get())))
}
