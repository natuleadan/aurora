use std::fs;

fn main() {
    let mut failed = false;
    for path in std::env::args().skip(1) {
        let source = match fs::read_to_string(&path) {
            Ok(text) => text,
            Err(error) => {
                println!("FAIL {path}: {error}");
                failed = true;
                continue;
            }
        };
        let diagnostics = aurora_lint::check(&path, &source);
        if diagnostics.is_empty() {
            println!("PASS {path}");
        } else {
            for d in diagnostics {
                println!("FAIL {}:{} {} — {}", d.file, d.line, d.rule, d.message);
            }
            failed = true;
        }
    }
    if failed {
        std::process::exit(1);
    }
}
