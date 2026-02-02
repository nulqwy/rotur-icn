use rotur_icn_syntax::lexer::token;
use rotur_icn_units::{Colour, Number, Vector};

#[derive(Debug, Clone)]
pub struct IconHir {
    pub operations: Vec<Operation>,
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub cmd_pos: token::Pos,
    pub cmd_index: usize,
    pub kind: OperationKind,
}

#[derive(Debug, Clone)]
pub enum OperationKind {
    SetWidth(SetWidth),
    SetColour(SetColour),
    DrawLine(DrawLine),
    ContinueLine(ContinueLine),
    DrawDisk(DrawDisk),
    DrawRectangle(DrawRectangle),
    DrawTriangle(DrawTriangle),
    MoveCentre(MoveCentre),
    ResetCentre(ResetCentre),
    DrawArc(DrawArc),
    DrawEllipse(DrawEllipse),
    DrawCurve(DrawCurve),
}

#[derive(Debug, Clone)]
pub struct SetWidth {
    pub value: Number,
}

impl SetWidth {
    pub const NAME: &str = "w";
}

#[derive(Debug, Clone)]
pub struct SetColour {
    pub value: Colour,
}

impl SetColour {
    pub const NAME: &str = "c";
}

#[derive(Debug, Clone)]
pub struct DrawLine {
    pub start: Vector,
    pub end: Vector,
}

impl DrawLine {
    pub const NAME: &str = "line";
}

#[derive(Debug, Clone)]
pub struct ContinueLine {
    pub next: Vector,
}

impl ContinueLine {
    pub const NAME: &str = "cont";
}

#[derive(Debug, Clone)]
pub struct DrawDisk {
    pub centre: Vector,
}

impl DrawDisk {
    pub const NAME: &str = "dot";
}

#[derive(Debug, Clone)]
pub struct DrawRectangle {
    pub centre: Vector,
    pub sizes: Vector,
    pub filled: bool,
}

impl DrawRectangle {
    pub const NAME_HOLLOW: &str = "square";
    pub const NAME_FILLED: &str = "rect";
}

#[derive(Debug, Clone)]
pub struct DrawTriangle {
    pub a: Vector,
    pub b: Vector,
    pub c: Vector,
}

impl DrawTriangle {
    pub const NAME: &str = "tri";
}

#[derive(Debug, Clone)]
pub struct MoveCentre {
    pub change: Vector,
}

impl MoveCentre {
    pub const NAME: &str = "move";
}

#[derive(Debug, Clone)]
pub struct ResetCentre;

impl ResetCentre {
    pub const NAME: &str = "back";
}

#[derive(Debug, Clone)]
pub struct DrawArc {
    pub centre: Vector,
    pub radius: Number,
    pub direction: Number,
    pub arm_angle: Number,
}

impl DrawArc {
    pub const NAME: &str = "cutcircle";
}

#[derive(Debug, Clone)]
pub struct DrawEllipse {
    pub centre: Vector,
    pub width: Number,
    pub ratio: Number,
    pub direction: Number,
}

impl DrawEllipse {
    pub const NAME: &str = "ellipse";
}

#[derive(Debug, Clone)]
pub struct DrawCurve {
    pub start: Vector,
    pub control: Vector,
    pub end: Vector,
}

impl DrawCurve {
    pub const NAME: &str = "curve";
}
