use rotur_icn_compiler::resolver::lir;
use rotur_icn_units::Vector;

pub fn test(el: &lir::Disk, pos: Vector) -> bool {
    let rel_pos = pos - el.centre;

    rel_pos.length_sq() < el.radius_sq
}
