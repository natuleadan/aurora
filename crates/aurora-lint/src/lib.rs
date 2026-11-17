pub struct Diagnostic {
    pub rule: &'static str,
    pub file: String,
    pub line: usize,
    pub message: String,
}

pub fn check(file: &str, source: &str) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    for (index, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") {
            continue;
        }
        let color_literal = trimmed.contains("rgb(")
            || trimmed.contains("0x")
            || (trimmed.contains("\"#") && !trimmed.starts_with("#["));
        if color_literal {
            out.push(Diagnostic {
                rule: "no_raw_colors",
                file: file.to_string(),
                line: index + 1,
                message: "Use a theme token instead of a literal color.".to_string(),
            });
        }
        if trimmed.contains("-[") {
            out.push(Diagnostic {
                rule: "no_arbitrary_values",
                file: file.to_string(),
                line: index + 1,
                message: "Use the spacing scale instead of an arbitrary class.".to_string(),
            });
        }
        if trimmed.contains("format!(") && trimmed.contains("-") {
            out.push(Diagnostic {
                rule: "require_static_classes",
                file: file.to_string(),
                line: index + 1,
                message: "Use literal classes or a static table.".to_string(),
            });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::check;

    #[test]
    fn finds_literal_colors() {
        let d = check("a.rs", "let x = rgb(0x112233);");
        assert!(d.iter().any(|v| v.rule == "no_raw_colors"));
    }

    #[test]
    fn ignores_comments() {
        let d = check("a.rs", "// let x = rgb(0x112233);");
        assert!(d.is_empty());
    }

    #[test]
    fn finds_arbitrary_values() {
        let d = check("a.rs", "class=\"w-[13px]\"");
        assert!(d.iter().any(|v| v.rule == "no_arbitrary_values"));
    }
}
