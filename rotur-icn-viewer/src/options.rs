use std::path::PathBuf;

use gumdrop::Options;

#[derive(Debug, Clone, Options)]
pub struct ViewerOptions {
    #[options(free, help = "path to the icon file to render")]
    pub icon: Option<PathBuf>,

    #[options(no_short, help = "path to save the exported PPM to", meta = "PATH")]
    pub export: PathBuf,

    #[options(
        short = "W",
        help = "width of the canvas",
        meta = "PXs",
        default = "512"
    )]
    pub width: usize,

    #[options(
        short = "H",
        help = "height of the canvas (default: same as width)",
        meta = "PXs"
    )]
    pub height: Option<usize>,

    #[options(help = "print this message")]
    pub help: bool,

    #[options(no_short, help = "abort, if any errors in the ICN were found")]
    pub error_abort: bool,

    #[options(no_short, help = "print the time taken to process the ICN")]
    pub perf_process: bool,

    #[options(no_short, help = "print the time taken to render the ICN")]
    pub perf_render: bool,

    #[options(no_short, help = "print AST representation of the ICN")]
    pub ast: bool,

    #[options(no_short, help = "print HIR representation of the ICN")]
    pub hir: bool,

    #[options(no_short, help = "print LIR representation of the ICN")]
    pub lir: bool,
}
