use nsh_core::prelude::*;

pub fn number_input(label: &'static str) -> impl IntoElement {
    let value = signal(String::new());
    view()
        .role(AccessibilityRole::Spinbutton)
        .gap_2()
        .child(
            text(label)
                .text_size(px(13.0))
                .font_weight(FontWeight::MEDIUM),
        )
        .child(
            nsh_base::input()
                .value(value)
                .on_change(move |text: &str| value.set(text.to_string()))
                .placeholder("0")
                .p_3()
                .border(px(1.0))
                .border_color(move || nsh_base::theme().input)
                .rounded(px(8.0)),
        )
}

pub fn otp_input(label: &'static str) -> impl IntoElement {
    let value = signal(String::new());
    let mut cells = view().flex_row().items_center().gap_2();
    for _ in 0..4 {
        cells = cells.child(
            nsh_base::input()
                .value(value)
                .on_change(move |text: &str| value.set(text.to_string()))
                .role(AccessibilityRole::Text)
                .w(px(44.0))
                .p_2()
                .text_center()
                .border(px(1.0))
                .border_color(move || nsh_base::theme().border)
                .rounded(px(8.0))
                .bg(move || nsh_base::theme().background),
        );
    }
    view()
        .gap_2()
        .child(
            text(label)
                .text_size(px(13.0))
                .font_weight(FontWeight::MEDIUM),
        )
        .child(cells)
}

pub fn select(options: &'static [&'static str]) -> impl IntoElement {
    let selected = signal(0_usize);
    let mut rows = view().flex_col().gap_1();
    for (index, option) in options.iter().enumerate() {
        let title = *option;
        rows = rows.child(
            view()
                .role(AccessibilityRole::Option)
                .px_3()
                .py_2()
                .rounded(px(6.0))
                .bg(move || {
                    if selected.get() == index {
                        nsh_base::theme().muted
                    } else {
                        nsh_base::theme().background
                    }
                })
                .on_press(move || selected.set(index))
                .child(text(title).text_size(px(13.0))),
        );
    }
    view()
        .role(AccessibilityRole::Combobox)
        .gap_2()
        .child(
            view()
                .px_3()
                .py_2()
                .rounded(px(8.0))
                .border(px(1.0))
                .border_color(move || nsh_base::theme().border)
                .flex_row()
                .items_center()
                .justify_between()
                .child(text(move || {
                    options.get(selected.get()).copied().unwrap_or("")
                }))
                .child(
                    text("-")
                        .text_size(px(12.0))
                        .text_color(move || nsh_base::theme().muted_foreground),
                ),
        )
        .child(rows)
}

pub fn slider(label: &'static str) -> impl IntoElement {
    let value = signal(50.0_f64);
    view()
        .gap_2()
        .child(
            text(label)
                .text_size(px(13.0))
                .font_weight(FontWeight::MEDIUM),
        )
        .child(
            view()
                .role(AccessibilityRole::Adjustable)
                .flex_row()
                .items_center()
                .gap_3()
                .child(
                    view()
                        .w(px(160.0))
                        .h(px(6.0))
                        .rounded(px(999.0))
                        .bg(move || nsh_base::theme().muted)
                        .on_press(move || value.update(|v| *v = (*v + 10.0) % 101.0))
                        .child(
                            view()
                                .h_full()
                                .w(px(value.get() as f32 / 100.0 * 160.0))
                                .rounded(px(999.0))
                                .bg(move || nsh_base::theme().primary),
                        ),
                )
                .child(
                    text(move || format!("{:.0}%", value.get()))
                        .text_size(px(12.0))
                        .text_color(move || nsh_base::theme().muted_foreground),
                ),
        )
}

pub fn time_field(label: &'static str) -> impl IntoElement {
    let value = signal(String::new());
    view()
        .role(AccessibilityRole::Adjustable)
        .gap_2()
        .child(
            text(label)
                .text_size(px(13.0))
                .font_weight(FontWeight::MEDIUM),
        )
        .child(
            nsh_base::input()
                .value(value)
                .on_change(move |text: &str| value.set(text.to_string()))
                .placeholder("HH:MM")
                .p_3()
                .border(px(1.0))
                .border_color(move || nsh_base::theme().input)
                .rounded(px(8.0)),
        )
}

pub fn date_picker(label: &'static str) -> impl IntoElement {
    let value = signal(String::new());
    view()
        .role(AccessibilityRole::Combobox)
        .gap_2()
        .child(
            text(label)
                .text_size(px(13.0))
                .font_weight(FontWeight::MEDIUM),
        )
        .child(
            nsh_base::input()
                .value(value)
                .on_change(move |text: &str| value.set(text.to_string()))
                .placeholder("YYYY-MM-DD")
                .p_3()
                .border(px(1.0))
                .border_color(move || nsh_base::theme().input)
                .rounded(px(8.0)),
        )
}

