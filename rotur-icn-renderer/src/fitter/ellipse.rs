use rotur_icn_compiler::resolver::lir;
use rotur_icn_units::Vector;

pub fn get_bounds(el: &lir::Ellipse) -> (Vector, Vector) {
    let pad = el.outline_width / 2.;

    let axis2 = el.axis.powi(2);
    let phase = axis2.x - axis2.y;
    let coef = el.direction.sin().powi(2);
    let offset = phase * coef;

    let bounds = Vector {
        x: axis2.x - offset,
        y: axis2.y + offset,
    }
    .sqrt();

    (el.centre - bounds - pad, el.centre + bounds + pad)
}
