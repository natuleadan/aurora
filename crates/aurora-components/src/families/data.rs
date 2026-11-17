use nsh_core::prelude::*;

pub fn data_table(rows: &'static [(&'static str, &'static str)]) -> impl IntoElement {
    let mut body = view()
        .role(AccessibilityRole::Table)
        .p_3()
        .gap_1()
        .rounded(px(10.0))
        .border(px(1.0))
        .border_color(move || nsh_base::theme().border);
    for (left, right) in rows {
        body = body.child(
            view()
                .role(AccessibilityRole::Row)
                .flex_row()
                .items_center()
                .justify_between()
                .py_2()
                .border_b_1()
                .border_color(move || nsh_base::theme().border)
                .child(
                    view()
                        .role(AccessibilityRole::Cell)
                        .child(text(*left).text_size(px(13.0))),
                )
                .child(
                    view().role(AccessibilityRole::Cell).child(
                        text(*right)
                            .text_size(px(13.0))
                            .text_color(move || nsh_base::theme().muted_foreground),
                    ),
                ),
        );
    }
    body
}

pub fn plot(label: &'static str) -> impl IntoElement {
    view()
        .role(AccessibilityRole::Image)
        .label(label)
        .p_4()
        .rounded(px(10.0))
        .border(px(1.0))
        .border_color(move || nsh_base::theme().border)
        .gap_3()
        .child(
            text(label)
                .text_size(px(14.0))
                .font_weight(FontWeight::SEMIBOLD),
        )
        .child(
            view()
                .flex_row()
                .items_end()
                .justify_between()
                .gap_2()
                .h(px(140.0))
                .child(
                    view()
                        .w(px(28.0))
                        .h(px(54.0))
                        .rounded(px(4.0))
                        .bg(move || nsh_base::theme().chart_1),
                )
                .child(
                    view()
                        .w(px(28.0))
                        .h(px(92.0))
                        .rounded(px(4.0))
                        .bg(move || nsh_base::theme().chart_2),
                )
                .child(
                    view()
                        .w(px(28.0))
                        .h(px(70.0))
                        .rounded(px(4.0))
                        .bg(move || nsh_base::theme().chart_3),
                )
                .child(
                    view()
                        .w(px(28.0))
                        .h(px(118.0))
                        .rounded(px(4.0))
                        .bg(move || nsh_base::theme().chart_4),
                )
                .child(
                    view()
                        .w(px(28.0))
                        .h(px(84.0))
                        .rounded(px(4.0))
                        .bg(move || nsh_base::theme().chart_5),
                ),
        )
        .child(
            text("Chart preview")
                .text_size(px(12.0))
                .text_color(move || nsh_base::theme().muted_foreground),
        )
}

pub fn tree(items: &'static [&'static str]) -> impl IntoElement {
    let mut body = view()
        .role(AccessibilityRole::Tree)
        .p_3()
        .gap_1()
        .rounded(px(10.0))
        .border(px(1.0))
        .border_color(move || nsh_base::theme().border);
    for item in items {
        body = body.child(
            view()
                .role(AccessibilityRole::TreeItem)
                .flex_row()
                .items_center()
                .gap_2()
                .px_2()
                .py_1()
                .rounded(px(6.0))
                .child(
                    text("-")
                        .text_size(px(12.0))
                        .text_color(move || nsh_base::theme().muted_foreground),
                )
                .child(text(*item).text_size(px(13.0))),
        );
    }
    body
}

pub fn diff_view(label: &'static str) -> impl IntoElement {
    view()
        .role(AccessibilityRole::Region)
        .label(label)
        .p_4()
        .rounded(px(10.0))
        .border(px(1.0))
        .border_color(move || nsh_base::theme().border)
        .gap_2()
        .child(
            text(label)
                .text_size(px(14.0))
                .font_weight(FontWeight::SEMIBOLD),
        )
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_2()
                .px_3()
                .py_2()
                .rounded(px(6.0))
                .bg(move || nsh_base::theme().muted)
                .child(
                    text("-")
                        .text_size(px(13.0))
                        .text_color(move || nsh_base::theme().destructive),
                )
                .child(
                    text("removed line")
                        .text_size(px(12.0))
                        .text_color(move || nsh_base::theme().muted_foreground),
                ),
        )
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_2()
                .px_3()
                .py_2()
                .rounded(px(6.0))
                .bg(move || nsh_base::theme().muted)
                .child(
                    text("+")
                        .text_size(px(13.0))
                        .text_color(move || nsh_base::theme().primary),
                )
                .child(
                    text("added line")
                        .text_size(px(12.0))
                        .text_color(move || nsh_base::theme().muted_foreground),
                ),
        )
}

