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
}
