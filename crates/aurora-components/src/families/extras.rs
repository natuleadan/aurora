use nsh_core::prelude::*;

pub fn button_group(labels: &'static [&'static str]) -> impl IntoElement {
    let selected = signal(0_usize);
    let mut row = view().flex_row().items_center().gap_1();
    for (index, label) in labels.iter().enumerate() {
        row = row.child(
            view()
                .px_3()
                .py_2()
                .rounded(px(6.0))
                .bg(move || {
                    if selected.get() == index {
                        nsh_base::theme().primary
                    } else {
                        nsh_base::theme().muted
                    }
                })
                .on_press(move || selected.set(index))
                .child(text(*label).text_size(px(13.0))),
        );
    }
    row
}

pub fn toggle_group(items: &'static [&'static str]) -> impl IntoElement {
    let mut row = view().flex_row().items_center().gap_2();
    for item in items {
        row = row.child(super::overlays::toolbar(item));
    }
    row
}

pub fn textarea(label: &'static str) -> impl IntoElement {
    view()
        .role(AccessibilityRole::Text)
        .label(label)
        .gap_1()
        .child(text(label).text_size(px(12.0)))
        .child(
            view()
                .h(px(100.0))
                .p_3()
                .rounded(px(8.0))
                .border(px(1.0))
                .border_color(move || nsh_base::theme().border)
                .child(text("Type your message").text_size(px(13.0))),
        )
}

pub fn chart(label: &'static str) -> impl IntoElement {
    super::data::plot(label)
}

pub fn carousel(slides: &'static [&'static str]) -> impl IntoElement {
    let current = signal(0_usize);
    let mut dots = view().flex_row().items_center().justify_center().gap_2();
    for index in 0..slides.len() {
        dots = dots.child(
            view()
                .size(px(8.0))
                .rounded(px(999.0))
                .bg(move || {
                    if current.get() == index {
                        nsh_base::theme().primary
                    } else {
                        nsh_base::theme().muted
                    }
                })
                .on_press(move || current.set(index)),
        );
    }
    view()
        .role(AccessibilityRole::Group)
        .label("Carousel")
        .gap_3()
        .child(
            view()
                .h(px(160.0))
                .items_center()
                .justify_center()
                .rounded(px(10.0))
                .border(px(1.0))
                .border_color(move || nsh_base::theme().border)
                .child(
                    text(move || slides[current.get() % slides.len()])
                        .text_size(px(16.0))
                        .font_weight(FontWeight::SEMIBOLD),
                ),
        )
        .child(dots)
}

pub fn command(items: &'static [&'static str]) -> impl IntoElement {
    let query = signal(String::new());
    let mut rows = view().gap_1();
    for item in items {
        rows = rows.child(when(
            move || {
                let needle = query.get().to_lowercase();
                needle.is_empty() || item.to_lowercase().contains(&needle)
            },
            move || {
                view()
                    .px_3()
                    .py_2()
                    .rounded(px(6.0))
                    .bg(move || nsh_base::theme().muted)
                    .child(text(*item).text_size(px(13.0)))
            },
        ));
    }
    view()
        .role(AccessibilityRole::Group)
        .label("Command palette")
        .gap_2()
        .child(
            nsh_base::input()
                .value(query)
                .on_change(move |value: &str| query.set(value.to_string()))
                .placeholder("Search commands")
                .p_3()
                .border_1()
                .border_color(move || nsh_base::theme().border)
                .rounded(px(8.0)),
        )
        .child(rows)
}

pub fn editor(label: &'static str) -> impl IntoElement {
    view()
        .role(AccessibilityRole::Text)
        .label(label)
        .p_4()
        .rounded(px(8.0))
        .border(px(1.0))
        .border_color(move || nsh_base::theme().border)
        .bg(move || nsh_base::theme().background)
        .child(
            text(label)
                .text_size(px(13.0))
                .text_color(move || nsh_base::theme().muted_foreground),
        )
}

pub fn emoji_picker(label: &'static str) -> impl IntoElement {
    let emojis = ["smile", "heart", "star", "check"];
    let chosen = signal(0_usize);
    let mut row = view().flex_row().items_center().gap_2();
    for (index, emoji) in emojis.iter().enumerate() {
        row = row.child(
            view()
                .px_3()
                .py_2()
                .rounded(px(6.0))
                .bg(move || {
                    if chosen.get() == index {
                        nsh_base::theme().muted
                    } else {
                        nsh_base::theme().background
                    }
                })
                .on_press(move || chosen.set(index))
                .child(text(*emoji).text_size(px(13.0))),
        );
    }
    view()
        .role(AccessibilityRole::Group)
        .label(label)
        .gap_2()
        .child(text(label).text_size(px(12.0)))
        .child(row)
}
