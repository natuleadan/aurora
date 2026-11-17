use nsh_core::prelude::*;

mod pages;
mod router;
mod shell;
mod ui;

#[native_module("aurora/showcase")]
fn native(registry: &mut Registry) {
    registry.register::<Showcase>();
}

#[component]
pub fn Showcase() -> impl IntoElement {
    let route = signal("Home");
    let menu = signal(false);
    let scheme = signal(0_u8);
    router::start(route);

    view()
        .flex_1()
        .bg(move || nsh_base::theme().background)
        .text_color(move || nsh_base::theme().foreground)
        .p_safe_area()
        .child(shell::header(route, menu, scheme))
        .child(scroll().flex_1().px_6().py_4().child(route_view(route)))
        .child(shell::footer())
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
            move || route.get() == "Command",
            move || pages::command::command(route),
        ))
        .child(when(move || route.get() == "Login", pages::login::login))
        .child(when(
            move || route.get() == "Components",
            pages::stubs::components_index,
        ))
        .child(when(move || route.get() == "Gallery", pages::home::gallery))
        .child(when(
            move || route.get() == "Installation",
            pages::stubs::installation,
        ))
        .child(when(move || route.get() == "Blocks", pages::stubs::blocks))
        .child(when(move || route.get() == "Charts", pages::stubs::charts))
        .child(when(
            move || route.get() == "Directory",
            pages::stubs::directory,
        ))
        .child(when(
            move || route.get() == "Typeset",
            pages::stubs::typeset,
        ))
        .child(when(
            move || {
                let r = route.get();
                r != "Home"
                    && r != "Create"
                    && r != "Command"
                    && r != "Login"
                    && r != "Components"
                    && r != "Gallery"
                    && r != "Installation"
                    && r != "Blocks"
                    && r != "Charts"
                    && r != "Directory"
                    && r != "Typeset"
            },
            move || pages::placeholder(route.get()),
        ))
}
