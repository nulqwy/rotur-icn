use std::time::Instant;

use rotur_icn_syntax::lex_and_parse;

const ICN: &str = "\
w 3 square 7 7 2.5 2.5 1 square 7 -7 2.5 2.5 1 square -7 7 2.5 2.5 1 square -7 -7 2.5 2.5 1 dot -7 7 dot 7 7 dot -7 -7 dot 7 -7
";

fn main() {
    let start = Instant::now();
    let (icon, errors) = lex_and_parse(std::hint::black_box(ICN));
    let end = Instant::now();

    let dur = end - start;

    println!("{icon}");
    println!(
        "Lexing & parsing time: {}μs",
        (dur.as_secs_f64() * 1_000_000.0) as u64
    );

    if !errors.is_empty() {
        eprintln!("=== FOUND ERRORS ===");
        if !errors.parsing.is_empty() {
            eprintln!("parsing errors:");
            for error in errors.parsing {
                eprintln!("- {error}");
                eprintln!("    [ {} ]", error.help());
            }
        }
        if !errors.lexing.is_empty() {
            eprintln!("lexing errors:");
            for error in errors.lexing {
                eprintln!("- {error}");
                eprintln!("    [ {} ]", error.kind.help());
            }
        }
    }
}
