#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimulatedKey {
    Tab,
    Enter,
    Escape,
    Space,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Char(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointerAction {
    Hover,
    Press,
    Release,
    Click,
    Leave,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SimulatedPointerEvent {
    pub action: PointerAction,
    pub x: f32,
    pub y: f32,
}

impl SimulatedPointerEvent {
    pub fn click(x: f32, y: f32) -> Self {
        Self {
            action: PointerAction::Click,
            x,
            y,
        }
    }

    pub fn hover(x: f32, y: f32) -> Self {
        Self {
            action: PointerAction::Hover,
            x,
            y,
        }
    }
}
