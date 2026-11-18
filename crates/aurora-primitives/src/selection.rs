#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectionState {
    #[default]
    Unselected,
    Selected,
    Indeterminate,
}

impl SelectionState {
    pub fn is_selected(&self) -> bool {
        matches!(self, Self::Selected)
    }

    pub fn is_indeterminate(&self) -> bool {
        matches!(self, Self::Indeterminate)
    }

    pub fn toggle(&self) -> Self {
        match self {
            Self::Unselected => Self::Selected,
            Self::Selected => Self::Unselected,
            Self::Indeterminate => Self::Selected,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MultiSelection<T: Clone + PartialEq> {
    selected_items: Vec<T>,
}

impl<T: Clone + PartialEq> MultiSelection<T> {
    pub fn new() -> Self {
        Self {
            selected_items: Vec::new(),
        }
    }

    pub fn is_selected(&self, item: &T) -> bool {
        self.selected_items.contains(item)
    }

    pub fn select(&mut self, item: T) {
        if !self.is_selected(&item) {
            self.selected_items.push(item);
        }
    }

    pub fn deselect(&mut self, item: &T) {
        self.selected_items.retain(|existing| existing != item);
    }

    pub fn toggle(&mut self, item: T) {
        if self.is_selected(&item) {
            self.deselect(&item);
        } else {
            self.select(item);
        }
    }

    pub fn count(&self) -> usize {
        self.selected_items.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_toggle() {
        let state = SelectionState::Unselected;
        assert_eq!(state.toggle(), SelectionState::Selected);
        assert_eq!(state.toggle().toggle(), SelectionState::Unselected);
    }

    #[test]
    fn multi_selection_ops() {
        let mut multi = MultiSelection::new();
        multi.select("item1");
        multi.select("item2");
        assert_eq!(multi.count(), 2);
        assert!(multi.is_selected(&"item1"));

        multi.toggle("item1");
        assert!(!multi.is_selected(&"item1"));
        assert_eq!(multi.count(), 1);
    }
}
