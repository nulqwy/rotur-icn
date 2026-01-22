use std::{
    io::{self, Write as _},
    path::Path,
    time::Instant,
};

use ansi_term::{Colour, Style};
use codespan_reporting::{
    files::SimpleFile,
    term::termcolor::{ColorChoice, StandardStream},
};
use gumdrop::Options as _;

use error::FailureError;
use options::ViewerOptions;
use rotur_icn_pipeline::{Errors, process};
use rotur_icn_renderer::cpu::Renderer;

mod error;
// mod gui;
mod options;

const BASE_ERROR_EXIT_CODE: i32 = 90;
const EXIT_CODE_FOUND_ERRORS: i32 = BASE_ERROR_EXIT_CODE;
const EXIT_CODE_FAILED_OPEN_FILE: i32 = BASE_ERROR_EXIT_CODE + 1;
const EXIT_CODE_FAILED_READ_FILE: i32 = BASE_ERROR_EXIT_CODE + 2;
const EXIT_CODE_FAILED_DISPLAY_DIAGNOSTICS: i32 = BASE_ERROR_EXIT_CODE + 3;

fn main() {
    let opts = ViewerOptions::parse_args_default_or_exit();

    let icn = read_file(opts.icon.as_deref());

    let start_process = Instant::now();
    let (icon_ast, icon_hir, icon_lir, errors) = process(&icn);
    let end_process = Instant::now();

    if opts.ast {
        println!("--- AST ---\n{icon_ast}");
    }

    if opts.hir {
        println!("--- HIR ---\n{icon_hir}");
    }

    if opts.lir {
        println!("--- LIR ---\n{icon_lir}");
    }

    if !errors.is_empty() {
        display_diagnostics(opts.icon.as_deref(), &icn, &errors);

        eprintln!(
            "{} {} {}",
            Colour::Red.paint("Found"),
            Style::new().bold().paint(errors.len().to_string()),
            Colour::Red.paint("errors"),
        );
    }

    if opts.perf_process {
        let perf = end_process - start_process;
        println!(
            "Time taken to process the ICN: {:.3}υs",
            perf.as_secs_f64() * 1e6
        );
    }

    if !errors.is_empty() && opts.error_abort {
        std::process::exit(EXIT_CODE_FOUND_ERRORS)
    }

    let buf_size = (opts.width, opts.height.unwrap_or(opts.width));
    let mut icon_buf = Renderer::new_buf(buf_size);

    let mut renderer = Renderer::default();
    renderer.set_buf(&mut icon_buf, buf_size);
    renderer.icon = Some(&icon_lir);

    let start_render = Instant::now();
    renderer.render();
    let end_render = Instant::now();

    if opts.perf_render {
        let perf = end_render - start_render;
        println!(
            "Time taken to render the ICN: {:.3}ms",
            perf.as_secs_f64() * 1e3
        );
    }

    save_buf(&opts.export, &icon_buf, buf_size);

    if !errors.is_empty() {
        std::process::exit(EXIT_CODE_FOUND_ERRORS)
    }
}

fn read_file(path: Option<&Path>) -> String {
    if let Some(path) = path {
        io::read_to_string(
            std::fs::File::open(path).unwrap_or_else(|err| {
                abort(&FailureError::OpenFile(err), EXIT_CODE_FAILED_OPEN_FILE)
            }),
        )
    } else {
        io::read_to_string(io::stdin())
    }
    .unwrap_or_else(|err| abort(&FailureError::ReadFile(err), EXIT_CODE_FAILED_READ_FILE))
}

fn save_buf(path: &Path, buf: &[u8], buf_size: (usize, usize)) {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .unwrap();

    file.lock().unwrap();

    let buf_no_alpha = buf
        .iter()
        .copied()
        .enumerate()
        .filter(|(i, _)| i % 4 != 3)
        .map(|(_, b)| b)
        .collect::<Vec<_>>();

    writeln!(file, "P6 {} {} 255", buf_size.0, buf_size.1).unwrap();
    file.write_all(&buf_no_alpha).unwrap();
}

fn display_diagnostics(file: Option<&Path>, src: &str, errors: &Errors) {
    let file = SimpleFile::new(
        file.map_or("<stdin>".into(), |p| {
            p.file_name().unwrap().to_string_lossy()
        }),
        src,
    );

    let writer = StandardStream::stderr(ColorChoice::Auto);
    let config = codespan_reporting::term::Config::default();

    for diag in errors.into_diagnostics() {
        codespan_reporting::term::emit_to_io_write(&mut writer.lock(), &config, &file, &diag)
            .unwrap_or_else(|err| {
                abort(
                    &FailureError::DisplayDiagnostics(err),
                    EXIT_CODE_FAILED_DISPLAY_DIAGNOSTICS,
                )
            });
    }
}

fn abort(msg: &impl std::error::Error, code: i32) -> ! {
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
