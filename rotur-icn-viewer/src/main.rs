use gumdrop::Options as _;

use check::check;
use export::export;
use options::{ViewerMode, ViewerOptions};

mod abort;
mod check;
mod error;
mod export;
mod file_tools;
mod options;
mod ui;

fn main() {
    let opts = ViewerOptions::parse_args_default_or_exit();

    match opts.mode() {
        ViewerMode::Export(opts) => export(opts),
        ViewerMode::Check(opts) => check(opts),
        ViewerMode::Gui(_) => todo!("GUI will not be ready soon"),
    }
}
