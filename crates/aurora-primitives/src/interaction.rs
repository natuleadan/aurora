#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct InteractionState {
    pub hovered: bool,
    pub pressed: bool,
    pub disabled: bool,
    pub loading: bool,
}

impl InteractionState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn is_interactive(&self) -> bool {
        !self.disabled && !self.loading
    }

    pub fn opacity_factor(&self) -> f32 {
        if self.disabled {
            0.5
        } else if self.pressed {
            0.8
        } else {
            1.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interaction_state_logic() {
        let mut state = InteractionState::new();
        assert!(state.is_interactive());
        assert_eq!(state.opacity_factor(), 1.0);

        state.pressed = true;
        assert_eq!(state.opacity_factor(), 0.8);

        let disabled = InteractionState::new().disabled(true);
        assert!(!disabled.is_interactive());
        assert_eq!(disabled.opacity_factor(), 0.5);
    }
}
