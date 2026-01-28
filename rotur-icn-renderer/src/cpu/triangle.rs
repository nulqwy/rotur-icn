use rotur_icn_compiler::resolver::lir;
use rotur_icn_units::Vector;

pub fn distance_sq(el: &lir::Triangle, pos: Vector) -> f32 {
    debug_assert_ne!(
        el.a, el.b,
        "equal points (AB) in the tri should be resolved to a line"
    );
    debug_assert_ne!(
        el.b, el.c,
        "equal points (BC) in the tri should be resolved to a line"
    );
    debug_assert_ne!(
        el.c, el.a,
        "equal points (CA) in the tri should be resolved to a line"
    );

    let ab = el.b - el.a;
    let bc = el.c - el.b;
    let ca = el.a - el.c;

    let ap = pos - el.a;
    let bp = pos - el.b;
    let cp = pos - el.c;

    let ab_normal = ab.rotate_90_cc();
    let bc_normal = bc.rotate_90_cc();
    let ca_normal = ca.rotate_90_cc();

    // so that no matter the positions of ABC, inside is (true, true, true)
    let normalisation = ca_normal.dot_product(ab).is_sign_negative();

    let ab_dot = ab_normal.dot_product(ap);
    let bc_dot = bc_normal.dot_product(bp);
    let ca_dot = ca_normal.dot_product(cp);

    let side_ab = ab_dot.is_sign_positive() ^ normalisation;
    let side_bc = bc_dot.is_sign_positive() ^ normalisation;
    let side_ca = ca_dot.is_sign_positive() ^ normalisation;

    match (side_ab, side_bc, side_ca) {
        // inside
        (true, true, true) => 0.,
        // opposite AB
        (false, true, true) => {
            let a_closer = ab.dot_product(ap).is_sign_negative();
            let b_closer = ab.dot_product(bp).is_sign_positive();
            match (a_closer, b_closer) {
                (true, false) => ap.length_sq(),
                (false, true) => bp.length_sq(),
                (false, false) => ab_dot * ab_dot / ab_normal.length_sq(),
                (true, true) => unreachable!("point cannot be outside at both places"),
            }
        }
        // opposite B
        (false, false, true) => bp.length_sq(),
        // opposite BC
        (true, false, true) => {
            let b_closer = bc.dot_product(bp).is_sign_negative();
            let c_closer = bc.dot_product(cp).is_sign_positive();
            match (b_closer, c_closer) {
                (true, false) => bp.length_sq(),
                (false, true) => cp.length_sq(),
                (false, false) => bc_dot * bc_dot / bc_normal.length_sq(),
                (true, true) => unreachable!("point cannot be outside at both places"),
            }
        }
        // opposite C
        (true, false, false) => cp.length_sq(),
        // opposite CA
        (true, true, false) => {
            let c_closer = ca.dot_product(cp).is_sign_negative();
            let a_closer = ca.dot_product(ap).is_sign_positive();
            match (c_closer, a_closer) {
                (true, false) => cp.length_sq(),
                (false, true) => ap.length_sq(),
                (false, false) => ca_dot * ca_dot / ca_normal.length_sq(),
                (true, true) => unreachable!("point cannot be outside at both places"),
            }
        }
        // opposite A
        (false, true, false) => ap.length_sq(),
        (false, false, false) => {
            unreachable!("the point cannot be on the outside of every triangle's side")
        }
    }
}

pub fn test(el: &lir::Triangle, pos: Vector) -> bool {
    distance_sq(el, pos) <= el.outline_width * el.outline_width / 4.
}
