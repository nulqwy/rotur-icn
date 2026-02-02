use arrayvec::ArrayVec;
use rotur_icn_compiler::{lowerer::hir, resolver::lir};
use rotur_icn_syntax::lexer::token;
use rotur_icn_units::Colour;

pub fn transform(lir: &lir::IconLir) -> hir::IconHir {
    hir::IconHir {
        operations: lir
            .elements
            .iter()
            .scan(Transformer::default(), |trans, el| {
                Some(match &el.kind {
                    // TODO detect `cont`s
                    lir::ElementKind::Line(line) => trans.process(
                        el,
                        line.width,
                        hir::OperationKind::DrawLine(hir::DrawLine {
                            start: line.start,
                            end: line.end,
                        }),
                    ),
                    lir::ElementKind::Disk(disk) => trans.process(
                        el,
                        disk.radius * 2.,
                        hir::OperationKind::DrawDisk(hir::DrawDisk {
                            centre: disk.centre,
                        }),
                    ),
                    lir::ElementKind::Circle(circle) => trans.process(
                        el,
                        circle.width,
                        hir::OperationKind::DrawArc(hir::DrawArc {
                            centre: circle.centre,
                            radius: circle.radius,
                            direction: 0.,
                            arm_angle: 180.,
                        }),
                    ),
                    lir::ElementKind::Rectangle(rectangle) => trans.process(
                        el,
                        rectangle.outline_width,
                        hir::OperationKind::DrawRectangle(hir::DrawRectangle {
                            centre: rectangle.bottom_left + rectangle.sizes / 2.,
                            sizes: rectangle.sizes,
                            filled: rectangle.filled,
                        }),
                    ),
                    lir::ElementKind::Triangle(triangle) => trans.process(
                        el,
                        triangle.outline_width,
                        hir::OperationKind::DrawTriangle(hir::DrawTriangle {
                            a: triangle.a,
                            b: triangle.b,
                            c: triangle.c,
                        }),
                    ),
                    lir::ElementKind::Arc(arc) => trans.process(
                        el,
                        arc.width,
                        hir::OperationKind::DrawArc({
                            let direction = std::f32::consts::FRAC_PI_2
                                - arc.start_angle.midpoint(arc.end_angle);
                            let arm_angle = (arc.end_angle - arc.start_angle) / 2.;

                            hir::DrawArc {
                                centre: arc.centre,
                                radius: arc.radius,
                                direction: direction.to_degrees() / 10.,
                                arm_angle: arm_angle.to_degrees(),
                            }
                        }),
                    ),
                    lir::ElementKind::Ellipse(ellipse) => trans.process(
                        el,
                        ellipse.outline_width,
                        hir::OperationKind::DrawEllipse(hir::DrawEllipse {
                            centre: ellipse.centre,
                            major: ellipse.axis.x,
                            ratio: ellipse.axis.y / ellipse.axis.x,
                            direction: -ellipse.direction.to_degrees(),
                        }),
                    ),
                    lir::ElementKind::Curve(curve) => trans.process(
                        el,
                        curve.width,
                        hir::OperationKind::DrawCurve(hir::DrawCurve {
                            start: curve.start,
                            control: curve.control,
                            end: curve.end,
                        }),
                    ),
                })
            })
            .flatten()
            .collect(),
    }
}

#[derive(Debug, Clone)]
struct Transformer {
    colour: Colour,
    width: f32,
}

impl Default for Transformer {
    fn default() -> Self {
        Self {
            colour: Colour::WHITE,
            width: 10.,
        }
    }
}

impl Transformer {
    fn process(
        &mut self,
        el: &lir::Element,
        width: f32,
        op: hir::OperationKind,
    ) -> ArrayVec<hir::Operation, 3> {
        // XXX address this somehow?
        let cmd_pos = (token::Loc::default(), token::Loc::default());

        let mut buf = ArrayVec::<_, 3>::new();

        if self.width != width {
            self.width = width;

            buf.push(hir::Operation {
                cmd_pos,
                kind: hir::OperationKind::SetWidth(hir::SetWidth { value: width }),
            });
        }

        if self.colour != el.colour {
            self.colour = el.colour;

            buf.push(hir::Operation {
                cmd_pos,
                kind: hir::OperationKind::SetColour(hir::SetColour { value: el.colour }),
            });
        }

        buf.push(hir::Operation { cmd_pos, kind: op });

        buf
    }
}
