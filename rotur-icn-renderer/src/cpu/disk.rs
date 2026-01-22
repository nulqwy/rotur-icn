use rotur_icn_compiler::resolver::lir;
use rotur_icn_units::Vector;

pub fn distance_sq(el: &lir::Disk, pos: Vector) -> f32 {
    let rel_pos = pos - el.centre;

    rel_pos.length_sq()
}

pub fn test(el: &lir::Disk, pos: Vector) -> bool {
    distance_sq(el, pos) < el.radius * el.radius
}
