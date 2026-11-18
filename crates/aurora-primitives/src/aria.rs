#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AriaRole {
    Button,
    Checkbox,
    Dialog,
    Alert,
    Switch,
    Tab,
    TabPanel,
    Tooltip,
    Menu,
    MenuItem,
    Status,
    Navigation,
}

impl AriaRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Button => "button",
            Self::Checkbox => "checkbox",
            Self::Dialog => "dialog",
            Self::Alert => "alert",
            Self::Switch => "switch",
            Self::Tab => "tab",
            Self::TabPanel => "tabpanel",
            Self::Tooltip => "tooltip",
            Self::Menu => "menu",
            Self::MenuItem => "menuitem",
            Self::Status => "status",
            Self::Navigation => "navigation",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AriaAttributes {
    pub role: Option<AriaRole>,
    pub label: Option<String>,
    pub expanded: Option<bool>,
    pub selected: Option<bool>,
    pub disabled: bool,
}

impl AriaAttributes {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn role(mut self, role: AriaRole) -> Self {
        self.role = Some(role);
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = Some(expanded);
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = Some(selected);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aria_role_strings() {
        assert_eq!(AriaRole::Button.as_str(), "button");
        assert_eq!(AriaRole::Dialog.as_str(), "dialog");
        assert_eq!(AriaRole::Switch.as_str(), "switch");
    }

    #[test]
    fn aria_attributes_builder() {
        let attrs = AriaAttributes::new()
            .role(AriaRole::Button)
            .label("Submit form")
            .expanded(false);

        assert_eq!(attrs.role, Some(AriaRole::Button));
        assert_eq!(attrs.label.as_deref(), Some("Submit form"));
        assert_eq!(attrs.expanded, Some(false));
    }
}
