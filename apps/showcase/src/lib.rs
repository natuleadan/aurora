//! Aurora — a component showcase written entirely in Rust.
//!
//! The engine renders these elements with each platform's own views, so the
//! same source is the website, the desktop app and the terminal surface.

use nsh_core::prelude::*;

mod pages;
mod shell;
mod ui;

/// Components exported to an optional script entry by the engine.
#[native_module("aurora/showcase")]
fn native(registry: &mut Registry) {
    registry.register::<Showcase>();
}

/// Application root: one scrollable column with a shell and a route view.
#[component]
pub fn Showcase() -> impl IntoElement {
    let route = signal("Home");
    let menu = signal(false);
    let scheme = signal(0_u8);

    scroll()
        .flex_1()
        .bg(move || nsh_base::theme().background)
        .text_color(move || nsh_base::theme().foreground)
        .child(
            view()
                .flex_1()
                .p_safe_area()
                .px_6()
                .py_4()
                .gap_6()
                .child(shell::header(route, menu, scheme))
                .child(route_view(route))
                .child(shell::footer()),
        )
}

fn route_view(route: Signal<&'static str>) -> impl IntoElement {
    view()
        .flex_1()
        .gap_6()
        .child(when(
            move || route.get() == "Home",
            move || pages::home::home(route),
        ))
        .child(when(move || route.get() == "Create", pages::create::create))
        .child(when(
            move || route.get() == "Components",
            pages::home::gallery,
        ))
        .child(when(
            move || {
                let r = route.get();
                r != "Home" && r != "Create" && r != "Components"
            },
            move || pages::placeholder(route.get()),
        ))
}
