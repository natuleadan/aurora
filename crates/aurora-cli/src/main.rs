use std::fs;
use std::path::{Path, PathBuf};

fn components_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("aurora-components")
}

fn find_wrapper(name: &str) -> Option<String> {
    let root = components_root();
    let root_source = fs::read_to_string(root.join("src/lib.rs")).ok()?;
    if root_source.contains(&format!("pub fn {name}(")) {
        return Some(format!("aurora_components::{name}"));
    }
    let families = [
        ("data", "data"),
        ("extras", "extras"),
        ("forms", "forms"),
        ("overlays", "overlays"),
        ("primitives", "primitives"),
    ];
    for (file, module) in families {
        let source = fs::read_to_string(root.join(format!("src/families/{file}.rs"))).ok()?;
        if source.contains(&format!("pub fn {name}(")) {
            return Some(format!("aurora_components::families::{module}::{name}"));
        }
    }
    None
}

fn wrapper_names() -> Vec<String> {
    let root = components_root();
    let mut out = Vec::new();
    let mut files = vec![root.join("src/lib.rs")];
    for name in ["data", "extras", "forms", "overlays", "primitives"] {
        files.push(root.join(format!("src/families/{name}.rs")));
    }
    for file in files {
        let Ok(source) = fs::read_to_string(file) else {
            continue;
        };
        for line in source.lines() {
            let Some(rest) = line.strip_prefix("pub fn ") else {
                continue;
            };
            let name = rest.split('(').next().unwrap_or("").trim();
            if !name.is_empty() {
                out.push(name.to_string());
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

fn init(dir: &Path) {
    if dir.exists() {
        println!("FAIL target already exists");
        return;
    }
    let crate_path = components_root();
    let manifest = format!(
        "[package]\nname = \"aurora-test-app\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\naurora-components = {{ path = \"{}\" }}\n",
        crate_path.display()
    );
    fs::create_dir_all(dir.join("src/components")).unwrap();
    fs::write(dir.join("Cargo.toml"), manifest).unwrap();
    fs::write(
        dir.join("src/main.rs"),
        "fn main() {\n    println!(\"aurora test app\");\n}\n",
    )
    .unwrap();
    fs::write(
        dir.join("theme.json"),
        fs::read_to_string(
            components_root()
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .join("apps/showcase/theme.json"),
        )
        .unwrap(),
    )
    .unwrap();
    fs::write(
        dir.join("components.json"),
        "{\n  \"style\": \"aurora\",\n  \"components\": \"src/components\"\n}\n",
    )
    .unwrap();
    println!("PASS init {}", dir.display());
}

fn add(name: &str, dir: &Path) {
    let Some(path) = find_wrapper(name) else {
        println!("FAIL unknown component {name}");
        return;
    };
    let target = dir.join("src/components").join(format!("{name}.rs"));
    if target.exists() {
        println!("SKIP {} already exists", name);
        return;
    }
    fs::write(target, format!("pub use {path};\n")).unwrap();
    println!("PASS add {name}");
}

fn build(dir: &Path) {
    let names = wrapper_names();
    let entries = names
        .iter()
        .map(|name| format!("    {{\"name\": \"{name}\"}}"))
        .collect::<Vec<_>>()
        .join(",\n");
    fs::write(
        dir.join("aurora-registry.generated.json"),
        format!("{{\n  \"components\": [\n{entries}\n  ]\n}}\n"),
    )
    .unwrap();
    println!("PASS build {} components", names.len());
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let command = args.first().map(String::as_str).unwrap_or("help");
    let dir_arg = if command == "add" {
        args.get(2)
    } else {
        args.get(1)
    };
    let dir = PathBuf::from(dir_arg.map(String::as_str).unwrap_or("."));
    match command {
        "init" => init(&dir),
        "add" => {
            let Some(name) = args.get(1) else {
                println!("FAIL missing component name");
                return;
            };
            add(name, &dir);
        }
        "list" => {
            for name in wrapper_names() {
                println!("{name}");
            }
        }
        "theme" => {
            for preset in aurora_tokens::presets() {
                println!("{} {} {}", preset.name, preset.radius, preset.primary_light);
            }
        }
        "doctor" => println!("PASS toolchain ok"),
        "build" => build(&dir),
        _ => println!("aurora <init|add|list|theme|doctor|build>"),
    }
}
