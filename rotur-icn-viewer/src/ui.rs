use std::{path::Path, time::Instant};

use ansi_term::{Color, Style};
use codespan_reporting::{
    files::SimpleFile,
    term::termcolor::{ColorChoice, StandardStream},
};

use rotur_icn_pipeline::ProcessErrors;
use rotur_icn_resolver::lir;
use rotur_icn_units::{Colour, Vector};

use crate::{
    abort::abort,
    error::{EXIT_CODE_FAILED_DISPLAY_DIAGNOSTICS, FailureError},
};

pub fn process(
    src: &str,
    print_perf: bool,
    (print_ast, print_high_ir, print_low_ir): (bool, bool, bool),
) -> (lir::IconLir, ProcessErrors) {
    let (icon_low_ir, errors, start, end) = if !print_ast && !print_high_ir && !print_low_ir {
        let start = Instant::now();

        let (icon_low_ir, errors) = rotur_icn_pipeline::process_final(src);

        let end = Instant::now();

        (icon_low_ir, errors, start, end)
    } else {
        let start = Instant::now();

        let (icon_ast, icon_high_ir, icon_low_ir, errors) = rotur_icn_pipeline::process(src);

        let end = Instant::now();

        if print_ast {
            eprintln!("--- AST ---\n{icon_ast}");
        }

        if print_high_ir {
            eprintln!("--- HIR ---\n{icon_high_ir}");
        }

        if print_low_ir {
            eprintln!("--- LIR ---\n{icon_low_ir}");
        }

        (icon_low_ir, errors, start, end)
    };

    if print_perf {
        let perf = end - start;
        eprintln!(
            "Time taken to process the ICN: {:.3}μs",
            perf.as_secs_f64() * 1e6
        );
    }

    (icon_low_ir, errors)
}

pub fn display_diagnostics(file: Option<&Path>, src: &str, errors: &ProcessErrors) {
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

    let color = if errors.is_empty() {
        Color::Green
    } else {
        Color::Red
    };

    eprintln!(
        "{} {} {}",
        color.paint("Found"),
        Style::new().bold().paint(errors.len().to_string()),
        color.paint("errors"),
    );
}

pub fn choose_canvas_camera(
    icon: &lir::IconLir,
    fit: bool,
    pad: f32,
    canvas: Option<Vector>,
    camera: Vector,
    print: bool,
) -> (Vector, Vector) {
    let (canvas, camera) = rotur_icn_pipeline::choose_canvas_camera(icon, fit, pad, canvas, camera);

    if print {
        let half = canvas / 2.;
        let bl = camera - half;
        let tr = camera + half;

        eprintln!("Chosen canvas & camera: {canvas} {camera} ({bl} - {tr})");
    }

    (canvas, camera)
}

pub fn render(
    icon: &lir::IconLir,
    canvas: Vector,
    scale: f32,
    zoom: f32,
    camera: Vector,
    background: Colour,
    print_perf: bool,
) -> (Vec<u8>, (usize, usize)) {
    let start = Instant::now();

    let (img, img_size) =
        rotur_icn_pipeline::render(icon, canvas / zoom, scale * zoom, camera, background);

    let end = Instant::now();

    if print_perf {
        let perf = end - start;
        eprintln!(
            "Time taken to render the ICN: {:.3}ms",
            perf.as_secs_f64() * 1e3
        );
    }

    (img, img_size)
}
