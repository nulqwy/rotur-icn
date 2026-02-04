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
