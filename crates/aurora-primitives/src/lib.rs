pub mod aria;
pub mod focus;
pub mod interaction;
pub mod selection;

pub use aria::{AriaAttributes, AriaRole};
pub use aurora_cn::cn;
pub use aurora_tokens::TOKENS;
pub use focus::{FocusRing, FocusState};
pub use interaction::InteractionState;
pub use selection::{MultiSelection, SelectionState};

pub fn style_focus_ring(focused: bool) -> &'static str {
    if focused { "ring" } else { "" }
}

pub fn style_disabled(disabled: bool) -> f32 {
    if disabled { 0.5 } else { 1.0 }
}

pub fn is_valid_token(name: &str) -> bool {
    TOKENS.contains(&name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitives_helpers() {
        assert_eq!(style_focus_ring(true), "ring");
        assert_eq!(style_focus_ring(false), "");
        assert_eq!(style_disabled(true), 0.5);
        assert_eq!(style_disabled(false), 1.0);
        assert!(is_valid_token("primary"));
        assert!(!is_valid_token("nonexistent-token"));
    }
}
