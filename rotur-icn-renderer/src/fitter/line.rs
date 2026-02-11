use rotur_icn_resolver::lir;
use rotur_icn_units::Vector;

use super::points_bounds;

pub fn get_bounds(el: &lir::Line) -> (Vector, Vector) {
    let (bl, tr) = points_bounds([el.start, el.end].into_iter());
    let pad = el.width / 2.;
    (bl - pad, tr + pad)
}
