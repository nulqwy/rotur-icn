use std::f32;

use rotur_icn_compiler::resolver::lir;
use rotur_icn_units::Vector;

pub fn distance_sq(el: &lir::Line, pos: Vector) -> f32 {
    let ab = el.end - el.start;

    let ap = pos - el.start;
    let bp = pos - el.end;

    let ab_dot = ab.dot_product(ap);

    let a_closer = ab_dot.is_sign_negative();
    let b_closer = ab_dot > ab.length_sq();

    match (a_closer, b_closer) {
        (true, false) => ap.length_sq(),
        (false, true) => bp.length_sq(),
        (false, false) => {
            let ab_normal = ab.rotate_90_cc();
            let ab_norm_dot = ab_normal.dot_product(ap);
            ab_norm_dot * ab_norm_dot / ab_normal.length_sq()
        }
        (true, true) => unreachable!("point cannot be outside at both places"),
    }
}

pub fn test(el: &lir::Line, pos: Vector) -> bool {
    distance_sq(el, pos) <= el.width * el.width / 4.
}
