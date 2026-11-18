use aurora_primitives::AriaAttributes;

pub fn assert_accessible_label(attrs: &AriaAttributes) {
    assert!(
        attrs.label.is_some() && !attrs.label.as_ref().unwrap().is_empty(),
        "Component must provide an accessible label"
    );
}

pub fn assert_interactive_state(disabled: bool, should_be_interactive: bool) {
    if disabled {
        assert!(
            !should_be_interactive,
            "Disabled component cannot be interactive"
        );
    } else {
        assert!(
            should_be_interactive,
            "Enabled component should be interactive"
        );
    }
}

pub fn assert_valid_token_name(token: &str) {
    assert!(
        aurora_tokens::TOKENS.contains(&token),
        "Token '{token}' must be present in aurora-tokens catalog"
    );
}
