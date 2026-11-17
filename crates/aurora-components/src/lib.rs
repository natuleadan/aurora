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

pub fn radio_group(options: &'static [(&'static str, &'static str)]) -> impl IntoElement {
    let value = signal(options[0].0);
    let mut group = nsh_base::radio_group().flex_col().gap_2();
    for option in options {
        let (id, label) = *option;
        group = group.child(
            nsh_base::radio()
                .checked(move || value.get() == id)
                .on_change(move |_: &bool| value.set(id))
                .flex_row()
                .items_center()
                .gap_2()
                .child(
                    view()
                        .size(px(18.0))
                        .items_center()
                        .justify_center()
                        .rounded(px(999.0))
                        .border_2()
                        .border_color(move || nsh_base::theme().border)
                        .checked(|s| s.border_color(nsh_base::theme().primary)),
                )
                .child(text(label).text_size(px(13.0))),
        );
    }
    group
}

pub fn dialog_trigger(
    label: &'static str,
    title: &'static str,
    body: &'static str,
) -> impl IntoElement {
    let open = signal(false);
    view()
        .gap_3()
        .child(button(label, move || open.set(true)))
        .child(when(open, move || {
            nsh_base::dialog()
                .on_cancel(move || open.set(false))
                .p_6()
                .gap_3()
                .bg(move || nsh_base::theme().background)
                .child(
                    text(title)
                        .text_size(px(18.0))
                        .font_weight(FontWeight::BOLD),
                )
                .child(text(body).text_size(px(13.0)))
                .child(button("Close", move || open.set(false)))
        }))
}

pub fn hint(content: &'static str, message: &'static str) -> impl IntoElement {
    view()
        .tooltip(
            nsh_base::tooltip()
                .p_2()
                .rounded(px(6.0))
                .bg(move || nsh_base::theme().popover)
                .child(text(message).text_size(px(12.0))),
        )
        .child(text(content).text_size(px(13.0)))
}

pub fn tabs(options: &'static [(&'static str, &'static str, &'static str)]) -> impl IntoElement {
    let current = signal(options[0].0);
    let mut list = nsh_base::tabs()
        .flex_row()
        .self_start()
        .gap_1()
        .p_1()
        .rounded(px(8.0))
        .bg(move || nsh_base::theme().muted);
    for option in options {
        let (value, label, _) = *option;
        let selected = move || current.get() == value;
        list = list.child(
            nsh_base::tab()
                .selected(selected)
                .on_press(move |_: &()| current.set(value))
                .px_3()
                .py_2()
                .rounded(px(6.0))
                .child(text(label).text_size(px(13.0))),
        );
    }
    let mut content = view().gap_2();
    for option in options {
        let (value, _, body) = *option;
        content = content.child(when(
            move || current.get() == value,
            move || text(body).text_size(px(13.0)),
        ));
    }
    view().gap_3().child(list).child(content)
}

pub fn accordion(items: &'static [(&'static str, &'static str, &'static str)]) -> impl IntoElement {
    let open = signal(vec![SharedString::from(items[0].0)]);
    let mut list = nsh_base::accordion()
        .value(open)
        .on_change(move |value: &Vec<SharedString>| open.set(value.clone()));
    for item in items {
        let (value, question, answer) = *item;
        list = list.child(
            nsh_base::accordion_item()
                .value(value)
                .border_b_1()
                .border_color(move || nsh_base::theme().border)
                .child(
                    nsh_base::accordion_header().child(
                        nsh_base::accordion_trigger()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .py_3()
                            .child(text(question).font_weight(FontWeight::MEDIUM)),
                    ),
                )
                .child(
                    nsh_base::accordion_panel()
                        .accessibility_label(question)
                        .child(view().pb_3().child(text(answer))),
                ),
        );
    }
    list
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
