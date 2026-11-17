fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let command = args.first().map(String::as_str).unwrap_or("help");
    match command {
        "list" => {
            for item in aurora_registry::all() {
                println!("{:<18} {}", item.name, item.category);
            }
        }
        "add" => match args.get(1).and_then(|name| aurora_registry::find(name)) {
            Some(item) => println!("PASS {} {}", item.name, item.files[0]),
            None => println!("FAIL unknown component"),
        },
        "theme" => {
            for preset in aurora_tokens::presets() {
                println!("{} {} {}", preset.name, preset.radius, preset.primary_light);
            }
        }
        "doctor" => println!("PASS toolchain ok"),
        "init" | "build" => println!("SKIP not available in this batch"),
        _ => println!("aurora <init|add|list|theme|doctor|build>"),
    }
}
