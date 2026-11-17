use nsh_core::prelude::*;

macro_rules! case {
    ($id:literal, $element:expr) => {
        view()
            .test_id(concat!("case-", $id))
            .gap_1()
            .child(text(concat!("case-", $id)).text_size(px(10.0)))
            .child($element)
    };
}

pub fn testbed() -> impl IntoElement {
    let input_value = signal(String::new());
    let progress_value = signal(40.0_f32);
    static ROWS: [(&str, &str); 2] = [("left", "right"), ("one", "two")];
    static ITEMS: [&str; 3] = ["one", "two", "three"];
    static PAIRS: [(&str, &str); 2] = [("a", "Alpha"), ("b", "Beta")];
    static TRIPLES: [(&str, &str, &str); 2] =
        [("a", "First", "First body"), ("b", "Second", "Second body")];

    scroll()
        .flex_1()
        .gap_2()
        .child(case!("button", aurora_components::button("Button", || {})))
        .child(case!("badge", aurora_components::badge("Badge")))
        .child(case!("card", aurora_components::card("Card", "Body")))
        .child(case!(
            "input",
            aurora_components::input(input_value, "Input")
        ))
        .child(case!("checkbox", aurora_components::checkbox("Checkbox")))
        .child(case!("switch", aurora_components::switch("Switch")))
        .child(case!("toggle", aurora_components::toggle("Toggle")))
        .child(case!(
            "collapsible",
            aurora_components::collapsible("Collapsible", "Body")
        ))
        .child(case!("separator", aurora_components::separator()))
        .child(case!("skeleton", aurora_components::skeleton(40.0, 12.0)))
        .child(case!("label", aurora_components::label("Label")))
        .child(case!("alert", aurora_components::alert("Alert", "Body")))
        .child(case!("avatar", aurora_components::avatar("AV")))
        .child(case!("kbd", aurora_components::kbd("Kbd")))
        .child(case!("breadcrumb", aurora_components::breadcrumb(&ITEMS)))
        .child(case!("table", aurora_components::table(&ROWS)))
        .child(case!("pagination", aurora_components::pagination(&ITEMS)))
        .child(case!("radio_group", aurora_components::radio_group(&PAIRS)))
        .child(case!(
            "dialog_trigger",
            aurora_components::dialog_trigger("Dialog", "Title", "Body")
        ))
        .child(case!("hint", aurora_components::hint("Hint", "Message")))
        .child(case!("tabs", aurora_components::tabs(&TRIPLES)))
        .child(case!("accordion", aurora_components::accordion(&TRIPLES)))
        .child(case!(
            "progress",
            aurora_components::progress(progress_value)
        ))
        .child(case!(
            "data_table",
            aurora_components::families::data::data_table(&ROWS)
        ))
        .child(case!(
            "plot",
            aurora_components::families::data::plot("Plot")
        ))
        .child(case!(
            "tree",
            aurora_components::families::data::tree(&ITEMS)
        ))
        .child(case!(
            "diff_view",
            aurora_components::families::data::diff_view("Diff")
        ))
        .child(case!(
            "rich_editor",
            aurora_components::families::data::rich_editor("Editor")
        ))
        .child(case!(
            "terminal",
            aurora_components::families::data::terminal("Terminal")
        ))
        .child(case!(
            "text_view",
            aurora_components::families::data::text_view("Text")
        ))
        .child(case!(
            "resizable",
            aurora_components::families::data::resizable("Resizable")
        ))
        .child(case!(
            "dock",
            aurora_components::families::data::dock("Dock")
        ))
        .child(case!(
            "scrollbar",
            aurora_components::families::data::scrollbar()
        ))
        .child(case!(
            "safe_area",
            aurora_components::families::data::safe_area()
        ))
        .child(case!(
            "button_group",
            aurora_components::families::extras::button_group(&ITEMS)
        ))
        .child(case!(
            "toggle_group",
            aurora_components::families::extras::toggle_group(&ITEMS)
        ))
        .child(case!(
            "textarea",
            aurora_components::families::extras::textarea("Textarea")
        ))
        .child(case!(
            "chart",
            aurora_components::families::extras::chart("Chart")
        ))
        .child(case!(
            "carousel",
            aurora_components::families::extras::carousel(&ITEMS)
        ))
        .child(case!(
            "command",
            aurora_components::families::extras::command(&ITEMS)
        ))
        .child(case!(
            "editor",
            aurora_components::families::extras::editor("Code")
        ))
        .child(case!(
            "emoji_picker",
            aurora_components::families::extras::emoji_picker("Emoji")
        ))
        .child(case!(
            "number_input",
            aurora_components::families::forms::number_input("Number")
        ))
        .child(case!(
            "otp_input",
            aurora_components::families::forms::otp_input("OTP")
        ))
        .child(case!(
            "select",
            aurora_components::families::forms::select(&ITEMS)
        ))
        .child(case!(
            "slider",
            aurora_components::families::forms::slider("Slider")
        ))
        .child(case!(
            "time_field",
            aurora_components::families::forms::time_field("Time")
        ))
        .child(case!(
            "date_picker",
            aurora_components::families::forms::date_picker("Date")
        ))
        .child(case!(
            "color_picker",
            aurora_components::families::forms::color_picker("Color")
        ))
        .child(case!(
            "calendar",
            aurora_components::families::forms::calendar("Calendar")
        ))
        .child(case!(
            "combobox",
            aurora_components::families::forms::combobox(&ITEMS)
        ))
        .child(case!(
            "keyboard",
            aurora_components::families::forms::keyboard("Keyboard")
        ))
        .child(case!(
            "text_selection",
            aurora_components::families::forms::text_selection("Selection")
        ))
        .child(case!(
            "link",
            aurora_components::families::overlays::link("Link")
        ))
        .child(case!(
            "menu",
            aurora_components::families::overlays::menu("Menu")
        ))
        .child(case!(
            "nav_stack",
            aurora_components::families::overlays::nav_stack("Stack")
        ))
        .child(case!(
            "toolbar",
            aurora_components::families::overlays::toolbar("Toolbar")
        ))
        .child(case!(
            "popover",
            aurora_components::families::overlays::popover("Popover")
        ))
        .child(case!(
            "sheet",
            aurora_components::families::overlays::sheet("Sheet")
        ))
        .child(case!(
            "toast",
            aurora_components::families::overlays::toast("Toast")
        ))
        .child(case!(
            "alert_dialog",
            aurora_components::families::overlays::alert_dialog("Confirm")
        ))
        .child(case!(
            "hover_card",
            aurora_components::families::overlays::hover_card("Card")
        ))
        .child(case!(
            "context_menu",
            aurora_components::families::overlays::context_menu("Actions")
        ))
        .child(case!(
            "view",
            aurora_components::families::primitives::view(text("View"))
        ))
        .child(case!(
            "text",
            aurora_components::families::primitives::text("Text")
        ))
        .child(case!(
            "image",
            aurora_components::families::primitives::image("Image")
        ))
        .child(case!(
            "scroll",
            aurora_components::families::primitives::scroll(text("Scroll"))
        ))
        .child(case!(
            "list",
            aurora_components::families::primitives::list(&ITEMS)
        ))
        .child(case!(
            "canvas",
            aurora_components::families::primitives::canvas("Canvas")
        ))
        .child(case!(
            "shader",
            aurora_components::families::primitives::shader("Shader")
        ))
        .child(case!(
            "web_view",
            aurora_components::families::primitives::web_view("Web")
        ))
        .child(case!(
            "video",
            aurora_components::families::primitives::video("Video")
        ))
        .child(case!(
            "overlay",
            aurora_components::families::primitives::overlay("Overlay")
        ))
        .child(case!(
            "control_flow",
            aurora_components::families::primitives::control_flow("Flow")
        ))
}
