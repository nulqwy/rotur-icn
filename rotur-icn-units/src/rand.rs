use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};

use crate::{Colour, Vector};

impl Distribution<Vector> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vector {
        Vector::new_normal(rng.random_range(0.0..std::f32::consts::TAU))
    }
}

impl Distribution<Colour> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Colour {
        Colour {
            r: rng.random(),
            g: rng.random(),
            b: rng.random(),
            a: 0xff,
        }
    }
}
