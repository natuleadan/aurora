const GROUPS: [&str; 12] = [
    "p-", "px-", "py-", "m-", "mx-", "my-", "gap-", "rounded-", "bg-", "text-", "border-", "w-",
];

fn group_of(class: &str) -> Option<&'static str> {
    let mut i = 0;
    while i < GROUPS.len() {
        if class.starts_with(GROUPS[i]) {
            return Some(GROUPS[i]);
        }
        i += 1;
    }
    None
}

pub fn cn(parts: &[&str]) -> String {
    let mut out: Vec<String> = Vec::new();
    let mut groups: Vec<Option<&'static str>> = Vec::new();
    for part in parts {
        for class in part.split_whitespace() {
            let group = group_of(class);
            if let Some(g) = group
                && let Some(pos) = groups.iter().position(|v| *v == Some(g))
            {
                out[pos] = class.to_string();
                continue;
            }
            out.push(class.to_string());
            groups.push(group);
        }
    }
    out.join(" ")
}

#[macro_export]
macro_rules! cn {
    ($($part:expr),* $(,)?) => {
        $crate::cn(&[$($part),*])
    };
}

#[cfg(test)]
mod tests {
    use super::cn;

    #[test]
    fn joins_and_resolves_conflicts() {
        assert_eq!(cn(&["px-2 py-1", "px-4"]), "px-4 py-1");
        assert_eq!(cn(&["bg-neutral-100", "bg-neutral-900"]), "bg-neutral-900");
    }

    #[test]
    fn keeps_distinct_groups() {
        assert_eq!(cn(&["p-2", "text-sm"]), "p-2 text-sm");
    }
}
