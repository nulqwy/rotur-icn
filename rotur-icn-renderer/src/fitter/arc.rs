use rotur_icn_resolver::lir;
use rotur_icn_units::Vector;

use super::{extend_bound, points_bounds};

pub fn get_bounds(el: &lir::Arc) -> (Vector, Vector) {
    let start = Vector::new_from_length(el.radius, el.start_angle);
    let end = Vector::new_from_length(el.radius, el.end_angle);
    let bounds = points_bounds([start, end].into_iter());

    let (bl, tr) = [
        0.,
        std::f32::consts::FRAC_PI_2,
        std::f32::consts::PI,
        std::f32::consts::PI + std::f32::consts::FRAC_PI_2,
    ]
    .into_iter()
    .flat_map(|a| [a, -a].into_iter()) // FIXME a very lazy solution
    .filter_map(|t| compute_p(el, t))
    .fold(bounds, extend_bound);

    let pad = el.width / 2.;
    (el.centre + bl - pad, el.centre + tr + pad)
}

fn compute_p(arc: &lir::Arc, t: f32) -> Option<Vector> {
    (arc.start_angle < t && t < arc.end_angle).then(|| Vector::new_from_length(arc.radius, t))
}
