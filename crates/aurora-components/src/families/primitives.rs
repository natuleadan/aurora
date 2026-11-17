use nsh_core::prelude::*;
use nsh_core::prelude::{
    overlay as nsh_overlay, scroll as nsh_scroll, text as nsh_text, view as nsh_view,
};

fn placeholder(label: &'static str, role: AccessibilityRole) -> impl IntoElement {
    nsh_view()
        .role(role)
        .label(label)
        .h(px(120.0))
        .p_4()
        .rounded(px(8.0))
        .border(px(1.0))
        .border_color(move || nsh_base::theme().border)
        .bg(move || nsh_base::theme().card)
        .flex_col()
        .items_center()
        .justify_center()
        .gap_2()
        .child(
            nsh_text(label)
                .text_size(px(14.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(move || nsh_base::theme().foreground),
        )
        .child(
            nsh_text("Preview unavailable")
                .text_size(px(12.0))
                .text_color(move || nsh_base::theme().muted_foreground),
        )
}

pub fn view(content: impl IntoElement) -> impl IntoElement {
    nsh_view()
        .p_4()
        .rounded(px(8.0))
        .bg(move || nsh_base::theme().background)
        .child(content)
}

pub fn text(content: &'static str) -> impl IntoElement {
    nsh_text(content)
        .text_size(px(14.0))
        .text_color(move || nsh_base::theme().foreground)
}

pub fn image(alt: &'static str) -> impl IntoElement {
    nsh_view()
        .role(AccessibilityRole::Image)
        .label(alt)
        .size(px(120.0))
        .rounded(px(8.0))
        .border(px(1.0))
        .border_color(move || nsh_base::theme().border)
        .bg(move || nsh_base::theme().muted)
        .items_center()
        .justify_center()
        .child(
            nsh_text(alt)
                .text_size(px(12.0))
                .text_color(move || nsh_base::theme().muted_foreground),
        )
}

pub fn scroll(content: impl IntoElement) -> impl IntoElement {
    nsh_scroll()
        .p_4()
        .bg(move || nsh_base::theme().background)
        .child(content)
}

pub fn list(items: &'static [&'static str]) -> impl IntoElement {
    let mut column = nsh_view()
        .role(AccessibilityRole::List)
        .flex_col()
        .gap_1()
        .p_2()
        .rounded(px(8.0))
        .border(px(1.0))
        .border_color(move || nsh_base::theme().border);
    for item in items {
        column = column.child(
            nsh_view()
                .px_3()
                .py_2()
                .rounded(px(6.0))
                .hover(|s| s.bg(nsh_base::theme().accent))
                .child(
                    nsh_text(*item)
                        .text_size(px(13.0))
                        .text_color(move || nsh_base::theme().foreground),
                ),
        );
    }
    column
}

pub fn canvas(label: &'static str) -> impl IntoElement {
    placeholder(label, AccessibilityRole::None)
}

pub fn shader(label: &'static str) -> impl IntoElement {
    placeholder(label, AccessibilityRole::None)
}

pub fn web_view(label: &'static str) -> impl IntoElement {
    placeholder(label, AccessibilityRole::None)
}

pub fn video(label: &'static str) -> impl IntoElement {
    placeholder(label, AccessibilityRole::None)
}

pub fn overlay(label: &'static str) -> impl IntoElement {
    nsh_overlay()
        .label(label)
        .p_4()
        .rounded(px(8.0))
        .border(px(1.0))
        .border_color(move || nsh_base::theme().border)
        .bg(move || nsh_base::theme().popover)
        .child(
            nsh_text(label)
                .text_size(px(14.0))
                .text_color(move || nsh_base::theme().popover_foreground),
        )
}

pub fn control_flow(label: &'static str) -> impl IntoElement {
    placeholder(label, AccessibilityRole::None)
}
