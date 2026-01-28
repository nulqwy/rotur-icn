use rotur_icn_compiler::resolver::lir;
use rotur_icn_units::Vector;

pub fn distance(el: &lir::Circle, pos: Vector) -> f32 {
    let rel_pos = pos - el.centre;

    (rel_pos.length() - el.radius).abs()
}

// TODO rewrite in terms of range checking, which would allow to remove the sqrt from the calcs
pub fn test(el: &lir::Circle, pos: Vector) -> bool {
    distance(el, pos) <= el.width / 2.
}
