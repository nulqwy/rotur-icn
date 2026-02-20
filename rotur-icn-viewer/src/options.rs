use std::{num::ParseIntError, path::PathBuf};

use gumdrop::Options;

use rotur_icn_units::{Colour, Number};

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
        Self::Gui(GuiOptions::default())
    }
}

#[derive(Debug, Clone, Options)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "not a state machine, but a flag store (partially)"
)]
pub struct ExportOptions {
    #[options(help = "print this message")]
    pub help: bool,

    #[options(free, help = "path to the ICN file to render (default: stdin)")]
    pub icon: Option<PathBuf>,

    #[options(
        no_short,
        help = "path to export a PNG to (default: filename.png)",
        meta = "PATH"
    )]
    pub save: Option<PathBuf>,

    #[options(
        short = "o",
        help = "confirm silently overwriting the destination path"
    )]
    pub overwrite: bool,

    #[options(short = "q", help = "forbid silently overwriting the destination path")]
    pub forbid_overwrite: bool,

    #[options(short = "F", help = "fit canvas & camera to icon's edges")]
    pub fit: bool,

    #[options(short = "P", help = "padding around canvas' edges", default = "0")]
    pub pad: f32,

    #[options(short = "W", help = "width of the canvas (default: 20)", meta = "PXs")]
    pub width: Option<f32>,

    #[options(
        short = "H",
        help = "height of the canvas (default: same as width)",
        meta = "PXs"
    )]
    pub height: Option<f32>,

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
        short = "Z",
        help = "zooms in the canvas",
        meta = "COEF",
        default = "1"
    )]
    pub zoom: Number,

    #[options(
        short = "S",
        help = "scales the image for a higher resolution",
        default = "25"
    )]
    pub scale: Number,

    #[options(
        short = "C",
        help = "set the background colour (8-char HEX, RGBA)",
        meta = "COL",
        parse(try_from_str = "parse_colour"),
        default = "00000000"
    )]
    pub background: Colour,

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

    #[options(no_short, help = "print the final chosen canvas & camera")]
    pub chosen_sizes: bool,
}

fn parse_colour(s: &str) -> Result<Colour, ParseIntError> {
    Ok(Colour::from_u32_with_alpha(u32::from_str_radix(s, 16)?))
}

#[derive(Debug, Clone, Default, Options)]
pub struct GuiOptions {}
