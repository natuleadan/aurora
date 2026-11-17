use nsh_core::prelude::*;

use crate::ui;

static PROVIDER: &[&str] = &["github"];

pub fn login_variant(index: usize) -> impl IntoElement {
    match index {
        1 => view().child(card_login()),
        2 => view().child(plain_login()),
        3 => view().child(social_card_login()),
        4 => view().child(split_login()),
        _ => view().child(compact_login()),
    }
}

pub fn signup_variant(index: usize) -> impl IntoElement {
    match index {
        1 => view().child(card_signup()),
        2 => view().child(plain_signup()),
        3 => view().child(social_card_signup()),
        4 => view().child(split_signup()),
        _ => view().child(compact_signup()),
    }
}

fn email_field(placeholder: &'static str) -> impl IntoElement {
    let value = signal(String::new());
    field("Email", aurora_components::input(value, placeholder))
}

fn password_field(with_forgot: bool) -> impl IntoElement {
    let value = signal(String::new());
    let label = if with_forgot {
        view().child(
            view()
                .flex_row()
                .items_center()
                .justify_between()
                .child(aurora_components::label("Password"))
                .child(text("Forgot your password?").text_size(px(11.0))),
        )
    } else {
        view().child(aurora_components::label("Password"))
    };
    view()
        .gap_1()
        .child(label)
        .child(aurora_components::input(value, "Password"))
}

fn name_field() -> impl IntoElement {
    let value = signal(String::new());
    field("Name", aurora_components::input(value, "John Doe"))
}

fn field(label: &'static str, control: impl IntoElement) -> impl IntoElement {
    view()
        .gap_1()
        .child(aurora_components::label(label))
        .child(control)
}

fn separator(label: &'static str) -> impl IntoElement {
    view()
        .flex_row()
        .items_center()
        .gap_2()
        .child(
            view()
                .h(px(1.0))
                .flex_1()
                .bg(move || nsh_base::theme().border),
        )
        .child(text(label).text_size(px(11.0)))
        .child(
            view()
                .h(px(1.0))
                .flex_1()
                .bg(move || nsh_base::theme().border),
        )
}

fn socials(kinds: &'static [&'static str]) -> impl IntoElement {
    let mut row = view().flex_row().items_center().gap_2();
    for kind in kinds {
        row = row.child(aurora_components::button(kind, || {}));
    }
    row
}

fn card_login() -> impl IntoElement {
    view()
        .w(px(360.0))
        .p_5()
        .gap_3()
        .rounded(px(10.0))
        .border(px(1.0))
        .border_color(move || nsh_base::theme().border)
        .bg(move || nsh_base::theme().card)
        .child(title_block(
            "Login to your account",
            "Enter your email below to login to your account",
        ))
        .child(email_field("name@example"))
        .child(password_field(true))
        .child(actions(
            &["Login", "Login with Google"],
            "Don't have an account? Sign up",
        ))
}

fn plain_login() -> impl IntoElement {
    view()
        .w(px(360.0))
        .gap_3()
        .child(title_block(
            "Login to your account",
            "Enter your email below to login to your account",
        ))
        .child(email_field("name@example"))
        .child(password_field(true))
        .child(aurora_components::button("Login", || {}))
        .child(separator("Or continue with"))
        .child(socials(PROVIDER))
        .child(text("Don't have an account? Sign up").text_size(px(11.0)))
}

fn social_card_login() -> impl IntoElement {
    view()
        .w(px(360.0))
        .p_5()
        .gap_3()
        .rounded(px(10.0))
        .border(px(1.0))
        .border_color(move || nsh_base::theme().border)
        .bg(move || nsh_base::theme().card)
        .child(title_block(
            "Welcome back",
            "Login with your Apple or Google account",
        ))
        .child(socials(&["Apple", "Google"]))
        .child(separator("Or continue with"))
        .child(email_field("name@example"))
        .child(password_field(false))
        .child(aurora_components::button("Login", || {}))
}

fn split_login() -> impl IntoElement {
    view()
        .w(px(680.0))
        .flex_row()
        .rounded(px(10.0))
        .border(px(1.0))
        .border_color(move || nsh_base::theme().border)
        .bg(move || nsh_base::theme().card)
        .child(
            view()
                .flex_1()
                .p_5()
                .gap_3()
                .child(title_block(
                    "Welcome back",
                    "Login to your Acme Inc account",
                ))
                .child(email_field("name@example"))
                .child(password_field(true))
                .child(aurora_components::button("Login", || {}))
                .child(separator("Or continue with"))
                .child(socials(&["Apple", "Google", "GitHub"])),
        )
        .child(
            view()
                .w(px(220.0))
                .bg(move || nsh_base::theme().muted)
                .items_center()
                .justify_center()
                .child(text("Acme Inc").text_size(px(14.0))),
        )
}

