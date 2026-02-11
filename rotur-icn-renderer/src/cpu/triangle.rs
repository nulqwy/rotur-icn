use rotur_icn_resolver::lir;
use rotur_icn_units::Vector;

use super::shape::Shape;

pub struct Triangle {
    bb: (Vector, Vector),
    a: Vector,
    b: Vector,
    c: Vector,
    normalisation: bool,
    outline: f32,
}

impl Triangle {
    pub fn new(el: &lir::Triangle) -> Self {
        assert!(
            el.a != el.b && el.b != el.c && el.c != el.a,
            "equal points (ABC) in the tri should be resolved to a disk"
        );
        assert_ne!(
            el.a, el.b,
            "equal points (AB) in the tri should be resolved to a line"
        );
        assert_ne!(
            el.b, el.c,
            "equal points (BC) in the tri should be resolved to a line"
        );
        assert_ne!(
            el.c, el.a,
            "equal points (CA) in the tri should be resolved to a line"
        );

        Self {
            // TODO add internal culling box
            bb: crate::fitter::triangle::get_bounds(el),
            a: el.a,
            b: el.b,
            c: el.c,
            normalisation: (el.a - el.c).cross(el.b - el.a).is_sign_negative(),
            outline: el.outline_width.powi(2) / 4.,
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
}

impl Shape for Triangle {
    fn test(&self, pos: Vector) -> bool {
        if !pos.within(self.bb) {
            return false;
        }

        let ab = self.b - self.a;
        let bc = self.c - self.b;
        let ca = self.a - self.c;

        let ap = pos - self.a;
        let bp = pos - self.b;
        let cp = pos - self.c;

        let ab_cro = ab.cross(ap);
        let bc_cro = bc.cross(bp);
        let ca_cro = ca.cross(cp);

        // so that no matter the positions of ABC, inside is (true, true, true)
        let side_ab = ab_cro.is_sign_positive() ^ self.normalisation;
        let side_bc = bc_cro.is_sign_positive() ^ self.normalisation;
        let side_ca = ca_cro.is_sign_positive() ^ self.normalisation;

        let d = match (side_ab, side_bc, side_ca) {
            // inside
            (true, true, true) => return true,
            // opposite AB
            (false, true, true) => Self::line_dist(ab, ab_cro, ap, bp),
            // opposite B
            (false, false, true) => bp.length_sq(),
            // opposite BC
            (true, false, true) => Self::line_dist(bc, bc_cro, bp, cp),
            // opposite C
            (true, false, false) => cp.length_sq(),
            // opposite CA
            (true, true, false) => Self::line_dist(ca, ca_cro, cp, ap),
            // opposite A
            (false, true, false) => ap.length_sq(),
            (false, false, false) => {
                unreachable!("the point cannot be on the outside of every triangle's side")
            }
        };

        d <= self.outline
    }
}
