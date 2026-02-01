use rotur_icn_compiler::resolver::lir;
use rotur_icn_units::Vector;

mod arc;
mod circle;
mod curve;
mod disk;
mod ellipse;
mod line;
mod rectangle;
mod triangle;

pub fn fit(icon: &lir::IconLir) -> (Vector, Vector) {
    let (bl, tr) = icon
        .elements
        .iter()
        .map(get_bounds)
        .reduce(extend_bound)
        .unwrap_or((Vector::ZERO, Vector::ZERO));

    let canvas_size = tr - bl;
    let camera = bl + canvas_size / 2.;
    (canvas_size, camera)
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

fn points_bounds(p: &[Vector]) -> (Vector, Vector) {
    assert!(!p.is_empty(), "no points have no bounds");
    (min_most(p).unwrap(), max_most(p).unwrap())
}

fn extend_bounds(b: (Vector, Vector), p: &[Vector]) -> (Vector, Vector) {
    p.iter()
        .copied()
        .fold(b, |(min, max), p| (min.min(p), max.max(p)))
}

fn extend_bound(b: (Vector, Vector), other: (Vector, Vector)) -> (Vector, Vector) {
    (b.0.min(other.0), b.1.max(other.1))
}

fn min_most(p: &[Vector]) -> Option<Vector> {
    p.iter().copied().reduce(|m, p| m.min(p))
}

fn max_most(p: &[Vector]) -> Option<Vector> {
    p.iter().copied().reduce(|m, p| m.max(p))
}
