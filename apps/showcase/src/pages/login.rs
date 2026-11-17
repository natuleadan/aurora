use nsh_core::prelude::*;

use crate::ui;

pub fn login() -> impl IntoElement {
    let email = signal(String::new());
    let password = signal(String::new());

    view()
        .flex_1()
        .items_center()
        .justify_center()
        .py_8()
        .child(
            view()
                .w(px(380.0))
                .p_6()
                .gap_4()
                .rounded(px(12.0))
                .border(px(1.0))
                .border_color(move || nsh_base::theme().border)
                .bg(move || nsh_base::theme().card)
                .child(
                    view()
                        .gap_1()
                        .child(
                            text("Welcome back")
                                .text_size(px(22.0))
                                .font_weight(FontWeight::BOLD),
                        )
                        .child(
                            text("Sign in to continue to your workspace.")
                                .text_size(px(13.0))
                                .text_color(move || nsh_base::theme().muted_foreground),
                        ),
                )
                .child(field("Email", email, "Type your email"))
                .child(field("Password", password, "Your password"))
                .child(aurora_components::checkbox("Remember me"))
                .child(aurora_components::button("Sign in", || {}))
                .child(ui::chip("New here? Create an account")),
        )
}

fn field(
    label: &'static str,
    value: Signal<String>,
    placeholder: &'static str,
) -> impl IntoElement {
    view()
        .gap_1()
        .child(aurora_components::label(label))
        .child(aurora_components::input(value, placeholder))
}
