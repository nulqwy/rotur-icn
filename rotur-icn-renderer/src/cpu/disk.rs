use rotur_icn_compiler::resolver::lir;
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
            outline: el.radius,
        }
    }
}

impl Shape for Disk {
    fn test(&self, pos: Vector) -> bool {
        let rel = pos - self.centre;
        let d = rel.length_sq();

        d <= self.outline
    }
}
