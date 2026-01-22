use rotur_icn_compiler::resolver::lir;
use rotur_icn_units::Vector;

// TODO separate squared distance and not squared
pub fn distance_sq(el: &lir::Rectangle, pos: Vector) -> f32 {
    let bottom_left = el.bottom_left;
    let top_right = bottom_left + el.sizes;

    let bl_below = pos.y < bottom_left.y;
    let bl_left = pos.x < bottom_left.x;

    let tr_above = pos.y > top_right.y;
    let tr_right = pos.x > top_right.x;

    match (bl_below, bl_left, tr_above, tr_right) {
        // bottom left
        (true, true, _, _) => (bottom_left - pos).length_sq(),
        // below
        (true, false, _, false) => {
            let d = bottom_left.y - pos.y;
            d * d
        }
        // bottom right
        (true, _, _, true) => {
            let bottom_right = Vector {
                x: top_right.x,
                y: bottom_left.y,
            };
            (bottom_right - pos).length_sq()
        }
        // to the right
        (false, _, false, true) => {
            let d = pos.x - top_right.x;
            d * d
        }
        // top right
        (_, _, true, true) => (top_right - pos).length_sq(),
        // above
        (_, false, true, false) => {
            let d = pos.y - top_right.y;
            d * d
        }
        // top left
        (_, true, true, _) => {
            let top_left = Vector {
                x: bottom_left.x,
                y: top_right.y,
            };
            (top_left - pos).length_sq()
        }
        // to the left
        (false, true, false, _) => {
            let d = bottom_left.x - pos.x;
            d * d
        }
        // on the inside
        (false, false, false, false) => {
            if el.filled {
                0.
            } else {
                let middle = bottom_left + el.sizes / 2.;
                let horizontal = if pos.x < middle.x {
                    pos.x - bottom_left.x
                } else {
                    top_right.x - pos.x
                };
                let vertical = if pos.y < middle.y {
                    pos.y - bottom_left.y
                } else {
                    top_right.y - pos.y
                };
                let d = horizontal.min(vertical);
                d * d
            }
        }
    }
}

pub fn test(el: &lir::Rectangle, pos: Vector) -> bool {
    distance_sq(el, pos) < el.outline_width * el.outline_width
}
