use crate::events::{PointerAction, SimulatedKey, SimulatedPointerEvent};
use aurora_primitives::{FocusRing, FocusState, InteractionState};

#[derive(Debug, Default)]
pub struct HeadlessHarness {
    pub current_focus: FocusState,
    pub interaction: InteractionState,
    pub key_log: Vec<SimulatedKey>,
    pub pointer_log: Vec<SimulatedPointerEvent>,
}

impl HeadlessHarness {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn send_key(&mut self, key: SimulatedKey) {
        if key == SimulatedKey::Tab {
            self.current_focus = FocusState::FocusVisible;
        }
        self.key_log.push(key);
    }

    pub fn send_pointer(&mut self, event: SimulatedPointerEvent) {
        match event.action {
            PointerAction::Hover => self.interaction.hovered = true,
            PointerAction::Leave => self.interaction.hovered = false,
            PointerAction::Press => self.interaction.pressed = true,
            PointerAction::Release | PointerAction::Click => self.interaction.pressed = false,
        }
        self.pointer_log.push(event);
    }

    pub fn focus_ring(&self) -> FocusRing {
        FocusRing::new(self.current_focus.is_visible())
    }
}