fn compact_login() -> impl IntoElement {
    view()
        .w(px(360.0))
        .gap_3()
        .items_center()
        .child(
            text("Welcome to Acme Inc.")
                .text_size(px(18.0))
                .font_weight(FontWeight::BOLD),
        )
        .child(text("Don't have an account? Sign up").text_size(px(11.0)))
        .child(email_field("name@example"))
        .child(aurora_components::button("Login", || {}))
        .child(separator("Or"))
        .child(socials(&["Apple", "Google"]))
}

fn title_block(title: &'static str, subtitle: &'static str) -> impl IntoElement {
    view()
        .gap_1()
        .child(
            text(title)
                .text_size(px(18.0))
                .font_weight(FontWeight::BOLD),
        )
        .child(
            text(subtitle)
                .text_size(px(11.0))
                .text_color(move || nsh_base::theme().muted_foreground),
        )
}

fn actions(labels: &'static [&'static str], footer: &'static str) -> impl IntoElement {
    let mut col = view().gap_2();
    for label in labels {
        col = col.child(aurora_components::button(label, || {}));
    }
    col.child(
        text(footer)
            .text_size(px(11.0))
            .text_color(move || nsh_base::theme().muted_foreground),
    )
}

fn card_signup() -> impl IntoElement {
    view()
        .w(px(360.0))
        .p_5()
        .gap_3()
        .rounded(px(10.0))
        .border(px(1.0))
        .border_color(move || nsh_base::theme().border)
        .bg(move || nsh_base::theme().card)
        .child(title_block(
            "Create an account",
            "Enter your details below to create your account",
        ))
        .child(name_field())
        .child(email_field("name@example"))
        .child(password_field(false))
        .child(aurora_components::button("Create Account", || {}))
        .child(aurora_components::button("Sign up with Google", || {}))
}

fn plain_signup() -> impl IntoElement {
    view()
        .w(px(360.0))
        .gap_3()
        .child(title_block(
            "Create your account",
            "Enter your details below to create your account",
        ))
        .child(name_field())
        .child(email_field("name@example"))
        .child(password_field(false))
        .child(aurora_components::button("Create Account", || {}))
        .child(separator("Or continue with"))
        .child(socials(PROVIDER))
}

fn social_card_signup() -> impl IntoElement {
    view()
        .w(px(360.0))
        .p_5()
        .gap_3()
        .rounded(px(10.0))
        .border(px(1.0))
        .border_color(move || nsh_base::theme().border)
        .bg(move || nsh_base::theme().card)
        .child(title_block(
            "Create your account",
            "Enter your details below to create your account",
        ))
        .child(socials(&["Apple", "Google"]))
        .child(name_field())
        .child(email_field("name@example"))
        .child(password_field(false))
        .child(aurora_components::button("Create Account", || {}))
}

fn split_signup() -> impl IntoElement {
    view()
        .w(px(680.0))
        .flex_row()
        .rounded(px(10.0))
        .border(px(1.0))
        .border_color(move || nsh_base::theme().border)
        .bg(move || nsh_base::theme().card)
        .child(
            view()
                .flex_1()
                .p_5()
                .gap_3()
                .child(title_block(
                    "Create your account",
                    "Enter your details below to create your account",
                ))
                .child(name_field())
                .child(email_field("name@example"))
                .child(password_field(false))
                .child(aurora_components::button("Create Account", || {}))
                .child(separator("Or continue with"))
                .child(socials(&["Apple", "Google", "GitHub"])),
        )
        .child(view().w(px(220.0)).bg(move || nsh_base::theme().muted))
}

fn compact_signup() -> impl IntoElement {
    view()
        .w(px(360.0))
        .gap_3()
        .items_center()
        .child(
            text("Welcome to Acme Inc.")
                .text_size(px(18.0))
                .font_weight(FontWeight::BOLD),
        )
        .child(email_field("name@example"))
        .child(aurora_components::button("Create Account", || {}))
        .child(separator("Or"))
        .child(socials(&["Apple", "Google"]))
        .child(ui::chip("By clicking continue, you agree to our terms"))
}
