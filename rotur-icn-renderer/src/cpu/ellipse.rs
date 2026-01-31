use rotur_icn_compiler::resolver::lir;
use rotur_icn_units::Vector;

// from https://www.shadertoy.com/view/tt3yz7
pub fn distance_sq(el: &lir::Ellipse, pos: Vector) -> f32 {
    let e = el.sizes;
    let p_abs = (pos - el.centre).rotate(-el.direction).abs();
    let ei = 1. / e;
    let e2 = e.powi(2);
    let ve = ei
        * Vector {
            x: e2.x - e2.y,
            y: e2.y - e2.x,
        };

    let mut t = Vector::new_normal(std::f32::consts::FRAC_PI_4);
    for _ in 0..3 {
        let v = ve * t.powi(3);
        let u = (p_abs - v).normalise() * (t * e - v).length();
        let w = ei * (v + u);
        t = w.clamp(0., 1.).normalise();
    }

    let nearest_abs = t * e;
    (p_abs - nearest_abs).length_sq()
}

pub fn test(el: &lir::Ellipse, pos: Vector) -> bool {
    distance_sq(el, pos) <= el.outline_width * el.outline_width / 4.
}
