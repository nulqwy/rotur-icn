use std::f32;

use rotur_icn_compiler::resolver::lir;
use rotur_icn_units::Vector;

pub fn distance_sq(el: &lir::Line, pos: Vector) -> f32 {
    let end_to_pos = pos - el.end;
    let end_to_start = el.start - el.end;

    let core_product = end_to_start.dot_product(end_to_pos);

    if core_product < 0. {
        return end_to_pos.length_sq();
    }

    let end_to_start_len_sq = end_to_start.length_sq();

    if core_product > end_to_start_len_sq {
        let end_to_start = pos - el.start;
        return end_to_start.length_sq();
    }

    -(core_product * core_product) / end_to_start_len_sq + end_to_pos.length_sq()
}

pub fn test(el: &lir::Line, pos: Vector) -> bool {
    distance_sq(el, pos) <= el.width * el.width / 4.
}
