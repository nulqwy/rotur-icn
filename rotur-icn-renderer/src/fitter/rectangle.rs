use rotur_icn_compiler::resolver::lir;
use rotur_icn_units::Vector;

pub fn get_bounds(el: &lir::Rectangle) -> (Vector, Vector) {
    let pad = el.outline_width / 2.;
    (el.bottom_left - pad, el.bottom_left + el.sizes + pad)
}
