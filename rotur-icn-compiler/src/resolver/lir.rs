use rotur_icn_units::{Colour, Number, Vector};

#[derive(Debug, Clone)]
pub struct IconLir {
    pub elements: Vec<Element>,
}

#[derive(Debug, Clone)]
pub struct Element {
    pub colour: Colour,
    pub kind: ElementKind,
}

#[derive(Debug, Clone)]
pub enum ElementKind {
    Line(Line),
    Disk(Disk),
    Circle(Circle),
    Rectangle(Rectangle),
    Triangle(Triangle),
    Arc(Arc),
    Ellipse(Ellipse),
    Curve(Curve),
}

#[derive(Debug, Clone)]
pub struct Line {
    pub start: Vector,
    pub end: Vector,
    pub width: Number,
}

#[derive(Debug, Clone)]
pub struct Disk {
    pub centre: Vector,
    pub radius: Number,
}

#[derive(Debug, Clone)]
pub struct Circle {
    pub centre: Vector,
    pub radius: Number,
    pub width: Number,
}

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub bottom_left: Vector,
    pub sizes: Vector,
    pub filled: bool,
    pub outline_width: Number,
}

#[derive(Debug, Clone)]
pub struct Triangle {
    pub a: Vector,
    pub b: Vector,
    pub c: Vector,
    pub outline_width: Number,
}

#[derive(Debug, Clone)]
pub struct Arc {
    pub centre: Vector,
    pub radius: Number,
    pub width: Number,
    pub start_angle: Number,
    pub end_angle: Number,
}

#[derive(Debug, Clone)]
pub struct Ellipse {
    pub centre: Vector,
    pub sizes: Vector,
    pub direction: Number,
    pub outline_width: Number,
}

#[derive(Debug, Clone)]
pub struct Curve {
    pub start: Vector,
    pub control: Vector,
    pub end: Vector,
    pub width: Number,
}
