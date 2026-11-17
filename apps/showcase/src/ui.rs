use nsh_core::prelude::*;

#[derive(Clone, Copy)]
pub enum ButtonStyle {
    Primary,
    Outline,
    Ghost,
}

pub fn button(
    label: &'static str,
    style: ButtonStyle,
    on_press: impl Fn() + 'static,
) -> impl IntoElement {
    styled_button(text(label).text_size(px(13.0)), style, on_press)
}

pub fn dynamic_button(
    label: impl Fn() -> String + 'static,
    on_press: impl Fn() + 'static,
) -> impl IntoElement {
    styled_button(
        text(label).text_size(px(13.0)),
        ButtonStyle::Outline,
        on_press,
    )
}

fn styled_button(
    content: Text,
    style: ButtonStyle,
    on_press: impl Fn() + 'static,
) -> impl IntoElement {
    nsh_base::button()
        .on_press(move |_: &()| on_press())
        .px_4()
        .py_2()
        .rounded(px(8.0))
        .border(px(1.0))
        .border_color(move || match style {
            ButtonStyle::Primary => nsh_base::theme().primary,
            _ => nsh_base::theme().border,
        })
        .bg(move || match style {
            ButtonStyle::Primary => nsh_base::theme().primary,
            _ => nsh_base::theme().background,
        })
        .hover(|s| s.bg(nsh_base::theme().accent))
        .active(|s| s.opacity(0.8))
        .child(content.text_color(move || match style {
            ButtonStyle::Primary => nsh_base::theme().primary_foreground,
            _ => nsh_base::theme().foreground,
        }))
}

pub fn chip(label: &'static str) -> impl IntoElement {
    view()
        .px_3()
        .py_1()
        .rounded(px(999.0))
        .border(px(1.0))
        .border_color(move || nsh_base::theme().border)
        .child(
            text(label)
                .text_size(px(12.0))
                .text_color(move || nsh_base::theme().muted_foreground),
        )
}

pub fn section_title(title: &'static str, subtitle: &'static str) -> impl IntoElement {
    view()
        .gap_1()
        .child(
            text(title)
                .text_size(px(22.0))
                .font_weight(FontWeight::BOLD),
        )
        .child(
            text(subtitle)
                .text_size(px(13.0))
                .text_color(move || nsh_base::theme().muted_foreground),
        )
}

pub fn card(title: &'static str, body: &'static str) -> impl IntoElement {
    view()
        .max_w(px(520.0))
        .p_4()
        .rounded(px(10.0))
        .border(px(1.0))
        .border_color(move || nsh_base::theme().border)
        .bg(move || nsh_base::theme().card)
        .gap_2()
        .child(
            text(title)
                .text_size(px(16.0))
                .font_weight(FontWeight::BOLD)
                .text_color(move || nsh_base::theme().card_foreground),
        )
        .child(
            text(body)
                .text_size(px(13.0))
                .text_color(move || nsh_base::theme().muted_foreground),
        )
}

pub fn pill(label: &'static str) -> impl IntoElement {
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
