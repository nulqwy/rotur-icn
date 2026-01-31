use rotur_icn_compiler::resolver::lir;
use rotur_icn_units::Vector;

pub fn distance_sq(el: &lir::Arc, pos: Vector) -> f32 {
    assert_ne!(
        el.start_angle, el.end_angle,
        "arcless arcs should be resolved to discs"
    );

    let start = Vector::new_from_length(el.radius, el.start_angle);
    let end = Vector::new_from_length(el.radius, el.end_angle);

    let to_pos = pos - el.centre;

    // would have used a cross product, but that one gives weird FP shenanigans
    let inside_start = to_pos.dot(start.rotate_90_cc()).is_sign_positive();
    let inside_end = to_pos.dot(end.rotate_90_cw()).is_sign_positive();

    let is_open = el.end_angle - el.start_angle > std::f32::consts::PI;

    match (is_open, inside_start, inside_end) {
        // TODO same treatment as for the circle
        (_, true, true) | (true, true, false) | (true, false, true) => {
            (to_pos.length() - el.radius).abs().powi(2)
        }
        (false, false, true) => (to_pos - start).length_sq(),
        (false, true, false) => (to_pos - end).length_sq(),
        (_, false, false) => {
            let middle = Vector::new_normal(el.start_angle.midpoint(el.end_angle));

            let start_closer = to_pos.cross(middle).is_sign_positive();

            (to_pos
                - match start_closer {
                    true => start,
                    false => end,
                })
            .length_sq()
        }
    }
}

pub fn test(el: &lir::Arc, pos: Vector) -> bool {
    distance_sq(el, pos) <= el.width * el.width / 4.
}
