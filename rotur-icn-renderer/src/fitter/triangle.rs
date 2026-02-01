use rotur_icn_compiler::resolver::lir;
use rotur_icn_units::Vector;

use super::points_bounds;

pub fn get_bounds(el: &lir::Triangle) -> (Vector, Vector) {
    let (bl, tr) = points_bounds(&[el.a, el.b, el.c]);
    let pad = el.outline_width / 2.;
    (bl - pad, tr + pad)
}
