const VOCABULARY: [&str; 18] = [
    "flex",
    "flex-row",
    "flex-col",
    "items-center",
    "justify-between",
    "gap-1",
    "gap-2",
    "gap-3",
    "gap-4",
    "p-2",
    "p-3",
    "p-4",
    "px-4",
    "py-2",
    "rounded",
    "rounded-full",
    "text-sm",
    "text-lg",
];

pub struct Diagnostic {
    pub rule: &'static str,
    pub file: String,
    pub line: usize,
    pub message: String,
}

fn push(out: &mut Vec<Diagnostic>, rule: &'static str, file: &str, line: usize, message: &str) {
    out.push(Diagnostic {
        rule,
        file: file.to_string(),
        line,
        message: message.to_string(),
    });
}

pub fn check(file: &str, source: &str) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    for (index, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") {
            continue;
        }
        let number = index + 1;
        let color_literal = trimmed.contains("rgb(")
            || trimmed.contains("0x")
            || (trimmed.contains("\"#") && !trimmed.starts_with("#["));
        if color_literal {
            push(
                &mut out,
                "no_raw_colors",
                file,
                number,
                "Use a theme token instead of a literal color.",
            );
        }
        if trimmed.contains("-[") && !trimmed.starts_with("#[") {
            push(
                &mut out,
                "no_arbitrary_values",
                file,
                number,
                "Use the spacing scale instead of an arbitrary class.",
            );
        }
        if trimmed.contains("format!(")
            && (trimmed.contains("class") || trimmed.contains("p-") || trimmed.contains("bg-"))
        {
            push(
                &mut out,
                "require_static_classes",
                file,
                number,
                "Use literal classes or a static table.",
            );
        }
        if trimmed.contains(".inline_style(") || trimmed.contains(".style_with(") {
            push(
                &mut out,
                "no_inline_styles",
                file,
                number,
                "Use typed styling helpers instead of raw inline styles.",
            );
        }
        if let Some(pos) = trimmed.find("class=\"") {
            let rest = &trimmed[pos + 7..];
            let end = rest.find('"').unwrap_or(rest.len());
            for class in rest[..end].split_whitespace() {
                if !VOCABULARY.contains(&class) {
                    push(
                        &mut out,
                        "no_unknown_classes",
                        file,
                        number,
                        "Use a class from the supported vocabulary.",
                    );
                }
            }
        }
        if trimmed.contains(".class(\"p-") || trimmed.contains(".class(\"bg-") {
            push(
                &mut out,
                "no_restyle",
                file,
                number,
                "Use the component variants instead of restyling it.",
            );
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
        assert!(check("a.rs", "// let x = rgb(0x112233);").is_empty());
    }

    #[test]
    fn finds_arbitrary_values() {
        let d = check("a.rs", "class=\"w-[13px]\"");
        assert!(d.iter().any(|v| v.rule == "no_arbitrary_values"));
    }

    #[test]
    fn finds_inline_styles() {
        let d = check("a.rs", ".inline_style(raw)");
        assert!(d.iter().any(|v| v.rule == "no_inline_styles"));
    }

    #[test]
    fn finds_unknown_classes() {
        let d = check("a.rs", "class=\"rounded-huge\"");
        assert!(d.iter().any(|v| v.rule == "no_unknown_classes"));
    }

    #[test]
    fn finds_restyling() {
        let d = check("a.rs", ".class(\"p-4\")");
        assert!(d.iter().any(|v| v.rule == "no_restyle"));
    }
}
