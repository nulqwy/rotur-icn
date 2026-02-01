use gumdrop::Options as _;

use rotur_icn_units::Vector;

use options::ViewerOptions;

use crate::{export::export, options::ViewerMode};

mod abort;
mod error;
mod export;
mod options;

fn main() {
    let opts = ViewerOptions::parse_args_default_or_exit();

    match opts.mode() {
        ViewerMode::Gui(_) => todo!("GUI will not be ready soon"),
        ViewerMode::Export(opts) => {
            let height = opts.height();
            export(
                opts.icon.as_deref(),
                opts.save.as_deref(),
                opts.error_abort,
                Vector {
                    x: opts.width,
                    y: height,
                },
                opts.scale,
                Vector {
                    x: opts.camera_x,
                    y: opts.camera_y,
                },
                (opts.perf_process, opts.perf_render),
                (opts.ast, opts.hir, opts.lir),
            );
        }
    }
}
