use rand::{Rng, distr::Distribution};

use rotur_icn_compiler::lowerer::hir;
use rotur_icn_units::Vector;

use crate::units::NormalVector;

use super::IcnSampler;

impl Distribution<hir::SetWidth> for IcnSampler {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> hir::SetWidth {
        let value_log = rng.random_range(self.width_log_range.clone());
        hir::SetWidth {
            value: value_log.exp(),
        }
    }
}

impl Distribution<hir::SetColour> for IcnSampler {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> hir::SetColour {
        hir::SetColour {
            value: self.sample(rng),
        }
    }
}

impl Distribution<hir::DrawLine> for IcnSampler {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> hir::DrawLine {
        hir::DrawLine {
            start: self.sample(rng),
            end: self.sample(rng),
        }
    }
}

impl Distribution<hir::ContinueLine> for IcnSampler {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> hir::ContinueLine {
        hir::ContinueLine {
            next: self.sample(rng),
        }
    }
}

impl Distribution<hir::DrawDisk> for IcnSampler {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> hir::DrawDisk {
        hir::DrawDisk {
            centre: self.sample(rng),
        }
    }
}

impl Distribution<hir::DrawRectangle> for IcnSampler {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> hir::DrawRectangle {
        let filled: bool = rng.random();

        if filled {
            let centre: Vector = self.sample(rng);

            let a: NormalVector = self.sample(rng);
            let b: NormalVector = self.sample(rng);

            let radius: f32 = rng.random_range(self.filled_radius_range.clone());
            let sizes = (a.0 - b.0).abs() * radius;

            hir::DrawRectangle {
                centre,
                sizes: sizes / 2.,
                filled,
            }
        } else {
            let a: Vector = self.sample(rng);
            let b: Vector = self.sample(rng);

            let (bl, tr) = a.min_max(b);

            let centre = bl.midpoint(tr);
            let sizes = tr - bl;

            hir::DrawRectangle {
                centre,
                sizes: sizes / 2.,
                filled,
            }
        }
    }
}

impl Distribution<hir::DrawTriangle> for IcnSampler {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> hir::DrawTriangle {
        let centre: Vector = self.sample(rng);

        let radius: f32 = rng.random_range(self.filled_radius_range.clone());

        let a: NormalVector = self.sample(rng);
        let b: NormalVector = self.sample(rng);
        let c: NormalVector = self.sample(rng);

        hir::DrawTriangle {
            a: centre + a.0 * radius,
            b: centre + b.0 * radius,
            c: centre + c.0 * radius,
        }
    }
}

impl Distribution<hir::MoveCentre> for IcnSampler {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> hir::MoveCentre {
        // FIXME properly handle the canvas bounds, or at least do smth abt them
        hir::MoveCentre {
            change: self.sample(rng),
        }
    }
}

impl Distribution<hir::ResetCentre> for IcnSampler {
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> hir::ResetCentre {
        // XXX should it dummy-generate some value to just advance the RNG?
        hir::ResetCentre
    }
}

impl Distribution<hir::DrawArc> for IcnSampler {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> hir::DrawArc {
        // XXX should this be precomputed and stored?
        let space_size = self.space.1 - self.space.0;
        let max_radius = space_size.min_axis() / 2.;

        hir::DrawArc {
            centre: self.sample(rng),
            radius: rng.random_range(0.0..=max_radius),
            direction: rng.random_range(0.0..36.0),
            arm_angle: rng.random_range(0.0..=180.0),
        }
    }
}

impl Distribution<hir::DrawEllipse> for IcnSampler {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> hir::DrawEllipse {
        // XXX [same as for DrawArc]
        let space_size = self.space.1 - self.space.0;
        let max_radius = space_size.min_axis() / 2.;

        let major = rng.random_range(0.0..=max_radius);
        let minor = rng.random_range(0.0..=max_radius);

        hir::DrawEllipse {
            centre: self.sample(rng),
            major,
            ratio: minor / major,
            direction: rng.random_range(0.0..360.0),
        }
    }
}

impl Distribution<hir::DrawCurve> for IcnSampler {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> hir::DrawCurve {
        hir::DrawCurve {
            start: self.sample(rng),
            control: self.sample(rng),
            end: self.sample(rng),
        }
    }
}
