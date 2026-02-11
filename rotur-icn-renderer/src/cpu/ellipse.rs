use rotur_icn_resolver::lir;
use rotur_icn_units::Vector;

use super::shape::Shape;

pub struct Ellipse {
    bb: (Vector, Vector),
    centre: Vector,
    axis: Vector,
    rotation_coefs: (f32, f32),
    axis_inverse: Vector,
    axis_v: Vector,
    outline: f32,
}

impl Ellipse {
    pub fn new(el: &lir::Ellipse) -> Self {
        assert!(
            el.axis.x != 0. && el.axis.y != 0.,
            "zero-length-axis ellipse should be resolved to a dot or a line"
        );
        // FIXME use relative margin
        assert!(
            (el.axis.x - el.axis.y).abs() > 1e-7,
            "same-axis ellipse should be resolved to a circle"
        );

        let axis_inverse = 1. / el.axis;
        let axis2 = el.axis.powi(2);
        let axis_v = axis_inverse * Vector::new(axis2.x - axis2.y).conj();

        Self {
            // TODO add internal culling box
            bb: crate::fitter::ellipse::get_bounds(el),
            centre: el.centre,
            axis: el.axis,
            rotation_coefs: (-el.direction).sin_cos(),
            axis_inverse,
            axis_v,
            outline: el.outline_width.powi(2) / 4.,
        }
    }
}

impl Shape for Ellipse {
    // from https://www.shadertoy.com/view/tt3yz7
    // TODO find another algo which supports a rotated ellipse within itself
    fn test(&self, pos: Vector) -> bool {
        if !pos.within(self.bb) {
            return false;
        }

        let p_abs = (pos - self.centre)
            .rotate_with_coefs(self.rotation_coefs)
            .abs();

        let mut t = Vector::new_normal(std::f32::consts::FRAC_PI_4);
        for _ in 0..3 {
            let v = self.axis_v * t.powi(3);
            let u = (p_abs - v).normalise() * (t * self.axis - v).length();
            let w = self.axis_inverse * (v + u);
            t = w.clamp(0., 1.).normalise();
        }

        let nearest_abs = t * self.axis;
        let d = (p_abs - nearest_abs).length_sq();

        d <= self.outline
    }
}
