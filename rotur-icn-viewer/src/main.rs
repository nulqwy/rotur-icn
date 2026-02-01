use gumdrop::Options as _;

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
        ViewerMode::Export(opts) => export(opts),
    }
}
