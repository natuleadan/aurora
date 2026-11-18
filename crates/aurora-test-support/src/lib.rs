pub mod assertions;
pub mod events;
pub mod harness;

pub use assertions::{assert_accessible_label, assert_interactive_state, assert_valid_token_name};
pub use events::{PointerAction, SimulatedKey, SimulatedPointerEvent};
pub use harness::HeadlessHarness;

#[cfg(test)]
mod tests {
    use super::*;
    use aurora_primitives::AriaAttributes;

    #[test]
    fn harness_simulates_inputs() {
        let mut harness = HeadlessHarness::new();
        assert!(!harness.current_focus.is_focused());

        harness.send_key(SimulatedKey::Tab);
        assert!(harness.current_focus.is_focused());
        assert!(harness.focus_ring().visible);

        harness.send_pointer(SimulatedPointerEvent::hover(10.0, 20.0));
        assert!(harness.interaction.hovered);
    }

    #[test]
    fn test_assertions() {
        assert_valid_token_name("primary");
        assert_interactive_state(false, true);

        let attrs = AriaAttributes::new().label("Close dialog");
        assert_accessible_label(&attrs);
    }
}
