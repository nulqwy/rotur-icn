use std::f32;

use rotur_icn_compiler::resolver::lir;
use rotur_icn_units::Vector;

pub fn distance_sq(el: &lir::Line, pos: Vector) -> f32 {
    debug_assert_ne!(
        el.start, el.end,
        "equal points in the line should be resolved to a dot"
    );

    let ab = el.end - el.start;

    let ap = pos - el.start;
    let bp = pos - el.end;

    let ab_dot = ab.dot(ap);

    let a_closer = ab_dot.is_sign_negative();
    let b_closer = ab_dot > ab.length_sq();

    match (a_closer, b_closer) {
        (true, false) => ap.length_sq(),
        (false, true) => bp.length_sq(),
        (false, false) => ab.cross(ap).powi(2) / ab.length_sq(),
        (true, true) => unreachable!("point cannot be outside at both places"),
    }
}

pub fn test(el: &lir::Line, pos: Vector) -> bool {
    distance_sq(el, pos) <= el.width * el.width / 4.
}
