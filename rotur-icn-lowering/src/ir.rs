pub struct IconIr {
    pub instructions: Vec<Instruction>,
}

pub enum Instruction {
    SetWidth(SetWidth),
    SetColour(SetColour),
    DrawLine(DrawLine),
    ContinueLine(ContinueLine),
    DrawDot(DrawDot),
    DrawRectangle(DrawRectangle),
    DrawTriangle(DrawTriangle),
    MoveCentre(MoveCentre),
    ResetCentre(ResetCentre),
    DrawArc(DrawArc),
    DrawEllipse(DrawEllipse),
    DrawCurve(DrawCurve),
}

pub struct SetWidth {
    pub value: f64,
}

impl SetWidth {
    pub const NAME: &str = "w";
}

pub struct SetColour {
    pub value: Colour,
}

impl SetColour {
    pub const NAME: &str = "c";
}

pub struct DrawLine {
    pub start: Vector,
    pub end: Vector,
}

impl DrawLine {
    pub const NAME: &str = "line";
}

pub struct ContinueLine {
    pub next: Vector,
}

impl ContinueLine {
    pub const NAME: &str = "cont";
}

pub struct DrawDot {
    pub pos: Vector,
}

impl DrawDot {
    pub const NAME: &str = "dot";
}

pub struct DrawRectangle {
    pub centre: Vector,
    pub sizes: Vector,
    pub filled: bool,
}

impl DrawRectangle {
    pub const NAME_HOLLOW: &str = "square";
    pub const NAME_FILLED: &str = "rect";
}

pub struct DrawTriangle {
    pub a: Vector,
    pub b: Vector,
    pub c: Vector,
}

impl DrawTriangle {
    pub const NAME: &str = "tri";
}

pub struct MoveCentre {
    pub change: Vector,
}

impl MoveCentre {
    pub const NAME: &str = "move";
}

pub struct ResetCentre;

impl ResetCentre {
    pub const NAME: &str = "back";
}

pub struct DrawArc {
    pub centre: Vector,
    pub radius: f64,
    pub start_angle: f64,
    pub end_angle: f64,
}

impl DrawArc {
    pub const NAME: &str = "cutcircle";
}

pub struct DrawEllipse {
    pub centre: Vector,
    pub sizes: Vector,
    pub facing: f64,
}

impl DrawEllipse {
    pub const NAME: &str = "ellipse";
}

pub struct DrawCurve {
    pub start: Vector,
    pub end: Vector,
    pub control: Vector,
}

impl DrawCurve {
    pub const NAME: &str = "curve";
}

pub struct Colour {
    pub b: u8,
    pub g: u8,
    pub r: u8,
}

impl Default for Colour {
    fn default() -> Self {
        Self {
            r: 0xff,
            g: 0,
            b: 0xff,
        }
    }
}

pub struct Vector {
    pub x: f64,
    pub y: f64,
}