pub fn color_picker(label: &'static str) -> impl IntoElement {
    view()
        .gap_2()
        .child(
            text(label)
                .text_size(px(13.0))
                .font_weight(FontWeight::MEDIUM),
        )
        .child(
            view()
                .role(AccessibilityRole::Button)
                .flex_row()
                .items_center()
                .gap_3()
                .p_2()
                .rounded(px(8.0))
                .border(px(1.0))
                .border_color(move || nsh_base::theme().border)
                .child(
                    view()
                        .size(px(22.0))
                        .rounded(px(6.0))
                        .border(px(1.0))
                        .border_color(move || nsh_base::theme().border)
                        .bg(move || nsh_base::theme().primary),
                )
                .child(
                    text(label)
                        .text_size(px(12.0))
                        .text_color(move || nsh_base::theme().muted_foreground),
                ),
        )
}

pub fn calendar(label: &'static str) -> impl IntoElement {
    let weekdays = ["S", "M", "T", "W", "T", "F", "S"];
    let mut head = view().flex_row();
    for day in weekdays {
        head = head.child(
            view().w_full().items_center().child(
                text(day)
                    .text_size(px(11.0))
                    .text_color(move || nsh_base::theme().muted_foreground),
            ),
        );
    }
    let mut body = view().flex_col().gap_1();
    for week in 0..3_u32 {
        let mut row = view().flex_row();
        for day in 0..7_u32 {
            let number = week * 7 + day + 1;
            row = row.child(
                view().w_full().items_center().child(
                    text(move || number.to_string())
                        .text_size(px(12.0))
                        .text_color(move || nsh_base::theme().foreground),
                ),
            );
        }
        body = body.child(row);
    }
    view()
        .role(AccessibilityRole::Grid)
        .gap_3()
        .p_3()
        .rounded(px(10.0))
        .border(px(1.0))
        .border_color(move || nsh_base::theme().border)
        .bg(move || nsh_base::theme().popover)
        .child(
            text(label)
                .text_size(px(14.0))
                .font_weight(FontWeight::SEMIBOLD),
        )
        .child(head)
        .child(body)
}

pub fn combobox(options: &'static [&'static str]) -> impl IntoElement {
    let selected = signal(0_usize);
    let mut rows = view().flex_col().gap_1();
    for (index, option) in options.iter().enumerate() {
        let title = *option;
        rows = rows.child(
            view()
                .role(AccessibilityRole::Option)
                .px_3()
                .py_2()
                .rounded(px(6.0))
                .bg(move || {
                    if selected.get() == index {
                        nsh_base::theme().accent
                    } else {
                        nsh_base::theme().popover
                    }
                })
                .on_press(move || selected.set(index))
                .child(text(title).text_size(px(13.0))),
        );
    }
    view()
        .role(AccessibilityRole::Combobox)
        .gap_2()
        .child(
            view()
                .px_3()
                .py_2()
                .rounded(px(8.0))
                .border(px(1.0))
                .border_color(move || nsh_base::theme().input)
                .bg(move || nsh_base::theme().background)
                .child(text(move || {
                    options.get(selected.get()).copied().unwrap_or("")
                })),
        )
        .child(rows)
}

pub fn keyboard(label: &'static str) -> impl IntoElement {
    let keys = ["Ctrl", "Shift", "K"];
    let mut caps = view().flex_row().items_center().gap_1();
    for key in keys {
        caps = caps.child(
            view()
                .role(AccessibilityRole::KeyboardKey)
                .px_2()
                .py_1()
                .rounded(px(4.0))
                .border(px(1.0))
                .border_color(move || nsh_base::theme().border)
                .bg(move || nsh_base::theme().muted)
                .child(
                    text(key)
                        .text_size(px(11.0))
                        .text_color(move || nsh_base::theme().muted_foreground),
                ),
        );
    }
    view()
        .gap_2()
        .child(
            text(label)
                .text_size(px(13.0))
                .font_weight(FontWeight::MEDIUM),
        )
        .child(caps)
}

pub fn text_selection(label: &'static str) -> impl IntoElement {
    view()
        .role(AccessibilityRole::Text)
        .gap_2()
        .child(
            text(label)
                .text_size(px(13.0))
                .font_weight(FontWeight::MEDIUM),
        )
        .child(
            view()
                .p_3()
                .rounded(px(8.0))
                .border(px(1.0))
                .border_color(move || nsh_base::theme().border)
                .bg(move || nsh_base::theme().background)
                .child(
                    text("Selectable content")
                        .text_size(px(13.0))
                        .text_color(move || nsh_base::theme().foreground),
                )
                .child(
                    view()
                        .bg(move || nsh_base::theme().primary.opacity(0.22))
                        .child(
                            text(" selected span")
                                .text_size(px(13.0))
                                .text_color(move || nsh_base::theme().foreground),
                        ),
                ),
        )
}
