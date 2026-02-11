use rotur_icn_resolver::lir;
use rotur_icn_units::{Colour, Vector};

use rotur_icn_printer::print_lir;

fn main() {
    let icon = lir::IconLir {
        elements: vec![
            lir::Element {
                colour: Colour::default(),
                kind: lir::ElementKind::Line(lir::Line {
                    start: Vector { x: -2., y: 5. },
                    end: Vector { x: -1., y: -4. },
                    width: 1.,
                }),
            },
            lir::Element {
                colour: Colour::WHITE,
                kind: lir::ElementKind::Disk(lir::Disk {
                    centre: Vector { x: 4., y: 1. },
                    radius: 0.5,
                }),
            },
        ],
    };

    let stringified = print_lir(&icon, false);

    println!("{stringified}");
}
