use rotur_icn_compiler::resolver::lir;
use rotur_icn_units::Vector;

// from https://www.shadertoy.com/view/MlKcDD
pub fn distance_sq(el: &lir::Curve, pos: Vector) -> f32 {
    todo!()
}

pub fn test(el: &lir::Curve, pos: Vector) -> bool {
    distance_sq(el, pos) <= el.width * el.width / 4.
}
