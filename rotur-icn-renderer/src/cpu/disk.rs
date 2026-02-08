use rotur_icn_compiler::resolver::lir;
use rotur_icn_units::Vector;

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

    pub fn test(&self, pos: Vector) -> bool {
        let rel = pos - self.centre;
        let d = rel.length_sq();

        d <= self.outline
    }
}
