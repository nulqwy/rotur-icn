use rotur_icn_resolver::lir;
use rotur_icn_units::Vector;

use super::shape::Shape;

pub struct Circle {
    centre: Vector,
    inner_outline: f32,
    outer_outline: f32,
}

impl Circle {
    pub fn new(el: &lir::Circle) -> Self {
        let halfwidth = el.width / 2.;
        let inner_outline = (el.radius - halfwidth).powi(2);
        let outer_outline = (el.radius + halfwidth).powi(2);

        Self {
            centre: el.centre,
            inner_outline,
            outer_outline,
        }
    }
}

impl Shape for Circle {
    fn test(&self, pos: Vector) -> bool {
        let rel_pos = pos - self.centre;
        let d = rel_pos.length_sq();

        self.inner_outline <= d && d <= self.outer_outline
    }

    fn possibly_within(&self, bounds: (Vector, Vector)) -> bool {
        // TODO [see Disk::possibly_within()]
        let outer_radius = self.outer_outline.sqrt() * std::f32::consts::SQRT_2;
        let outer_bb = (self.centre - outer_radius, self.centre + outer_radius);
        let inner_radius = self.inner_outline.sqrt() * std::f32::consts::FRAC_1_SQRT_2;
        let inner_bb = (self.centre - inner_radius, self.centre + inner_radius);

        Vector::bounds_intersect(outer_bb, bounds) && !Vector::bounds_contained(inner_bb, bounds)
    }
}
