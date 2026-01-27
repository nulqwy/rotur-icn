use rotur_icn_compiler::resolver::lir;
use rotur_icn_units::Vector;

// TODO separate squared distance and not squared
pub fn distance(el: &lir::Rectangle, pos: Vector) -> Distance {
    let bottom_left = el.bottom_left;
    let top_right = bottom_left + el.sizes;

    let bl_below = pos.y < bottom_left.y;
    let bl_left = pos.x < bottom_left.x;

    let tr_above = pos.y > top_right.y;
    let tr_right = pos.x > top_right.x;

    match (bl_below, bl_left, tr_above, tr_right) {
        // bottom left
        (true, true, _, _) => Distance::Squared((bottom_left - pos).length_sq()),
        // below
        (true, false, _, false) => Distance::Direct(bottom_left.y - pos.y),
        // bottom right
        (true, _, _, true) => {
            let bottom_right = Vector {
                x: top_right.x,
                y: bottom_left.y,
            };

            Distance::Squared((bottom_right - pos).length_sq())
        }
        // to the right
        (false, _, false, true) => Distance::Direct(pos.x - top_right.x),
        // top right
        (_, _, true, true) => Distance::Squared((top_right - pos).length_sq()),
        // above
        (_, false, true, false) => Distance::Direct(pos.y - top_right.y),
        // top left
        (_, true, true, _) => {
            let top_left = Vector {
                x: bottom_left.x,
                y: top_right.y,
            };

            Distance::Squared((top_left - pos).length_sq())
        }
        // to the left
        (false, true, false, _) => Distance::Direct(bottom_left.x - pos.x),
        // on the inside
        (false, false, false, false) => {
            if el.filled {
                Distance::Direct(0.)
            } else {
                let middle = bottom_left + el.sizes / 2.;

                // TODO technically by some (even) more comparisons, the min() could be resolved
                // idk if it would have any meaningful difference tho, prob not

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

                Distance::Direct(horizontal.min(vertical))
            }
        }
    }
}

pub enum Distance {
    Direct(f32),
    Squared(f32),
}

pub fn test(el: &lir::Rectangle, pos: Vector) -> bool {
    match distance(el, pos) {
        Distance::Direct(d) => d <= el.outline_width / 2.,
        Distance::Squared(d) => d <= el.outline_width * el.outline_width / 4.,
    }
}
