use nsh_core::prelude::*;
use nsh_core::url_scheme::HistoryBehavior;

pub fn path_of(route: &'static str) -> &'static str {
    match route {
        "Installation" => "/docs/installation",
        "Components" => "/docs/components",
        "Gallery" => "/components",
        "Blocks" => "/blocks",
        "BlocksSidebar" => "/blocks/sidebar",
        "BlocksLogin" => "/blocks/login",
        "BlocksSignup" => "/blocks/signup",
        "Charts" => "/charts/area",
        "Directory" => "/docs/directory",
        "Typeset" => "/typeset",
        "Create" => "/create",
        "Command" => "/command",
        "ViewLogin1" => "/view/login-01",
        "ViewLogin2" => "/view/login-02",
        "ViewLogin3" => "/view/login-03",
        "ViewLogin4" => "/view/login-04",
        "ViewLogin5" => "/view/login-05",
        "ViewSignup1" => "/view/signup-01",
        "ViewSignup2" => "/view/signup-02",
        "ViewSignup3" => "/view/signup-03",
        "ViewSignup4" => "/view/signup-04",
        "ViewSignup5" => "/view/signup-05",
        "ViewSidebar1" => "/view/sidebar-01",
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
    } else if url.contains("/blocks/sidebar") {
        "BlocksSidebar"
    } else if url.contains("/blocks/login") {
        "BlocksLogin"
    } else if url.contains("/blocks/signup") {
        "BlocksSignup"
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
    } else if url.contains("/view/signup-01") {
        "ViewSignup1"
    } else if url.contains("/view/signup-02") {
        "ViewSignup2"
    } else if url.contains("/view/signup-03") {
        "ViewSignup3"
    } else if url.contains("/view/signup-04") {
        "ViewSignup4"
    } else if url.contains("/view/signup-05") {
        "ViewSignup5"
    } else if url.contains("/view/login-01") {
        "ViewLogin1"
    } else if url.contains("/view/login-02") {
        "ViewLogin2"
    } else if url.contains("/view/login-03") {
        "ViewLogin3"
    } else if url.contains("/view/login-04") {
        "ViewLogin4"
    } else if url.contains("/view/login-05") {
        "ViewLogin5"
    } else if url.contains("/view/sidebar-01") {
        "ViewSidebar1"
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
