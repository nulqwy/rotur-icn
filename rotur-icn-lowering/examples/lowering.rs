use std::time::Instant;

use rotur_icn_lowering::lower;
use rotur_icn_syntax::lex_and_parse;

const ICN: &str = "\
545 * w 3 4 4 4 4 4 4 4 4 square 7 #fffe 7 2.5
";

fn main() {
    let start = Instant::now();
    let (icon, errors) = lex_and_parse(std::hint::black_box(ICN));
    let (icon_ir, ir_errors) = lower(icon);
    let end = Instant::now();

    let dur = end - start;

    println!("{icon_ir}");
    println!(
        "Lexing, parsing & lowering time: {}μs",
        (dur.as_secs_f64() * 1_000_000.0) as u64
    );

    if !errors.is_empty() || !ir_errors.is_empty() {
        eprintln!("=== FOUND ERRORS ===");
        if !ir_errors.is_empty() {
            eprintln!("lowering errors:");
            for error in ir_errors {
                eprintln!("- {error}");
                if let Some(help) = error.kind.help() {
                    eprintln!("    [ {help} ]");
                }
            }
        }
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
