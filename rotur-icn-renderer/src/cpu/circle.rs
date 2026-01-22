use rotur_icn_compiler::resolver::lir;
use rotur_icn_units::Vector;

pub fn distance(el: &lir::Circle, pos: Vector) -> f32 {
    let rel_pos = pos - el.centre;

    // TODO get rid of sqrt
    (rel_pos.length() - el.radius).abs()
}

pub fn test(el: &lir::Circle, pos: Vector) -> bool {
    distance(el, pos) < el.width
}
