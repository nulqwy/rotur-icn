use gumdrop::Options as _;

use options::ViewerOptions;

use crate::{check::check, export::export, options::ViewerMode};

mod abort;
mod check;
mod error;
mod export;
mod options;

fn main() {
    let opts = ViewerOptions::parse_args_default_or_exit();

    match opts.mode() {
        ViewerMode::Export(opts) => export(opts),
        ViewerMode::Check(opts) => check(opts),
        ViewerMode::Gui(_) => todo!("GUI will not be ready soon"),
    }
}
