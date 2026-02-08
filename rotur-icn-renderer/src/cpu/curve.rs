use rotur_icn_compiler::resolver::lir;
use rotur_icn_units::Vector;

use super::maths::cos_acos_3;

pub struct Curve {
    bb: (Vector, Vector),
    start: Vector,
    a: Vector,
    b: Vector,
    kk: f32,
    outline: f32,
}

impl Curve {
    pub fn new(el: &lir::Curve) -> Self {
        let bb = crate::fitter::curve::get_bounds(el);
        let a = el.control - el.start;
        let b = el.start - 2. * el.control + el.end;
        let kk = 1. / b.length_sq();
        let outline = el.width.powi(2) / 4.;

        Self {
            bb,
            start: el.start,
            a,
            b,
            kk,
            outline,
        }
    }

    // based on https://www.shadertoy.com/view/MlKcDD
    // TODO try using the faster apprx method + aggressive triangle culling
    pub fn test(&self, pos: Vector) -> bool {
        if !pos.within(self.bb) {
            return false;
        }

        let c = 2. * self.a;
        let d = self.start - pos;

        let kx = self.kk * self.a.dot(self.b);
        let ky = self.kk * (2. * self.a.length_sq() + d.dot(self.b)) / 3.;
        let kz = self.kk * d.dot(self.a);

        let kx2 = kx.powi(2);
        let p = ky - kx2;
        let q = kx * (2. * kx2 - 3. * ky) + kz;
        let p3 = p.powi(3);
        let q2 = q.powi(2);
        let h = q2 + 4. * p3;

        let d = if h >= 0. {
            let nq = -q;

            let h = h.sqrt().copysign(nq);

            let x = h.midpoint(nq);
            let v = x.abs().cbrt().copysign(x);
            let mut t = v - p / v;
            let t2 = t.powi(2);

            t -= (t * (t2 + 3. * p) + q) / (3. * t2 + 3. * p);

            let t = (t - kx).clamp(0., 1.);
            let w = d + (c + self.b * t) * t;
            w.length_sq()
        } else {
            let z = (-p).sqrt();

            let m = cos_acos_3(q / (p * z * 2.));
            let n = (3. - 3. * m.powi(2)).sqrt();

            let t = (Vector {
                x: 2. * m,
                y: -n - m,
            } * z
                - kx)
                .clamp(0., 1.);

            let q1 = d + (c + self.b * t.x) * t.x;
            let d1 = q1.length_sq();

            let q2 = d + (c + self.b * t.y) * t.y;
            let d2 = q2.length_sq();

            d1.min(d2)
        };

        d <= self.outline
    }
}
