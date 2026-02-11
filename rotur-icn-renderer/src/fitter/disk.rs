use rotur_icn_resolver::lir;
use rotur_icn_units::Vector;

pub fn get_bounds(el: &lir::Disk) -> (Vector, Vector) {
    (el.centre - el.radius, el.centre + el.radius)
}
