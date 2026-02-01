use std::path::PathBuf;

use gumdrop::Options;

use rotur_icn_units::Number;

#[derive(Debug, Clone, Options)]
pub struct ViewerOptions {
    #[options(help = "print this message")]
    pub help: bool,

    #[options(command)]
    mode: Option<ViewerMode>,
}

impl ViewerOptions {
    pub fn mode(self) -> ViewerMode {
        self.mode.unwrap_or_default()
    }
}

#[derive(Debug, Clone, Options)]
pub enum ViewerMode {
    Export(ExportOptions),
    Gui(GuiOptions),
}

impl Default for ViewerMode {
    fn default() -> Self {
        Self::Gui(Default::default())
    }
}

#[derive(Debug, Clone, Options)]
pub struct ExportOptions {
    #[options(help = "print this message")]
    pub help: bool,

    #[options(free, help = "path to the ICN file to render (default: stdin)")]
    pub icon: Option<PathBuf>,

    #[options(
        no_short,
        help = "path to export a PAM to (default: stdout)",
        meta = "PATH"
    )]
    pub save: Option<PathBuf>,

    #[options(
        short = "W",
        help = "width of the canvas",
        meta = "PXs",
        default = "20"
    )]
    pub width: f32,

    #[options(
        short = "H",
        help = "height of the canvas (default: same as width)",
        meta = "PXs"
    )]
    height: Option<f32>,

    #[options(
        short = "X",
        help = "position of the camera",
        meta = "PXs",
        default = "0"
    )]
    pub camera_x: Number,

    #[options(
        short = "Y",
        help = "position of the camera",
        meta = "PXs",
        default = "0"
    )]
    pub camera_y: Number,

    #[options(
        short = "S",
        help = "scales image for a higher resolution",
        default = "10"
    )]
    pub scale: Number,

    #[options(no_short, help = "abort, if any errors in the ICN were found")]
    pub error_abort: bool,

    #[options(no_short, help = "print the time taken to process the ICN")]
    pub perf_process: bool,

    #[options(no_short, help = "print the time taken to render the ICN")]
    pub perf_render: bool,

    #[options(no_short, help = "print the AST representation of the ICN")]
    pub ast: bool,

    #[options(no_short, help = "print the HIR representation of the ICN")]
    pub hir: bool,

    #[options(no_short, help = "print the LIR representation of the ICN")]
    pub lir: bool,
}

impl ExportOptions {
    pub fn height(&self) -> f32 {
        self.height.unwrap_or(self.width)
    }
}

#[derive(Debug, Clone, Default, Options)]
pub struct GuiOptions {}