pub fn rich_editor(label: &'static str) -> impl IntoElement {
    view()
        .role(AccessibilityRole::Group)
        .label(label)
        .rounded(px(10.0))
        .border(px(1.0))
        .border_color(move || nsh_base::theme().border)
        .gap_2()
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_1()
                .px_3()
                .py_2()
                .border_b_1()
                .border_color(move || nsh_base::theme().border)
                .child(
                    view()
                        .px_2()
                        .py_1()
                        .rounded(px(4.0))
                        .bg(move || nsh_base::theme().muted)
                        .child(text("B").text_size(px(12.0))),
                )
                .child(
                    view()
                        .px_2()
                        .py_1()
                        .rounded(px(4.0))
                        .bg(move || nsh_base::theme().muted)
                        .child(text("I").text_size(px(12.0))),
                )
                .child(
                    view()
                        .px_2()
                        .py_1()
                        .rounded(px(4.0))
                        .bg(move || nsh_base::theme().muted)
                        .child(text("U").text_size(px(12.0))),
                ),
        )
        .child(
            view().p_4().min_h(px(120.0)).child(
                text(label)
                    .text_size(px(13.0))
                    .text_color(move || nsh_base::theme().muted_foreground),
            ),
        )
}

pub fn terminal(label: &'static str) -> impl IntoElement {
    view()
        .role(AccessibilityRole::Region)
        .label(label)
        .rounded(px(10.0))
        .bg(move || nsh_base::theme().background)
        .border(px(1.0))
        .border_color(move || nsh_base::theme().border)
        .gap_1()
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_2()
                .px_3()
                .py_2()
                .border_b_1()
                .border_color(move || nsh_base::theme().border)
                .child(
                    view()
                        .size(px(8.0))
                        .rounded(px(999.0))
                        .bg(move || nsh_base::theme().destructive),
                )
                .child(
                    view()
                        .size(px(8.0))
                        .rounded(px(999.0))
                        .bg(move || nsh_base::theme().secondary),
                )
                .child(
                    view()
                        .size(px(8.0))
                        .rounded(px(999.0))
                        .bg(move || nsh_base::theme().muted),
                )
                .child(
                    text(label)
                        .text_size(px(12.0))
                        .text_color(move || nsh_base::theme().muted_foreground),
                ),
        )
        .child(
            view()
                .p_4()
                .min_h(px(140.0))
                .gap_1()
                .child(text("$ command").text_size(px(13.0)))
                .child(
                    text("output")
                        .text_size(px(13.0))
                        .text_color(move || nsh_base::theme().muted_foreground),
                ),
        )
}

pub fn text_view(label: &'static str) -> impl IntoElement {
    view()
        .role(AccessibilityRole::Text)
        .label(label)
        .p_4()
        .rounded(px(10.0))
        .bg(move || nsh_base::theme().card)
        .border(px(1.0))
        .border_color(move || nsh_base::theme().border)
        .gap_2()
        .child(
            text(label)
                .text_size(px(14.0))
                .font_weight(FontWeight::SEMIBOLD),
        )
        .child(
            text("Line one of the document.")
                .text_size(px(13.0))
                .text_color(move || nsh_base::theme().muted_foreground),
        )
        .child(
            text("Line two of the document.")
                .text_size(px(13.0))
                .text_color(move || nsh_base::theme().muted_foreground),
        )
}

pub fn resizable(label: &'static str) -> impl IntoElement {
    view()
        .role(AccessibilityRole::Group)
        .label(label)
        .flex_row()
        .min_h(px(160.0))
        .rounded(px(10.0))
        .border(px(1.0))
        .border_color(move || nsh_base::theme().border)
        .child(
            view()
                .flex_1()
                .p_4()
                .bg(move || nsh_base::theme().muted)
                .child(text(label).text_size(px(13.0))),
        )
        .child(view().w(px(4.0)).bg(move || nsh_base::theme().border))
        .child(
            view()
                .flex_1()
                .p_4()
                .bg(move || nsh_base::theme().card)
                .child(text("Panel").text_size(px(13.0))),
        )
}

pub fn dock(label: &'static str) -> impl IntoElement {
    view()
        .role(AccessibilityRole::Toolbar)
        .label(label)
        .p_2()
        .rounded(px(10.0))
        .bg(move || nsh_base::theme().sidebar)
        .border(px(1.0))
        .border_color(move || nsh_base::theme().sidebar_border)
        .flex_row()
        .items_center()
        .gap_2()
        .child(
            view()
                .flex_1()
                .px_3()
                .py_2()
                .rounded(px(6.0))
                .bg(move || nsh_base::theme().sidebar_accent)
                .child(
                    text(label)
                        .text_size(px(13.0))
                        .text_color(move || nsh_base::theme().sidebar_accent_foreground),
                ),
        )
        .child(
            view()
                .size(px(32.0))
                .rounded(px(6.0))
                .bg(move || nsh_base::theme().sidebar_primary)
                .items_center()
                .justify_center()
                .child(
                    text("+")
                        .text_size(px(14.0))
                        .text_color(move || nsh_base::theme().sidebar_primary_foreground),
                ),
        )
}

pub fn scrollbar() -> impl IntoElement {
    view()
        .role(AccessibilityRole::Scrollbar)
        .w(px(8.0))
        .h(px(160.0))
        .rounded(px(999.0))
        .bg(move || nsh_base::theme().muted)
        .child(
            view()
                .w_full()
                .h(px(60.0))
                .rounded(px(999.0))
                .bg(move || nsh_base::theme().border),
        )
}

pub fn safe_area() -> impl IntoElement {
    view()
        .role(AccessibilityRole::Group)
        .p_safe_area()
        .w_full()
        .child(
            text("Safe area")
                .text_size(px(13.0))
                .text_color(move || nsh_base::theme().muted_foreground),
        )
}
