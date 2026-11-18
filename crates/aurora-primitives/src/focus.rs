#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FocusState {
    #[default]
    Unfocused,
    Focused,
    FocusVisible,
}

impl FocusState {
    pub fn is_focused(&self) -> bool {
        matches!(self, Self::Focused | Self::FocusVisible)
    }

    pub fn is_visible(&self) -> bool {
        matches!(self, Self::FocusVisible)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FocusRing {
    pub ring_width: u32,
    pub ring_offset: u32,
    pub visible: bool,
}

impl Default for FocusRing {
    fn default() -> Self {
        Self {
            ring_width: 2,
            ring_offset: 2,
            visible: false,
        }
    }
}

impl FocusRing {
    pub fn new(visible: bool) -> Self {
        Self {
            visible,
            ..Default::default()
        }
    }

    pub fn classes(&self) -> String {
        if self.visible {
            aurora_cn::cn(&["ring"])
        } else {
            String::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_state_checks() {
        let state = FocusState::FocusVisible;
        assert!(state.is_focused());
        assert!(state.is_visible());

        let unfocused = FocusState::Unfocused;
        assert!(!unfocused.is_focused());
    }

    #[test]
    fn focus_ring_classes() {
        let ring = FocusRing::new(true);
        assert_eq!(ring.classes(), "ring");
    }
}
