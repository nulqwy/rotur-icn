use std::{
    io::{self, Write},
    path::Path,
    time::Instant,
};

use ansi_term::{Color, Style};
use codespan_reporting::{
    files::SimpleFile,
    term::termcolor::{ColorChoice, StandardStream},
};
use rotur_icn_compiler::resolver::lir;
use rotur_icn_pipeline::Errors;
use rotur_icn_renderer::cpu::Renderer;
use rotur_icn_units::{Colour, Vector};

use crate::{
    abort::abort,
    error::{
        EXIT_CODE_FAILED_DISPLAY_DIAGNOSTICS, EXIT_CODE_FAILED_OPEN_FILE,
        EXIT_CODE_FAILED_READ_FILE, EXIT_CODE_FAILED_WRITE_FILE, EXIT_CODE_FOUND_ERRORS,
        FailureError,
    },
};

#[allow(clippy::too_many_arguments)]
pub fn export(
    src_path: Option<&Path>,
    save_path: Option<&Path>,
    abort_on_error: bool,
    canvas: Vector,
    scale: f32,
    camera: Vector,
    print_perf: (bool, bool),
    print_debug: (bool, bool, bool),
) {
    let icon_src = read(src_path);
    let (icon, errors) = process(&icon_src, print_perf.0, print_debug);

    if !errors.is_empty() {
        display_diagnostics(src_path, &icon_src, &errors);
    }

    if !errors.is_empty() && abort_on_error {
        std::process::exit(EXIT_CODE_FOUND_ERRORS);
    }

    let (image, image_size) = render(&icon, canvas, scale, camera, print_perf.1);
    save(save_path, &image, image_size);

    if !errors.is_empty() {
        std::process::exit(EXIT_CODE_FOUND_ERRORS)
    }
}

fn process(src: &str, print_perf: bool, print_debug: (bool, bool, bool)) -> (lir::IconLir, Errors) {
    let start = Instant::now();

    let (icon_ast, icon_hir, icon_lir, errors) = rotur_icn_pipeline::process(src);

    let end = Instant::now();

    if print_debug.0 {
        eprintln!("--- AST ---\n{icon_ast}");
    }

    if print_debug.1 {
        eprintln!("--- HIR ---\n{icon_hir}");
    }

    if print_debug.2 {
        eprintln!("--- LIR ---\n{icon_lir}");
    }

    if print_perf {
        let perf = end - start;
        eprintln!(
            "Time taken to process the ICN: {:.3}υs",
            perf.as_secs_f64() * 1e6
        );
    }

    (icon_lir, errors)
}

fn render(
    icon: &lir::IconLir,
    canvas: Vector,
    scale: f32,
    camera: Vector,
    print_perf: bool,
) -> (Vec<u8>, (usize, usize)) {
    let mut renderer = Renderer::new(canvas, scale, camera, Colour::ZERO);
    renderer.load(icon);

    let (mut buf, buf_size) = renderer.new_buf();

    let start = Instant::now();
    renderer.render(&mut buf);
    let end = Instant::now();

    if print_perf {
        let perf = end - start;
        eprintln!(
            "Time taken to render the ICN: {:.3}ms",
            perf.as_secs_f64() * 1e3
        );
    }

    (buf, buf_size)
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

    eprintln!(
        "{} {} {}",
        Color::Red.paint("Found"),
        Style::new().bold().paint(errors.len().to_string()),
        Color::Red.paint("errors"),
    );
}

fn read(path: Option<&Path>) -> String {
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

fn save(path: Option<&Path>, buf: &[u8], buf_size: (usize, usize)) {
    let mut writer = if let Some(file) = path {
        Box::new(
            std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(file)
                .unwrap_or_else(|err| {
                    abort(&FailureError::OpenFile(err), EXIT_CODE_FAILED_OPEN_FILE)
                }),
        ) as Box<dyn Write>
    } else {
        Box::new(std::io::stdout()) as Box<dyn Write>
    };

    write!(
        writer,
        "P7\n\
        WIDTH {}\n\
        HEIGHT {}\n\
        DEPTH 4\n\
        MAXVAL 255\n\
        TUPLTYPE RGB_ALPHA\n\
        ENDHDR\n",
        buf_size.0, buf_size.1
    )
    .unwrap_or_else(|err| abort(&FailureError::WriteFile(err), EXIT_CODE_FAILED_WRITE_FILE));

    writer
        .write_all(buf)
        .unwrap_or_else(|err| abort(&FailureError::WriteFile(err), EXIT_CODE_FAILED_WRITE_FILE));
}
