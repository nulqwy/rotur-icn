use rand::{Rng, distr::Distribution};

use rotur_icn_units::{Colour, Vector};

use crate::IcnSampler;

impl Distribution<Vector> for IcnSampler {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vector {
        Vector {
            x: rng.random_range(self.space_x()),
            y: rng.random_range(self.space_y()),
        }
    }
}

pub struct NormalVector(pub Vector);

impl Distribution<NormalVector> for IcnSampler {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> NormalVector {
        let x: f32 = rng.random_range(-1.0..=1.0);
        let y = (1. - x.powi(2)).sqrt() * if rng.random() { 1.0 } else { -1.0 };
        NormalVector(Vector { x, y })
    }
}

impl Distribution<Colour> for IcnSampler {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Colour {
        if self.full_colour {
            Colour {
                r: rng.random(),
                g: rng.random(),
                b: rng.random(),
                a: 0xff,
            }
        } else {
            let mut gen_col = || rng.random_range(0..16) * 17;

            Colour {
                r: gen_col(),
                g: gen_col(),
                b: gen_col(),
                a: 0xff,
            }
        }
    }
}
