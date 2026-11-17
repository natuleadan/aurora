use nsh_core::prelude::*;
use nsh_engine::{EngineConfig, Root};
use nsh_testing::Headless;

#[component]
fn Harness() -> impl IntoElement {
    aurora_showcase::testbed()
}

#[component]
fn BlocksHarness() -> impl IntoElement {
    aurora_showcase::blocks(signal("BlocksLogin"))
}

static ENGINE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

macro_rules! component_test {
    ($name:ident, $id:literal) => {
        #[test]
        fn $name() {
            let _guard = ENGINE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
            let config = EngineConfig::new(Root::of::<Harness>());
            let mut app = Headless::start(config).unwrap();
            app.wait_for(concat!("id:case-", $id)).unwrap();
            assert!(
                app.screen()
                    .texts()
                    .iter()
                    .any(|text| *text == concat!("case-", $id)),
                "missing case {}",
                $id
            );
        }
    };
}

component_test!(button_renders, "button");
component_test!(badge_renders, "badge");
component_test!(card_renders, "card");
component_test!(input_renders, "input");
component_test!(checkbox_renders, "checkbox");
component_test!(switch_renders, "switch");
component_test!(toggle_renders, "toggle");
component_test!(collapsible_renders, "collapsible");
component_test!(separator_renders, "separator");
component_test!(skeleton_renders, "skeleton");
component_test!(label_renders, "label");
component_test!(alert_renders, "alert");
component_test!(avatar_renders, "avatar");
component_test!(kbd_renders, "kbd");
component_test!(breadcrumb_renders, "breadcrumb");
component_test!(table_renders, "table");
component_test!(pagination_renders, "pagination");
component_test!(radio_group_renders, "radio_group");
component_test!(dialog_trigger_renders, "dialog_trigger");
component_test!(hint_renders, "hint");
component_test!(tabs_renders, "tabs");
component_test!(accordion_renders, "accordion");
component_test!(progress_renders, "progress");

component_test!(data_table_renders, "data_table");
component_test!(plot_renders, "plot");
component_test!(tree_renders, "tree");
component_test!(diff_view_renders, "diff_view");
component_test!(rich_editor_renders, "rich_editor");
component_test!(terminal_renders, "terminal");
component_test!(text_view_renders, "text_view");
component_test!(resizable_renders, "resizable");
component_test!(dock_renders, "dock");
component_test!(scrollbar_renders, "scrollbar");
component_test!(safe_area_renders, "safe_area");

component_test!(button_group_renders, "button_group");
component_test!(toggle_group_renders, "toggle_group");
component_test!(textarea_renders, "textarea");
component_test!(chart_renders, "chart");
component_test!(carousel_renders, "carousel");
component_test!(command_renders, "command");
component_test!(editor_renders, "editor");
component_test!(emoji_picker_renders, "emoji_picker");

component_test!(number_input_renders, "number_input");
component_test!(otp_input_renders, "otp_input");
component_test!(select_renders, "select");
component_test!(slider_renders, "slider");
component_test!(time_field_renders, "time_field");
component_test!(date_picker_renders, "date_picker");
component_test!(color_picker_renders, "color_picker");
component_test!(calendar_renders, "calendar");
component_test!(combobox_renders, "combobox");
component_test!(keyboard_renders, "keyboard");
component_test!(text_selection_renders, "text_selection");

component_test!(link_renders, "link");
component_test!(menu_renders, "menu");
component_test!(nav_stack_renders, "nav_stack");
component_test!(toolbar_renders, "toolbar");
component_test!(popover_renders, "popover");
component_test!(sheet_renders, "sheet");
component_test!(toast_renders, "toast");
component_test!(alert_dialog_renders, "alert_dialog");
component_test!(hover_card_renders, "hover_card");
component_test!(context_menu_renders, "context_menu");

component_test!(view_renders, "view");
component_test!(text_renders, "text");
component_test!(image_renders, "image");
component_test!(scroll_renders, "scroll");
component_test!(list_renders, "list");
component_test!(canvas_renders, "canvas");
component_test!(shader_renders, "shader");
component_test!(web_view_renders, "web_view");
component_test!(video_renders, "video");
component_test!(overlay_renders, "overlay");
component_test!(control_flow_renders, "control_flow");

#[test]
fn route_home_shows_hero() {
    let _guard = ENGINE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let config = EngineConfig::new(Root::of::<aurora_showcase::Showcase>());
    let mut app = Headless::start(config).unwrap();
    app.wait_for("text:Explore components").unwrap();
    assert!(app.screen().texts().contains(&"Component gallery"));
}

#[test]
fn login_block_renders_inside_showroom() {
    let _guard = ENGINE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let config = EngineConfig::new(Root::of::<BlocksHarness>());
    let mut app = Headless::start(config).unwrap();
    app.wait_for("text:Blocks").unwrap();
    assert!(app.screen().texts().contains(&"aurora add login-01"));
    assert!(app.screen().texts().contains(&"Copy"));
    assert!(app.screen().texts().contains(&"Signup"));
}
