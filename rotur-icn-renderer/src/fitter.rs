use rotur_icn_resolver::lir;
use rotur_icn_units::Vector;

pub mod arc;
pub mod circle;
pub mod curve;
pub mod disk;
pub mod ellipse;
pub mod line;
pub mod rectangle;
pub mod triangle;

pub fn fit(icon: &lir::IconLir) -> FittedCanvas {
    let (bl, tr) = icon
        .elements
        .iter()
        .map(get_bounds)
        .reduce(combine_bounds)
        .unwrap_or((Vector::ZERO, Vector::ZERO));

    let size = tr - bl;
    let camera = bl + size / 2.;

    FittedCanvas { size, camera }
}

pub struct FittedCanvas {
    pub size: Vector,
    pub camera: Vector,
}

fn get_bounds(el: &lir::Element) -> (Vector, Vector) {
    match &el.kind {
        lir::ElementKind::Line(line) => line::get_bounds(line),
        lir::ElementKind::Disk(disk) => disk::get_bounds(disk),
        lir::ElementKind::Circle(circle) => circle::get_bounds(circle),
        lir::ElementKind::Rectangle(rectangle) => rectangle::get_bounds(rectangle),
        lir::ElementKind::Triangle(triangle) => triangle::get_bounds(triangle),
        lir::ElementKind::Arc(arc) => arc::get_bounds(arc),
        lir::ElementKind::Ellipse(ellipse) => ellipse::get_bounds(ellipse),
        lir::ElementKind::Curve(curve) => curve::get_bounds(curve),
    }
}

fn points_bounds(mut p: impl Iterator<Item = Vector>) -> (Vector, Vector) {
    let base = p.next().expect("there should be at least one point");

    extend_bound_many((base, base), p)
}

fn extend_bound_many(b: (Vector, Vector), p: impl Iterator<Item = Vector>) -> (Vector, Vector) {
    p.fold(b, extend_bound)
}

fn extend_bound(b: (Vector, Vector), p: Vector) -> (Vector, Vector) {
    (b.0.min(p), b.1.max(p))
}

fn combine_bounds(b: (Vector, Vector), other: (Vector, Vector)) -> (Vector, Vector) {
    (b.0.min(other.0), b.1.max(other.1))
}
