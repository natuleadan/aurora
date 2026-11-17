pub struct Manifest {
    pub name: &'static str,
    pub category: &'static str,
    pub files: &'static [&'static str],
    pub dependencies: &'static [&'static str],
}

const CORE: &[&str] = &["aurora-primitives", "aurora-cn"];

static MANIFESTS: [Manifest; 20] = [
    Manifest {
        name: "button",
        category: "action",
        files: &["button.rs"],
        dependencies: CORE,
    },
    Manifest {
        name: "button-group",
        category: "action",
        files: &["button_group.rs"],
        dependencies: CORE,
    },
    Manifest {
        name: "badge",
        category: "data",
        files: &["badge.rs"],
        dependencies: CORE,
    },
    Manifest {
        name: "toggle",
        category: "action",
        files: &["toggle.rs"],
        dependencies: CORE,
    },
    Manifest {
        name: "toggle-group",
        category: "action",
        files: &["toggle_group.rs"],
        dependencies: CORE,
    },
    Manifest {
        name: "card",
        category: "container",
        files: &["card.rs"],
        dependencies: CORE,
    },
    Manifest {
        name: "separator",
        category: "container",
        files: &["separator.rs"],
        dependencies: CORE,
    },
    Manifest {
        name: "skeleton",
        category: "feedback",
        files: &["skeleton.rs"],
        dependencies: CORE,
    },
    Manifest {
        name: "input",
        category: "forms",
        files: &["input.rs"],
        dependencies: CORE,
    },
    Manifest {
        name: "textarea",
        category: "forms",
        files: &["textarea.rs"],
        dependencies: CORE,
    },
    Manifest {
        name: "label",
        category: "forms",
        files: &["label.rs"],
        dependencies: CORE,
    },
    Manifest {
        name: "checkbox",
        category: "forms",
        files: &["checkbox.rs"],
        dependencies: CORE,
    },
    Manifest {
        name: "switch",
        category: "forms",
        files: &["switch.rs"],
        dependencies: CORE,
    },
    Manifest {
        name: "radio-group",
        category: "forms",
        files: &["radio_group.rs"],
        dependencies: CORE,
    },
    Manifest {
        name: "select",
        category: "forms",
        files: &["select.rs"],
        dependencies: CORE,
    },
    Manifest {
        name: "slider",
        category: "forms",
        files: &["slider.rs"],
        dependencies: CORE,
    },
    Manifest {
        name: "progress",
        category: "feedback",
        files: &["progress.rs"],
        dependencies: CORE,
    },
    Manifest {
        name: "avatar",
        category: "data",
        files: &["avatar.rs"],
        dependencies: CORE,
    },
    Manifest {
        name: "tabs",
        category: "navigation",
        files: &["tabs.rs"],
        dependencies: CORE,
    },
    Manifest {
        name: "accordion",
        category: "container",
        files: &["accordion.rs"],
        dependencies: CORE,
    },
];

pub fn all() -> &'static [Manifest] {
    &MANIFESTS
}

pub fn find(name: &str) -> Option<&'static Manifest> {
    all().iter().find(|m| m.name == name)
}

#[cfg(test)]
mod tests {
    use super::{all, find};

    #[test]
    fn catalog_has_unique_names() {
        let items = all();
        let mut i = 1;
        while i < items.len() {
            assert_ne!(items[i - 1].name, items[i].name);
            i += 1;
        }
    }

    #[test]
    fn lookup_works() {
        assert_eq!(find("button").unwrap().category, "action");
        assert!(find("missing").is_none());
    }
}
