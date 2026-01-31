use rotur_icn_compiler::resolver::lir;
use rotur_icn_units::Vector;

// FIXME technically this is a rather wrong approximation
pub fn distance(el: &lir::Ellipse, pos: Vector) -> f32 {
    let ratio = el.sizes.y / el.sizes.x;

    let rel_pos = (pos - el.centre).rotate(-el.direction);

    let skewed = Vector {
        x: rel_pos.x,
        y: rel_pos.y / ratio,
    };

    let d = (skewed.length() - el.sizes.x).abs();

    let factor = skewed.angle().sin().abs();

    d * (1. + factor * (ratio - 1.))
}

pub fn test(el: &lir::Ellipse, pos: Vector) -> bool {
    distance(el, pos) <= el.outline_width / 2.
}
