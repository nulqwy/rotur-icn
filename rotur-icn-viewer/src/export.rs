use rotur_icn_units::Vector;

use crate::{
    error::EXIT_CODE_FOUND_ERRORS,
    file_tools::{pick_save_path, read, save},
    options::ExportOptions,
    ui::{choose_canvas_camera, display_diagnostics, process, render},
};

pub fn export(
    ExportOptions {
        help: _,
        icon: icon_path,
        overwrite,
        forbid_overwrite,
        dry,
        save: save_path,
        fit,
        pad,
        width,
        height,
        camera_x,
        camera_y,
        zoom,
        scale,
        background,
        error_abort,
        perf_process,
        perf_render,
        ast,
        hir,
        lir,
        chosen_sizes,
    }: ExportOptions,
) {
    let icon_save = pick_save_path(
        icon_path.as_deref(),
        save_path,
        // if dry mode is enabled, act as if overwrite is silently permitted
        overwrite | dry,
        forbid_overwrite && !dry,
    );

    let icon_src = read(icon_path.as_deref());
    let (icon, errors) = process(&icon_src, perf_process, (ast, hir, lir));

    if !errors.is_empty() {
        display_diagnostics(icon_path.as_deref(), &icon_src, &errors);
    }

    if !errors.is_empty() && error_abort {
        std::process::exit(EXIT_CODE_FOUND_ERRORS);
    }

    let (canvas, camera) = choose_canvas_camera(
        &icon,
        fit,
        pad,
        width.map(|w| Vector {
            x: w,
            y: height.unwrap_or(w),
        }),
        Vector {
            x: camera_x,
            y: camera_y,
        },
        chosen_sizes,
    );

    let (image, image_size) = render(&icon, canvas, scale, zoom, camera, background, perf_render);

    if !dry {
        save(icon_save.as_deref(), &image, image_size);
    }

    if !errors.is_empty() {
        std::process::exit(EXIT_CODE_FOUND_ERRORS)
    }
}
