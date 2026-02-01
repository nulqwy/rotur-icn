use rotur_icn_compiler::resolver::lir;
use rotur_icn_units::Vector;

pub fn get_bounds(el: &lir::Curve) -> (Vector, Vector) {
    let ab = el.control - el.start;
    let bc = el.end - el.control;
    let gamma = bc - ab;

    let (min_x, max_x) = get_bounds_axis(el.start.x, el.end.x, ab.x, gamma.x);
    let (min_y, max_y) = get_bounds_axis(el.start.y, el.end.y, ab.y, gamma.y);

    let pad = el.width / 2.;

    (
        Vector { x: min_x, y: min_y } - pad,
        Vector { x: max_x, y: max_y } + pad,
    )
}

fn get_bounds_axis(a: f32, c: f32, ab: f32, gamma: f32) -> (f32, f32) {
    let signs_differ = ab.is_sign_positive() ^ gamma.is_sign_positive();
    let less = -ab < gamma;

    if !(signs_differ && less) {
        min_max(a, c)
    } else {
        let t = a - ab.powi(2) / gamma;
        min_max_many(&[a, c, t])
    }
}

/// Sort a and b to asc order
fn min_max(a: f32, b: f32) -> (f32, f32) {
    if a < b { (a, b) } else { (b, a) }
}

fn min_max_many(n: &[f32]) -> (f32, f32) {
    assert!(!n.is_empty(), "empty slice has no min-max");

    n[1..]
        .iter()
        .copied()
        .fold((n[0], n[0]), |(min, max), n| (min.min(n), max.max(n)))
}
