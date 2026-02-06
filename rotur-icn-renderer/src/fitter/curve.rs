use rotur_icn_compiler::resolver::lir;
use rotur_icn_units::Vector;

use super::{extend_bound, points_bounds};

pub fn get_bounds(el: &lir::Curve) -> (Vector, Vector) {
    let ba = el.start - el.control;
    let bc = el.end - el.control;
    let alpha = ba + bc;
    let beta = -2. * ba;

    let bounds = points_bounds([el.start, el.end].into_iter());

    let q = ba / alpha;

    let (bl, tr) = [q.x, q.y]
        .into_iter()
        .filter_map(|t| compute_p(alpha, beta, el.start, t))
        .fold(bounds, extend_bound);

    let pad = el.width / 2.;

    (bl - pad, tr + pad)
}

fn compute_p(alpha: Vector, beta: Vector, start: Vector, t: f32) -> Option<Vector> {
    (0. < t && t < 1.).then(|| start + t * (beta + alpha * t))
}
