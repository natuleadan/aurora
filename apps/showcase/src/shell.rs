use nsh_core::prelude::*;

use crate::router;
use crate::ui;

pub fn header(
    route: Signal<&'static str>,
    menu: Signal<bool>,
    scheme: Signal<u8>,
) -> impl IntoElement {
    view()
        .px_6()
        .py_4()
        .gap_3()
        .border_b(px(1.0))
        .border_color(move || nsh_base::theme().border)
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_3()
                .child(brand())
                .child(when(
                    move || window().width >= px(600.0),
                    move || nav_panel(route),
                ))
                .child(view().flex_1())
                .child(
                    view()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(search_field(route))
                        .child(stars())
                        .child(theme_button(scheme))
                        .child(new_button(route))
                        .child(when(
                            move || window().width < px(600.0),
                            move || menu_button(menu),
                        )),
                ),
        )
        .child(when(
            move || window().width < px(600.0) && menu.get(),
            move || nav_panel(route),
        ))
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
        .child(when(
            move || window().width >= px(1500.0),
            move || meta_text("Rust components for every platform"),
        ))
}

fn search_field(route: Signal<&'static str>) -> impl IntoElement {
    ui::button("Search...", ui::ButtonStyle::Outline, move || {
        route.set("Command");
        router::navigate("Command");
    })
}

fn stars() -> impl IntoElement {
    let count = signal("...".to_string());
    spawn(async move {
        if let Ok(response) = fetch("https://api.github.com/repos/natuleadan/aurora").await
            && response.ok()
            && let Ok(value) = serde_json::from_str::<serde_json::Value>(&response.text())
            && let Some(number) = value.get("stargazers_count").and_then(|v| v.as_i64())
        {
            let label = if number >= 1000 {
                format!("{}k", number / 1000)
            } else {
                number.to_string()
            };
            count.set(label);
        }
    });
    view()
        .px_2()
        .py_1()
        .rounded(px(6.0))
        .border(px(1.0))
        .border_color(move || nsh_base::theme().border)
        .child(text(move || count.get()).text_size(px(12.0)))
}

fn meta_text(label: &'static str) -> impl IntoElement {
    text(label)
        .text_size(px(13.0))
        .text_color(move || nsh_base::theme().muted_foreground)
}

fn theme_button(scheme: Signal<u8>) -> impl IntoElement {
    ui::dynamic_button(
        move || match scheme.get() {
            1 => "Light".to_string(),
            2 => "Dark".to_string(),
            _ => "System".to_string(),
        },
        move || {
            let next = (scheme.get() + 1) % 3;
            scheme.set(next);
            match next {
                1 => set_color_scheme(Appearance::Light),
                2 => set_color_scheme(Appearance::Dark),
                _ => set_color_scheme(Appearance::System),
            }
        },
    )
}

fn new_button(route: Signal<&'static str>) -> impl IntoElement {
    ui::button("New", ui::ButtonStyle::Primary, move || {
        route.set("Create");
        router::navigate("Create");
    })
}

fn menu_button(menu: Signal<bool>) -> impl IntoElement {
    ui::button("Menu", ui::ButtonStyle::Outline, move || {
        menu.update(|open| *open = !*open)
    })
}

fn nav_panel(route: Signal<&'static str>) -> impl IntoElement {
    view()
        .flex_row()
        .items_center()
        .gap_1()
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
    ui::button(label, ui::ButtonStyle::Ghost, move || {
        route.set(target);
        router::navigate(target);
    })
}

pub fn footer() -> impl IntoElement {
    view()
        .px_6()
        .py_4()
        .border_t(px(1.0))
        .border_color(move || nsh_base::theme().border)
        .child(meta_text(
            "Aurora — reusable Rust components, one design system, every platform.",
        ))
}
