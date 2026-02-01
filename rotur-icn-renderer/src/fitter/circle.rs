use rotur_icn_compiler::resolver::lir;
use rotur_icn_units::Vector;

pub fn get_bounds(el: &lir::Circle) -> (Vector, Vector) {
    (
        el.centre - el.radius - el.width / 2.,
        el.centre + el.radius + el.width / 2.,
    )
}
