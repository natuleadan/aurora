use nsh_core::prelude::*;

use crate::router;
use crate::ui;

static TABS: [(&str, &str, &str); 3] = [
    ("Sidebar", "BlocksSidebar", "sidebar"),
    ("Login", "BlocksLogin", "login"),
    ("Signup", "BlocksSignup", "signup"),
];

static DEVICES: [(&str, f32); 3] = [("Desktop", 1080.0), ("Tablet", 720.0), ("Mobile", 390.0)];

pub fn blocks(route: Signal<&'static str>) -> impl IntoElement {
    let initial = match route.get() {
        "BlocksSidebar" => 0,
        "BlocksSignup" => 2,
        _ => 1,
    };
    let selected = signal(initial);
    let variant = signal(1_usize);
    let device = signal(0_usize);
    let show_code = signal(false);

    let mut tabs = view().flex_row().items_center().gap_2();
    for (index, (label, target, _)) in TABS.iter().enumerate() {
        let target = *target;
        tabs = tabs.child(ui::button(label, ui::ButtonStyle::Outline, move || {
            selected.set(index);
            route.set(target);
            router::navigate(target);
        }));
    }

    let mut variants = view().flex_row().items_center().gap_1();
    for number in 1..=5 {
        variants = variants.child(ui::button(
            match number {
                1 => "01",
                2 => "02",
                3 => "03",
                4 => "04",
                _ => "05",
            },
            ui::ButtonStyle::Outline,
            move || variant.set(number),
        ));
    }

    let mut devices = view().flex_row().items_center().gap_1();
    for (index, (label, _)) in DEVICES.iter().enumerate() {
        devices = devices.child(ui::button(label, ui::ButtonStyle::Outline, move || {
            device.set(index)
        }));
    }

    let command = move || {
        let slug = match selected.get() {
            0 => "sidebar",
            2 => "signup",
            _ => "login",
        };
        format!("aurora add {slug}-0{}", variant.get())
    };

    view()
        .flex_1()
        .gap_4()
        .child(ui::section_title(
            "Blocks",
            "Complete compositions assembled from live components.",
        ))
        .child(tabs)
        .child(variants)
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_2()
                .child(ui::button("Preview", ui::ButtonStyle::Outline, move || {
                    show_code.set(false)
                }))
                .child(ui::button("Code", ui::ButtonStyle::Outline, move || {
                    show_code.set(true)
                }))
                .child(view().flex_1())
                .child(devices),
        )
        .child(
            view()
                .w(px(DEVICES[device.get()].1))
                .p_6()
                .rounded(px(12.0))
                .border(px(1.0))
                .border_color(move || nsh_base::theme().border)
                .bg(move || nsh_base::theme().card)
                .child(when(
                    move || !show_code.get(),
                    move || preview(selected, variant),
                ))
                .child(when(
                    move || show_code.get(),
                    move || code_view(selected, variant),
                )),
        )
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_2()
                .child(
                    view()
                        .px_3()
                        .py_2()
                        .rounded(px(6.0))
                        .bg(move || nsh_base::theme().muted)
                        .child(text(command).text_size(px(12.0))),
                )
                .child(ui::button("Copy", ui::ButtonStyle::Outline, move || {
                    set_clipboard(command());
                }))
                .child(ui::button("Refresh", ui::ButtonStyle::Outline, move || {
                    variant.update(|value| *value = (*value % 5) + 1)
                }))
                .child(ui::button(
                    "Open in New Tab",
                    ui::ButtonStyle::Outline,
                    move || {
                        let target = match selected.get() {
                            0 => "ViewSidebar1",
                            2 => "ViewSignup1",
                            _ => "ViewLogin1",
                        };
                        route.set(target);
                        router::navigate(target);
                    },
                )),
        )
}

pub fn standalone(route: &'static str) -> impl IntoElement {
    let kind = route.starts_with("ViewSignup");
    let number = route
        .chars()
        .last()
        .and_then(|value| value.to_digit(10))
        .unwrap_or(1) as usize;
    view()
        .flex_1()
        .items_center()
        .justify_center()
        .py_8()
        .child(if kind {
            view().child(super::login::signup_variant(number))
        } else {
            view().child(super::login::login_variant(number))
        })
}

fn preview(selected: Signal<usize>, variant: Signal<usize>) -> impl IntoElement {
    let mut content = view();
    for category in 0..3 {
        for number in 1..=5 {
            content = content.child(when(
                move || selected.get() == category && variant.get() == number,
                move || {
                    if category == 2 {
                        view().child(super::login::signup_variant(number))
                    } else {
                        view().child(super::login::login_variant(number))
                    }
                },
            ));
        }
    }
    content
}

fn code_view(selected: Signal<usize>, variant: Signal<usize>) -> impl IntoElement {
    let slug = move || match selected.get() {
        0 => "sidebar",
        2 => "signup",
        _ => "login",
    };
    view()
        .gap_1()
        .child(
            text(move || format!("{}-0{}", slug(), variant.get()))
                .text_size(px(14.0))
                .font_weight(FontWeight::BOLD),
        )
        .child(
            text(move || format!("aurora add {}-0{}", slug(), variant.get())).text_size(px(12.0)),
        )
        .child(text("pub fn block() -> impl IntoElement {").text_size(px(12.0)))
        .child(
            text("    aurora_components::families::forms::input(value, \"Email\")".to_string())
                .text_size(px(12.0)),
        )
        .child(text("}").text_size(px(12.0)))
}
