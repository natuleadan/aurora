use nsh_core::prelude::*;

use crate::ui;

pub fn home(route: Signal<&'static str>) -> impl IntoElement {
    view().flex_1().gap_6().child(hero(route)).child(gallery())
}

fn hero(route: Signal<&'static str>) -> impl IntoElement {
    view()
        .flex_1()
        .py_6()
        .gap_4()
        .child(ui::chip("Live presets and a theme editor are here"))
        .child(
            text("Interfaces that feel at home on every platform")
                .text_size(px(40.0))
                .font_weight(FontWeight::BOLD),
        )
        .child(
            text("A compact set of Rust components with native behavior, accessible by default and styled entirely from your theme.")
                .text_size(px(16.0))
                .text_color(move || nsh_base::theme().muted_foreground),
        )
        .child(
            text("Build once for desktop, mobile, web, terminal and GPUI.")
                .text_size(px(14.0))
                .text_color(move || nsh_base::theme().muted_foreground),
        )
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_3()
                .child(cta("Explore components", "Gallery", route))
                .child(cta("Open the designer", "Create", route)),
        )
}

fn cta(label: &'static str, target: &'static str, route: Signal<&'static str>) -> impl IntoElement {
    view()
        .px_4()
        .py_2()
        .rounded(px(8.0))
        .bg(move || nsh_base::theme().primary)
        .active(|s| s.opacity(0.7))
        .on_press(move || {
            route.set(target);
            crate::router::navigate(target);
        })
        .child(
            text(label)
                .text_size(px(14.0))
                .text_color(move || nsh_base::theme().primary_foreground),
        )
}

pub fn gallery() -> impl IntoElement {
    view()
        .flex_1()
        .gap_4()
        .child(ui::section_title(
            "Component gallery",
            "One model, native behavior on every surface.",
        ))
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_3()
                .child(action_button("Default", ActionKind::Primary))
                .child(action_button("Secondary", ActionKind::Secondary))
                .child(action_button("Outline", ActionKind::Outline))
                .child(action_button("Ghost", ActionKind::Ghost)),
        )
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_2()
                .child(ui::pill("Default"))
                .child(ui::pill("Secondary"))
                .child(ui::pill("Outline")),
        )
        .child(ui::card(
            "Native everywhere",
            "Each control is drawn with the platform's own views and input system.",
        ))
}

#[derive(Clone, Copy)]
enum ActionKind {
    Primary,
    Secondary,
    Outline,
    Ghost,
}

fn action_button(label: &'static str, kind: ActionKind) -> impl IntoElement {
    let pressed = signal(false);
    view()
        .px_4()
        .py_2()
        .rounded(px(8.0))
        .border(px(1.0))
        .border_color(move || match kind {
            ActionKind::Outline => nsh_base::theme().border,
            _ => nsh_base::theme().primary,
        })
        .bg(move || match kind {
            ActionKind::Primary => nsh_base::theme().primary,
            ActionKind::Secondary => nsh_base::theme().secondary,
            ActionKind::Outline | ActionKind::Ghost => nsh_base::theme().background,
        })
        .active(|s| s.opacity(0.7))
        .on_press(move || pressed.update(|value| *value = !*value))
        .child(
            text(label)
                .text_size(px(14.0))
                .text_color(move || match kind {
                    ActionKind::Primary => nsh_base::theme().primary_foreground,
                    ActionKind::Secondary => nsh_base::theme().secondary_foreground,
                    ActionKind::Outline | ActionKind::Ghost => nsh_base::theme().foreground,
                }),
        )
}
