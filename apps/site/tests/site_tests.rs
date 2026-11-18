#[cfg(test)]
mod tests {
    use nsh_engine::{EngineConfig, Root};
    use nsh_testing::Headless;

    static ENGINE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    fn site_home_renders() {
        let _guard = ENGINE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let config = EngineConfig::new(Root::of::<aurora_site::Site>());
        let mut app = Headless::start(config).unwrap();
        app.wait_for("text:Natuleadan® Aurora™").unwrap();
        assert!(app.screen().texts().contains(&"Get Started"));
    }
}
