use nsh_core::prelude::*;
use nsh_core::url_scheme::HistoryBehavior;

pub fn path_of(route: &'static str) -> &'static str {
    match route {
        "Docs" => "/docs",
        "Components" => "/docs/components",
        "AIElements" => "/docs/ai-elements",
        "CLI" => "/docs/cli",
        "Theming" => "/docs/theming",
        _ => "/",
    }
}

pub fn route_of(url: &str) -> &'static str {
    if url.contains("/docs/components") {
        "Components"
    } else if url.contains("/docs/ai-elements") {
        "AIElements"
    } else if url.contains("/docs/cli") {
        "CLI"
    } else if url.contains("/docs/theming") {
        "Theming"
    } else if url.contains("/docs") {
        "Docs"
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
