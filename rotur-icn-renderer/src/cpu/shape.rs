use rotur_icn_resolver::lir;
use rotur_icn_units::Vector;

use super::{arc, circle, colour::InternalColour, curve, disk, ellipse, line, rectangle, triangle};

pub trait Shape {
    fn test(&self, pos: Vector) -> bool;

    fn possibly_within(&self, bounds: (Vector, Vector)) -> bool;
}

pub struct ComputedShapesBundle {
    pub shapes: Vec<ComputedShape>,
}

pub struct ComputedShape {
    pub colour: InternalColour,
    pub kind: ComputedShapeKind,
}

pub enum ComputedShapeKind {
    Line(line::Line),
    Disk(disk::Disk),
    Circle(circle::Circle),
    Rectangle(rectangle::Rectangle),
    Triangle(triangle::Triangle),
    Arc(arc::Arc),
    Ellipse(ellipse::Ellipse),
    Curve(curve::Curve),
}

impl ComputedShapesBundle {
    pub fn new(icon: &lir::IconLir) -> Self {
        Self {
            shapes: icon.elements.iter().map(ComputedShape::new).collect(),
        }
    }
}

impl ComputedShape {
    pub fn new(el: &lir::Element) -> Self {
        Self {
            colour: el.colour.into(),
            kind: ComputedShapeKind::new(&el.kind),
        }
    }

    pub fn test_with_colour(&self, pos: Vector) -> Option<InternalColour> {
        self.test(pos).then_some(self.colour)
    }
}

impl ComputedShapeKind {
    pub fn new(kind: &lir::ElementKind) -> Self {
        match kind {
            lir::ElementKind::Line(line) => ComputedShapeKind::Line(line::Line::new(line)),
            lir::ElementKind::Disk(disk) => ComputedShapeKind::Disk(disk::Disk::new(disk)),
            lir::ElementKind::Circle(circle) => {
                ComputedShapeKind::Circle(circle::Circle::new(circle))
            }
            lir::ElementKind::Rectangle(rectangle) => {
                ComputedShapeKind::Rectangle(rectangle::Rectangle::new(rectangle))
            }
            lir::ElementKind::Triangle(triangle) => {
                ComputedShapeKind::Triangle(triangle::Triangle::new(triangle))
            }
            lir::ElementKind::Arc(arc) => ComputedShapeKind::Arc(arc::Arc::new(arc)),
            lir::ElementKind::Ellipse(ellipse) => {
                ComputedShapeKind::Ellipse(ellipse::Ellipse::new(ellipse))
            }
            lir::ElementKind::Curve(curve) => ComputedShapeKind::Curve(curve::Curve::new(curve)),
        }
    }
}

impl Shape for ComputedShape {
    fn test(&self, pos: Vector) -> bool {
        self.kind.test(pos)
    }

    fn possibly_within(&self, bounds: (Vector, Vector)) -> bool {
        self.kind.possibly_within(bounds)
    }
}

impl Shape for ComputedShapeKind {
    fn test(&self, pos: Vector) -> bool {
        match self {
            ComputedShapeKind::Line(line) => line.test(pos),
            ComputedShapeKind::Disk(disk) => disk.test(pos),
            ComputedShapeKind::Circle(circle) => circle.test(pos),
            ComputedShapeKind::Rectangle(rectangle) => rectangle.test(pos),
            ComputedShapeKind::Triangle(triangle) => triangle.test(pos),
            ComputedShapeKind::Arc(arc) => arc.test(pos),
            ComputedShapeKind::Ellipse(ellipse) => ellipse.test(pos),
            ComputedShapeKind::Curve(curve) => curve.test(pos),
        }
    }

    fn possibly_within(&self, bounds: (Vector, Vector)) -> bool {
        match self {
            ComputedShapeKind::Line(line) => line.possibly_within(bounds),
            ComputedShapeKind::Disk(disk) => disk.possibly_within(bounds),
            ComputedShapeKind::Circle(circle) => circle.possibly_within(bounds),
            ComputedShapeKind::Rectangle(rectangle) => rectangle.possibly_within(bounds),
            ComputedShapeKind::Triangle(triangle) => triangle.possibly_within(bounds),
            ComputedShapeKind::Arc(arc) => arc.possibly_within(bounds),
            ComputedShapeKind::Ellipse(ellipse) => ellipse.possibly_within(bounds),
            ComputedShapeKind::Curve(curve) => curve.possibly_within(bounds),
        }
    }
}
