use rotur_icn_resolver::lir;
use rotur_icn_units::Vector;

use super::shape::Shape;

pub struct Arc {
    bb: (Vector, Vector),
    centre: Vector,
    start: Vector,
    middle: Vector,
    end: Vector,
    outline: f32,
    inner_outline: f32,
    outer_outline: f32,
    is_open: bool,
}

impl Arc {
    pub fn new(el: &lir::Arc) -> Self {
        // FIXME make float comp margin relative
        assert!(
            (el.start_angle - el.end_angle).abs() > 1e-7,
            "arcless arcs should be resolved to discs"
        );

        // TODO add internal culling box
        let bb = crate::fitter::arc::get_bounds(el);

        let start = Vector::new_from_length(el.radius, el.start_angle);
        let middle = Vector::new_normal(el.start_angle.midpoint(el.end_angle));
        let end = Vector::new_from_length(el.radius, el.end_angle);

        let is_open = el.end_angle - el.start_angle > std::f32::consts::PI;

        let halfwidth = el.width / 2.;
        let outline = halfwidth.powi(2);
        let inner_outline = (el.radius - halfwidth).powi(2);
        let outer_outline = (el.radius + halfwidth).powi(2);

        Self {
            bb,
            centre: el.centre,
            start,
            middle,
            end,
            is_open,
            outline,
            inner_outline,
            outer_outline,
        }
    }
}

impl Shape for Arc {
    fn test(&self, pos: Vector) -> bool {
        if !pos.within(self.bb) {
            return false;
        }

        let to_pos = pos - self.centre;

        let inside_start = to_pos.cross(self.start).is_sign_negative();
        let inside_end = to_pos.cross(self.end).is_sign_positive();

        let d = match (self.is_open, inside_start, inside_end) {
            (_, true, true) | (true, true, false) | (true, false, true) => {
                let d = to_pos.length_sq();
                return self.inner_outline <= d && d <= self.outer_outline;
            }
            (false, false, true) => (to_pos - self.start).length_sq(),
            (false, true, false) => (to_pos - self.end).length_sq(),
            (_, false, false) => {
                let start_closer = to_pos.cross(self.middle).is_sign_positive();

                (to_pos - if start_closer { self.start } else { self.end }).length_sq()
            }
        };

        d <= self.outline
    }
}
