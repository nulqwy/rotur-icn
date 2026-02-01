use rotur_icn_compiler::resolver::lir;
use rotur_icn_units::Vector;

use super::{extend_bounds, points_bounds};

pub fn get_bounds(el: &lir::Arc) -> (Vector, Vector) {
    let start = compute_p(el, el.start_angle).unwrap();
    let end = compute_p(el, el.end_angle).unwrap();
    let bounds = points_bounds(&[start, end]);

    let (bl, tr) = [
        0.,
        std::f32::consts::FRAC_PI_2,
        std::f32::consts::PI,
        std::f32::consts::PI + std::f32::consts::FRAC_PI_2,
    ]
    .into_iter()
    .filter_map(|t| compute_p(el, t))
    .fold(bounds, |b, p| extend_bounds(b, &[p]));

    let pad = el.width / 2.;
    (bl - pad, tr + pad)
}

fn compute_p(arc: &lir::Arc, t: f32) -> Option<Vector> {
    (arc.start_angle <= t && t <= arc.end_angle).then_some(Vector::new_from_length(arc.radius, t))
}
