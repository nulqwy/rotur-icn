use std::f32;

use rotur_icn_resolver::lir;
use rotur_icn_units::Vector;

use super::shape::Shape;

pub struct Line {
    bb: (Vector, Vector),
    start: Vector,
    end: Vector,
    outline: f32,
}

impl Line {
    pub fn new(el: &lir::Line) -> Self {
        assert_ne!(
            el.start, el.end,
            "equal points in the line should be resolved to a dot"
        );

        let bb = crate::fitter::line::get_bounds(el);

        Self {
            bb,
            start: el.start,
            end: el.end,
            outline: el.width.powi(2) / 4.,
        }
    }
}

impl Shape for Line {
    fn test(&self, pos: Vector) -> bool {
        if !pos.within(self.bb) {
            return false;
        }

        let ab = self.end - self.start;

        let ap = pos - self.start;
        let ab_dot = ab.dot(ap);

        let a_closer = ab_dot.is_sign_negative();
        let b_closer = ab_dot > ab.length_sq();

        let d = match (a_closer, b_closer) {
            (true, false) => ap.length_sq(),
            (false, true) => (pos - self.end).length_sq(),
            (false, false) => ab.cross(ap).powi(2) / ab.length_sq(),
            (true, true) => unreachable!("point cannot be outside at both places"),
        };

        d <= self.outline
    }
}
