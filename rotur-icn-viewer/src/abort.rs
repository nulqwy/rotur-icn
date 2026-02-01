use ansi_term::{Colour, Style};

pub fn abort(msg: &impl std::error::Error, code: i32) -> ! {
    eprintln!(
        "{}: {}",
        Colour::Red.paint("Aborting due to a fatal error"),
        Style::new().bold().paint(msg.to_string())
    );

    let mut source = msg.source();
    while let Some(source_) = source {
        eprintln!("{} {source_}", Colour::Red.paint(" |\n | ->"));
        source = source_.source();
    }

    std::process::exit(code)
}
