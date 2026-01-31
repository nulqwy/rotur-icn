use rotur_icn_compiler::resolver::lir;
use rotur_icn_units::Vector;

pub fn distance_sq(el: &lir::Triangle, pos: Vector) -> f32 {
    debug_assert!(
        el.a != el.b && el.b != el.c && el.c != el.a,
        "equal points (ABC) in the tri should be resolved to a disk"
    );
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

    // so that no matter the positions of ABC, inside is (true, true, true)
    let normalisation = ca.cross(ab).is_sign_negative();

    let ab_cro = ab.cross(ap);
    let bc_cro = bc.cross(bp);
    let ca_cro = ca.cross(cp);

    let side_ab = ab_cro.is_sign_positive() ^ normalisation;
    let side_bc = bc_cro.is_sign_positive() ^ normalisation;
    let side_ca = ca_cro.is_sign_positive() ^ normalisation;

    match (side_ab, side_bc, side_ca) {
        // inside
        (true, true, true) => 0.,
        // opposite AB
        (false, true, true) => {
            let abp_dot = ab.dot(ap);
            let a_closer = abp_dot.is_sign_negative();
            let b_closer = abp_dot > ab.length_sq();
            match (a_closer, b_closer) {
                (true, false) => ap.length_sq(),
                (false, true) => bp.length_sq(),
                (false, false) => ab_cro.powi(2) / ab.length_sq(),
                (true, true) => unreachable!("point cannot be outside at both places"),
            }
        }
        // opposite B
        (false, false, true) => bp.length_sq(),
        // opposite BC
        (true, false, true) => {
            let bcp_dot = bc.dot(bp);
            let b_closer = bcp_dot.is_sign_negative();
            let c_closer = bcp_dot > bc.length_sq();
            match (b_closer, c_closer) {
                (true, false) => bp.length_sq(),
                (false, true) => cp.length_sq(),
                (false, false) => bc_cro.powi(2) / bc.length_sq(),
                (true, true) => unreachable!("point cannot be outside at both places"),
            }
        }
        // opposite C
        (true, false, false) => cp.length_sq(),
        // opposite CA
        (true, true, false) => {
            let cap_dot = ca.dot(cp);
            let c_closer = cap_dot.is_sign_negative();
            let a_closer = cap_dot > ca.length_sq();
            match (c_closer, a_closer) {
                (true, false) => cp.length_sq(),
                (false, true) => ap.length_sq(),
                (false, false) => ca_cro.powi(2) / ca.length_sq(),
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
