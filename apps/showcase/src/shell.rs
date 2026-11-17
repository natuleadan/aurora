use nsh_core::prelude::*;

use crate::router;

pub fn header(
    route: Signal<&'static str>,
    menu: Signal<bool>,
    scheme: Signal<u8>,
) -> impl IntoElement {
    view()
        .gap_3()
        .child(
            view()
                .flex_row()
                .items_center()
                .justify_between()
                .gap_4()
                .child(brand())
                .child(
                    view()
                        .flex_row()
                        .items_center()
                        .gap_3()
                        .child(search_field())
                        .child(meta_text("Repository"))
                        .child(theme_button(scheme))
                        .child(new_button(route))
                        .child(menu_button(menu)),
                ),
        )
        .child(when(move || menu.get(), move || nav_panel(route)))
}

fn brand() -> impl IntoElement {
    view()
        .flex_row()
        .items_center()
        .gap_3()
        .child(
            view()
                .w(px(14.0))
                .h(px(14.0))
                .rounded(px(4.0))
                .border(px(1.0))
                .border_color(move || nsh_base::theme().border),
        )
        .child(
            text("aurora")
                .text_size(px(20.0))
                .font_weight(FontWeight::BOLD),
        )
        .child(meta_text("Rust components for every platform"))
}

fn search_field() -> impl IntoElement {
    view()
        .px_3()
        .py_2()
        .rounded(px(8.0))
        .border(px(1.0))
        .border_color(move || nsh_base::theme().border)
        .child(meta_text("Search..."))
}

fn meta_text(label: &'static str) -> impl IntoElement {
    text(label)
        .text_size(px(13.0))
        .text_color(move || nsh_base::theme().muted_foreground)
}

fn theme_button(scheme: Signal<u8>) -> impl IntoElement {
    let label = move || match scheme.get() {
        1 => "Light",
        2 => "Dark",
        _ => "System",
    };
    view()
        .px_3()
        .py_2()
        .rounded(px(8.0))
        .border(px(1.0))
        .border_color(move || nsh_base::theme().border)
        .hover(|s| s.bg(nsh_base::theme().muted))
        .on_press(move || {
            let next = (scheme.get() + 1) % 3;
            scheme.set(next);
            match next {
                1 => set_color_scheme(Appearance::Light),
                2 => set_color_scheme(Appearance::Dark),
                _ => set_color_scheme(Appearance::System),
            }
        })
        .child(text(label).text_size(px(13.0)))
}

fn new_button(route: Signal<&'static str>) -> impl IntoElement {
    view()
        .px_4()
        .py_2()
        .rounded(px(8.0))
        .bg(move || nsh_base::theme().primary)
        .active(|s| s.opacity(0.7))
        .on_press(move || {
            route.set("Create");
            router::navigate("Create");
        })
        .child(
            text("New")
                .text_size(px(13.0))
                .text_color(move || nsh_base::theme().primary_foreground),
        )
}

fn menu_button(menu: Signal<bool>) -> impl IntoElement {
    view()
        .px_3()
        .py_2()
        .rounded(px(8.0))
        .border(px(1.0))
        .border_color(move || nsh_base::theme().border)
        .on_press(move || menu.update(|open| *open = !*open))
        .child(text("Menu").text_size(px(13.0)))
}

fn nav_panel(route: Signal<&'static str>) -> impl IntoElement {
    view()
        .flex_row()
        .items_center()
        .gap_4()
        .child(nav_item("Home", route, "Home"))
        .child(nav_item("Docs", route, "Docs"))
        .child(nav_item("Components", route, "Components"))
        .child(nav_item("Blocks", route, "Blocks"))
        .child(nav_item("Charts", route, "Charts"))
        .child(nav_item("Directory", route, "Directory"))
        .child(nav_item("Typeset", route, "Typeset"))
        .child(nav_item("Create", route, "Create"))
}

fn nav_item(
    label: &'static str,
    route: Signal<&'static str>,
    target: &'static str,
) -> impl IntoElement {
    view()
        .px_2()
        .py_1()
        .rounded(px(6.0))
        .hover(|s| s.bg(nsh_base::theme().muted))
        .on_press(move || {
            route.set(target);
            router::navigate(target);
        })
        .child(meta_text(label))
}

pub fn footer() -> impl IntoElement {
    view()
        .py_6()
        .border(px(1.0))
        .border_color(move || nsh_base::theme().border)
        .child(meta_text(
            "Aurora — reusable Rust components, one design system, every platform.",
        ))
}
