use nsh_core::prelude::*;
use nsh_core::url_scheme::HistoryBehavior;

pub fn path_of(route: &'static str) -> &'static str {
    match route {
        "Installation" => "/docs/installation",
        "Components" => "/docs/components",
        "Gallery" => "/components",
        "Blocks" => "/blocks",
        "Charts" => "/charts/area",
        "Directory" => "/docs/directory",
        "Typeset" => "/typeset",
        "Create" => "/create",
        "Command" => "/command",
        "Login" => "/login",
        _ => "/",
    }
}

pub fn route_of(url: &str) -> &'static str {
    if url.contains("/docs/installation") {
        "Installation"
    } else if url.contains("/docs/components") {
        "Components"
    } else if url.contains("/components") {
        "Gallery"
    } else if url.contains("/blocks") {
        "Blocks"
    } else if url.contains("/charts/area") {
        "Charts"
    } else if url.contains("/docs/directory") {
        "Directory"
    } else if url.contains("/typeset") {
        "Typeset"
    } else if url.contains("/create") {
        "Create"
    } else if url.contains("/command") {
        "Command"
    } else if url.contains("/login") {
        "Login"
    } else {
        "Home"
    }
}

pub fn navigate(route: &'static str) {
    let _ = set_app_url_with(path_of(route), HistoryBehavior::Push);
}

pub fn start(route: Signal<&'static str>) {
    spawn(async move {
        let current = app_url().await;
        route.set(route_of(current.as_deref().unwrap_or("")));
        loop {
            let change = next_app_url_change().await;
            route.set(route_of(&change.url));
        }
    });
}
