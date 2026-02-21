use crate::{
    error::EXIT_CODE_FOUND_ERRORS,
    file_tools::read,
    options::CheckOptions,
    ui::{display_diagnostics, process},
};

pub fn check(
    CheckOptions {
        help: _,
        icon,
        perf_process,
        ast,
        hir,
        lir,
    }: CheckOptions,
) {
    let icon_src = read(icon.as_deref());
    let (_, errors) = process(&icon_src, perf_process, (ast, hir, lir));

    display_diagnostics(icon.as_deref(), &icon_src, &errors);

    if !errors.is_empty() {
        std::process::exit(EXIT_CODE_FOUND_ERRORS)
    }
}
