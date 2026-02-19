use rotur_icn_resolver::lir;
use rotur_icn_units::Vector;

use super::shape::Shape;

#[derive(Debug, Clone)]
pub struct Disk {
    centre: Vector,
    outline: f32,
}

impl Disk {
    pub fn new(el: &lir::Disk) -> Self {
        Self {
            centre: el.centre,
            outline: el.radius.powi(2),
        }
    }
}

impl Shape for Disk {
    fn test(&self, pos: Vector) -> bool {
        let rel = pos - self.centre;
        let d = rel.length_sq();

        d <= self.outline
    }

    fn possibly_within(&self, bounds: (Vector, Vector)) -> bool {
        // TODO try removing sqrt()
        let radius = self.outline.sqrt() * std::f32::consts::SQRT_2;
        let bb = (self.centre - radius, self.centre + radius);

        Vector::bounds_intersect(bb, bounds)
    }
}
