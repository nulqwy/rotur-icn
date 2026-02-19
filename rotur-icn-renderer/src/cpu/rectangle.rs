use rotur_icn_resolver::lir;
use rotur_icn_units::Vector;

use super::shape::Shape;

pub struct Rectangle {
    centre: Vector,
    halfsizes: Vector,
    outline: f32,
    filled: bool,
}

impl Rectangle {
    pub fn new(el: &lir::Rectangle) -> Self {
        let halfsizes = el.sizes / 2.;
        let centre = el.bottom_left + halfsizes;

        Self {
            centre,
            halfsizes,
            outline: el.outline_width / 2.,
            filled: el.filled,
        }
    }
}

impl Shape for Rectangle {
    fn test(&self, pos: Vector) -> bool {
        let rel_pos_abs = (pos - self.centre).abs();

        let d_vec = rel_pos_abs - self.halfsizes;

        let above = d_vec.y.is_sign_positive();
        let on_right = d_vec.x.is_sign_positive();

        match (above, on_right) {
            (false, false) if self.filled => true,
            (false, false) => d_vec.x.max(d_vec.y) >= -self.outline,
            (true, false) => d_vec.y <= self.outline,
            (false, true) => d_vec.x <= self.outline,
            (true, true) => d_vec.length_sq() <= self.outline.powi(2),
        }
    }

    fn possibly_within(&self, bounds: (Vector, Vector)) -> bool {
        let edges = (self.centre - self.halfsizes, self.centre + self.halfsizes);

        let outer_bb = (edges.0 - self.outline, edges.1 + self.outline);

        if Vector::bounds_intersect(outer_bb, bounds) {
            if self.filled {
                true
            } else {
                let inner_bb = (edges.0 + self.outline, edges.1 - self.outline);

                !Vector::bounds_contained(inner_bb, bounds)
            }
        } else {
            false
        }
    }
}
