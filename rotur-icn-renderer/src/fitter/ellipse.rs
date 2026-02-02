use rotur_icn_compiler::resolver::lir;
use rotur_icn_units::Vector;

use super::points_bounds;

pub fn get_bounds(el: &lir::Ellipse) -> (Vector, Vector) {
    let pad = el.outline_width / 2.;
    let bl = (el.centre - el.axis - pad).rotate(el.direction);
    let tr = (el.centre + el.axis + pad).rotate(el.direction);
    let br = el
        .centre
        .conj_add(el.axis)
        .conj_add(Vector::new(pad))
        .rotate(el.direction);
    let tl = el
        .centre
        .conj_add(-el.axis)
        .conj_add(-Vector::new(pad))
        .rotate(el.direction);

    points_bounds(&[bl, tr, br, tl])
}
