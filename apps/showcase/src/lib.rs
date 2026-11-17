//! Aurora — a component showcase written entirely in Rust.
//!
//! The engine renders these elements with each platform's own views, so the
//! same source is the website, the desktop app and the terminal surface.

use nsh_base::theme;
use nsh_core::prelude::*;

/// Components exported to an optional TSX entry by the engine.
#[native_module("aurora/showcase")]
fn native(registry: &mut Registry) {
    registry.register::<Showcase>();
}

/// Application root: one responsive column with a header and live sections.
#[component]
pub fn Showcase() -> impl IntoElement {
    let scheme = signal(0_u8);

    scroll()
        .flex_1()
        .bg(move || theme().background)
        .text_color(move || theme().foreground)
        .child(
            view()
                .flex_1()
                .p_safe_area()
                .px_6()
                .py_4()
                .gap_6()
                .child(header(scheme))
                .child(hero())
                .child(section_intro(
                    "Component gallery",
                    "One model, native behavior on every surface.",
                ))
                .child(button_gallery())
                .child(badge_gallery())
                .child(surface_gallery())
                .child(footer()),
        )
}

fn header(scheme: Signal<u8>) -> impl IntoElement {
    let label = move || match scheme.get() {
        1 => "Light",
        2 => "Dark",
        _ => "System",
    };

    view()
        .flex_row()
        .items_center()
        .justify_between()
        .gap_4()
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_3()
                .child(
                    view()
                        .w(px(14.0))
                        .h(px(14.0))
                        .rounded(px(4.0))
                        .bg(move || theme().primary),
                )
                .child(
                    text("aurora")
                        .text_size(px(20.0))
                        .font_weight(FontWeight::BOLD),
                )
                .child(
                    text("Rust components for every platform")
                        .text_size(px(13.0))
                        .text_color(move || theme().muted_foreground),
                ),
        )
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_3()
                .child(
                    text("Search components")
                        .text_size(px(13.0))
                        .text_color(move || theme().muted_foreground),
                )
                .child(
                    view()
                        .px_3()
                        .py_1()
                        .rounded(px(6.0))
                        .border(px(1.0))
                        .border_color(move || theme().border)
                        .child(text("⌘ K").text_size(px(12.0))),
                )
                .child(
                    view()
                        .px_3()
                        .py_2()
                        .rounded(px(8.0))
                        .border(px(1.0))
                        .border_color(move || theme().border)
                        .hover(|s| s.bg(theme().muted))
                        .on_press(move || {
                            let next = (scheme.get() + 1) % 3;
                            scheme.set(next);
                            match next {
                                1 => set_color_scheme(Appearance::Light),
                                2 => set_color_scheme(Appearance::Dark),
                                _ => set_color_scheme(Appearance::System),
                            }
                        })
                        .child(text(label).text_size(px(13.0))),
                ),
        )
}

fn hero() -> impl IntoElement {
    view()
        .flex_1()
        .py_6()
        .gap_3()
        .child(
            text("The foundation for your Rust design system")
                .text_size(px(38.0))
                .font_weight(FontWeight::BOLD),
        )
        .child(
            text("Composable, accessible components with thoughtful defaults.")
                .text_size(px(16.0))
                .text_color(move || theme().muted_foreground),
        )
        .child(
            text("One component model for iOS, Android, macOS, Windows, Linux, web, terminal and GPUI.")
                .text_size(px(14.0))
                .text_color(move || theme().muted_foreground),
        )
}

fn section_intro(title: &'static str, subtitle: &'static str) -> impl IntoElement {
    view()
        .gap_1()
        .child(text(title).text_size(px(22.0)).font_weight(FontWeight::BOLD))
        .child(
            text(subtitle)
                .text_size(px(13.0))
                .text_color(move || theme().muted_foreground),
        )
}

fn button_gallery() -> impl IntoElement {
    view()
        .gap_3()
        .child(text("Button").text_size(px(15.0)).font_weight(FontWeight::BOLD))
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
}

#[derive(Clone, Copy)]
enum ActionKind {
    Primary,
    Secondary,
    Outline,
    Ghost,
}

fn action_button(label: &'static str, kind: ActionKind) -> impl IntoElement {
    let selected = signal(false);
    view()
        .px_4()
        .py_2()
        .rounded(px(8.0))
        .border(px(1.0))
        .border_color(move || match kind {
            ActionKind::Outline => theme().border,
            _ => theme().primary,
        })
        .bg(move || match kind {
            ActionKind::Primary => theme().primary,
            ActionKind::Secondary => theme().secondary,
            ActionKind::Outline | ActionKind::Ghost => theme().background,
        })
        .active(|s| s.opacity(0.7))
        .on_press(move || selected.update(|value| *value = !*value))
        .child(
            text(label).text_size(px(14.0)).text_color(move || match kind {
                ActionKind::Primary => theme().primary_foreground,
                ActionKind::Secondary => theme().secondary_foreground,
                ActionKind::Outline | ActionKind::Ghost => theme().foreground,
            }),
        )
}

fn badge_gallery() -> impl IntoElement {
    view()
        .gap_3()
        .child(text("Badge").text_size(px(15.0)).font_weight(FontWeight::BOLD))
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_2()
                .child(badge("Default"))
                .child(badge("Secondary"))
                .child(badge("Outline")),
        )
}

fn badge(label: &'static str) -> impl IntoElement {
    view()
        .px_3()
        .py_1()
        .rounded(px(999.0))
        .bg(move || theme().muted)
        .child(
            text(label)
                .text_size(px(12.0))
                .text_color(move || theme().muted_foreground),
        )
}

fn surface_gallery() -> impl IntoElement {
    view()
        .gap_3()
        .child(text("Card").text_size(px(15.0)).font_weight(FontWeight::BOLD))
        .child(
            view()
                .max_w(px(420.0))
                .p_4()
                .rounded(px(10.0))
                .border(px(1.0))
                .border_color(move || theme().border)
                .bg(move || theme().card)
                .gap_2()
                .child(
                    text("Native everywhere")
                        .text_size(px(16.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(move || theme().card_foreground),
                )
                .child(
                    text("Each control is drawn with the platform's own views and input system.")
                        .text_size(px(13.0))
                        .text_color(move || theme().muted_foreground),
                ),
        )
}

fn footer() -> impl IntoElement {
    view()
        .py_6()
        .border(px(1.0))
        .border_color(move || theme().border)
        .child(
            text("aurora — reusable Rust components, one design system, every platform.")
                .text_size(px(12.0))
                .text_color(move || theme().muted_foreground),
        )
}
