use nsh_base::theme;
use nsh_core::prelude::*;

pub fn link(label: &'static str) -> impl IntoElement {
    view()
        .role(AccessibilityRole::Link)
        .label(label)
        .pb_1()
        .border_b_1()
        .border_color(move || theme().primary)
        .child(
            text(label)
                .text_size(px(14.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(move || theme().primary),
        )
}

pub fn menu(label: &'static str) -> impl IntoElement {
    view()
        .role(AccessibilityRole::Menu)
        .label(label)
        .min_w(px(180.0))
        .p_1()
        .rounded(px(8.0))
        .border(px(1.0))
        .border_color(move || theme().border)
        .bg(move || theme().popover)
        .gap_1()
        .child(
            view()
                .px_3()
                .py_2()
                .rounded(px(6.0))
                .bg(move || theme().accent)
                .child(
                    text(label)
                        .text_size(px(13.0))
                        .text_color(move || theme().accent_foreground),
                ),
        )
        .child(
            view().px_3().py_2().child(
                text("Open")
                    .text_size(px(13.0))
                    .text_color(move || theme().popover_foreground),
            ),
        )
}

pub fn nav_stack(label: &'static str) -> impl IntoElement {
    view()
        .role(AccessibilityRole::Navigation)
        .label(label)
        .p_3()
        .rounded(px(8.0))
        .bg(move || theme().muted)
        .gap_2()
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_2()
                .child(
                    view()
                        .size(px(4.0))
                        .rounded(px(999.0))
                        .bg(move || theme().primary),
                )
                .child(
                    text(label)
                        .text_size(px(14.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(move || theme().foreground),
                ),
        )
}

pub fn toolbar(label: &'static str) -> impl IntoElement {
    view()
        .role(AccessibilityRole::Toolbar)
        .label(label)
        .flex_row()
        .items_center()
        .gap_2()
        .p_2()
        .rounded(px(8.0))
        .border(px(1.0))
        .border_color(move || theme().border)
        .bg(move || theme().card)
        .child(
            text(label)
                .text_size(px(13.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(move || theme().card_foreground),
        )
}

pub fn popover(label: &'static str) -> impl IntoElement {
    view()
        .role(AccessibilityRole::Group)
        .label(label)
        .p_4()
        .rounded(px(10.0))
        .border(px(1.0))
        .border_color(move || theme().border)
        .bg(move || theme().popover)
        .gap_2()
        .child(
            text(label)
                .text_size(px(14.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(move || theme().popover_foreground),
        )
        .child(
            text("More details")
                .text_size(px(12.0))
                .text_color(move || theme().muted_foreground),
        )
}

pub fn sheet(label: &'static str) -> impl IntoElement {
    view()
        .role(AccessibilityRole::Region)
        .label(label)
        .w_full()
        .p_5()
        .rounded(px(16.0))
        .border(px(1.0))
        .border_color(move || theme().border)
        .bg(move || theme().card)
        .gap_3()
        .child(
            view()
                .w(px(40.0))
                .h(px(4.0))
                .rounded(px(999.0))
                .bg(move || theme().muted),
        )
        .child(
            text(label)
                .text_size(px(16.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(move || theme().card_foreground),
        )
}

pub fn toast(label: &'static str) -> impl IntoElement {
    view()
        .role(AccessibilityRole::Alert)
        .label(label)
        .flex_row()
        .items_center()
        .gap_3()
        .p_3()
        .rounded(px(8.0))
        .border(px(1.0))
        .border_color(move || theme().border)
        .bg(move || theme().popover)
        .child(
            view()
                .size(px(8.0))
                .rounded(px(999.0))
                .bg(move || theme().primary),
        )
        .child(
            text(label)
                .text_size(px(13.0))
                .text_color(move || theme().popover_foreground),
        )
}

pub fn alert_dialog(label: &'static str) -> impl IntoElement {
    view()
        .role(AccessibilityRole::Alert)
        .label(label)
        .p_6()
        .rounded(px(12.0))
        .border(px(1.0))
        .border_color(move || theme().destructive)
        .bg(move || theme().background)
        .gap_3()
        .child(
            text(label)
                .text_size(px(18.0))
                .font_weight(FontWeight::BOLD)
                .text_color(move || theme().foreground),
        )
        .child(
            text("This action cannot be undone.")
                .text_size(px(13.0))
                .text_color(move || theme().muted_foreground),
        )
}

pub fn hover_card(label: &'static str) -> impl IntoElement {
    view()
        .role(AccessibilityRole::Group)
        .label(label)
        .p_4()
        .rounded(px(10.0))
        .border(px(1.0))
        .border_color(move || theme().border)
        .bg(move || theme().popover)
        .gap_2()
        .child(
            text(label)
                .text_size(px(14.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(move || theme().popover_foreground),
        )
        .child(
            text("Preview content")
                .text_size(px(12.0))
                .text_color(move || theme().muted_foreground),
        )
}

pub fn context_menu(label: &'static str) -> impl IntoElement {
    view()
        .role(AccessibilityRole::Menu)
        .label(label)
        .min_w(px(160.0))
        .p_1()
        .rounded(px(8.0))
        .border(px(1.0))
        .border_color(move || theme().border)
        .bg(move || theme().popover)
        .gap_1()
        .child(
            view()
                .role(AccessibilityRole::Menuitem)
                .px_3()
                .py_2()
                .rounded(px(6.0))
                .bg(move || theme().accent)
                .child(
                    text(label)
                        .text_size(px(13.0))
                        .text_color(move || theme().accent_foreground),
                ),
        )
        .child(
            view()
                .role(AccessibilityRole::Menuitem)
                .px_3()
                .py_2()
                .child(
                    text("Delete")
                        .text_size(px(13.0))
                        .text_color(move || theme().destructive),
                ),
        )
}
