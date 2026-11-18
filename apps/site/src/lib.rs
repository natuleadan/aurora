use nsh_core::prelude::*;

mod pages;
mod router;
mod shell;

#[native_module("aurora/site")]
fn native(registry: &mut Registry) {
    registry.register::<Site>();
}

#[component]
pub fn Site() -> impl IntoElement {
    let route = signal("Home");
    router::start(route);

    view()
        .flex_1()
        .bg(move || nsh_base::theme().background)
        .text_color(move || nsh_base::theme().foreground)
        .p_safe_area()
        .child(shell::header(route))
        .child(scroll().flex_1().px_6().py_4().child(route_view(route)))
        .child(shell::footer())
}

fn route_view(route: Signal<&'static str>) -> impl IntoElement {
    view()
        .flex_1()
        .gap_6()
        .child(when(move || route.get() == "Home", pages::home::home))
        .child(when(move || route.get() == "Docs", pages::docs::docs))
        .child(when(
            move || route.get() == "Components",
            pages::components::components,
        ))
        .child(when(
            move || route.get() == "AIElements",
            pages::ai_elements::ai_elements,
        ))
        .child(when(move || route.get() == "CLI", pages::cli::cli))
        .child(when(
            move || route.get() == "Theming",
            pages::theming::theming,
        ))
}
