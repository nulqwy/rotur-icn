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
        (false, true, true) => line_dist(ab, ab_cro, ap, bp),
        // opposite B
        (false, false, true) => bp.length_sq(),
        // opposite BC
        (true, false, true) => line_dist(bc, bc_cro, bp, cp),
        // opposite C
        (true, false, false) => cp.length_sq(),
        // opposite CA
        (true, true, false) => line_dist(ca, ca_cro, cp, ap),
        // opposite A
        (false, true, false) => ap.length_sq(),
        (false, false, false) => {
            unreachable!("the point cannot be on the outside of every triangle's side")
        }
    }
}

#[inline]
fn line_dist(side: Vector, side_cro: f32, start_pos: Vector, end_pos: Vector) -> f32 {
    let side_dot = side.dot(start_pos);

    let start_closer = side_dot.is_sign_negative();
    let end_closer = side_dot > side.length_sq();

    match (start_closer, end_closer) {
        (true, false) => start_pos.length_sq(),
        (false, true) => end_pos.length_sq(),
        (false, false) => side_cro.powi(2) / side.length_sq(),
        (true, true) => unreachable!("point cannot be outside at both places"),
    }
}

pub fn test(el: &lir::Triangle, pos: Vector) -> bool {
    distance_sq(el, pos) <= el.outline_width * el.outline_width / 4.
}
