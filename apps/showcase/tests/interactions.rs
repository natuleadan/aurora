use nsh_core::prelude::*;
use nsh_engine::{EngineConfig, Root};
use nsh_testing::Headless;

#[component]
fn BlocksHarness() -> impl IntoElement {
    aurora_showcase::blocks(signal("BlocksLogin"))
}

static ENGINE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

macro_rules! app_test {
    ($name:ident, $app:ident, $body:block) => {
        #[test]
        fn $name() {
            let _guard = ENGINE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
            let config = EngineConfig::new(Root::of::<aurora_showcase::Showcase>());
            let mut $app = Headless::start(config).unwrap();
            $body
        }
    };
}

macro_rules! blocks_test {
    ($name:ident, $app:ident, $body:block) => {
        #[test]
        fn $name() {
            let _guard = ENGINE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
            let config = EngineConfig::new(Root::of::<BlocksHarness>());
            let mut $app = Headless::start(config).unwrap();
            $body
        }
    };
}

app_test!(search_opens_command_palette, app, {
    app.wait_for("text:Search...").unwrap();
    app.press("text:Search...").unwrap();
    app.wait_for("text:Command palette").unwrap();
    assert!(
        app.screen()
            .texts()
            .contains(&"Search and jump to a section.")
    );
});

app_test!(new_opens_designer, app, {
    app.wait_for("text:Open the designer").unwrap();
    app.press("text:Open the designer").unwrap();
    app.wait_for("text:New project").unwrap();
    assert!(app.screen().texts().contains(&"Color tokens"));
});

app_test!(hero_cta_opens_gallery, app, {
    app.wait_for("text:Explore components").unwrap();
    app.press("text:Explore components").unwrap();
    app.wait_for("text:Component gallery").unwrap();
    assert!(app.screen().texts().contains(&"Native everywhere"));
});

app_test!(theme_button_cycles_label, app, {
    app.wait_for("text:System").unwrap();
    app.press("text:System").unwrap();
    app.wait_for("text:Light").unwrap();
    assert!(app.screen().texts().contains(&"Light"));
});

blocks_test!(blocks_tabs_switch_category, app, {
    app.wait_for("text:Login to your account").unwrap();
    app.press("text:Signup").unwrap();
    app.wait_for("text:Create an account").unwrap();
    assert!(app.screen().texts().contains(&"aurora add signup-01"));
});

blocks_test!(refresh_changes_block_variant, app, {
    app.wait_for("text:aurora add login-01").unwrap();
    app.press("text:Refresh").unwrap();
    app.wait_for("text:aurora add login-02").unwrap();
    assert!(app.screen().texts().contains(&"Login to your account"));
});

blocks_test!(copy_command_does_not_break_preview, app, {
    app.wait_for("text:Copy").unwrap();
    app.press("text:Copy").unwrap();
    assert!(app.screen().texts().contains(&"aurora add login-01"));
});
