use rotur_icn_units::{Colour, Vector};

use crate::lowerer::hir;

mod display;
mod error;
pub mod lir;

pub use error::{Error, ErrorKind};

pub fn resolve(hir: &hir::IconHir) -> (lir::IconLir, Vec<Error>) {
    let mut errors = Vec::new();

    let mut origin = Vector::ZERO;
    let mut colour = Colour::WHITE;
    let mut width = 10.;
    let mut last_point = None;

    let mut elements = Vec::with_capacity(hir.operations.len());
    let mut dangling_contlines_chained = false;
    for op in &hir.operations {
        if !matches!(op.kind, hir::OperationKind::ContinueLine(..)) {
            dangling_contlines_chained = false;
        }

        let el = lir::Element {
            colour,
            kind: match &op.kind {
                hir::OperationKind::SetWidth(set_width) => {
                    width = set_width.value;
                    continue;
                }
                hir::OperationKind::SetColour(set_colour) => {
                    colour = set_colour.value;
                    continue;
                }
                hir::OperationKind::DrawLine(draw_line) => {
                    let end = origin + draw_line.end;

                    last_point = Some(end);

                    if draw_line.start == draw_line.end {
                        lir::ElementKind::Disk(lir::Disk {
                            centre: end,
                            radius: width / 2.,
                        })
                    } else {
                        lir::ElementKind::Line(lir::Line {
                            start: origin + draw_line.start,
                            end,
                            width,
                        })
                    }
                }
                hir::OperationKind::ContinueLine(continue_line) => {
                    if let Some(start) = last_point {
                        debug_assert!(
                            !dangling_contlines_chained,
                            "this shouldn't happen as dangling continued lines don't define a last point",
                        );

                        let end = origin + continue_line.next;

                        last_point = Some(end);

                        lir::ElementKind::Line(lir::Line {
                            start: origin + start,
                            end,
                            width,
                        })
                    } else {
                        if !dangling_contlines_chained {
                            errors.push(Error {
                                cmd_pos: op.cmd_pos,
                                cmd_index: op.cmd_index,
                                kind: ErrorKind::DanglingContinuedLine,
                            });

                            dangling_contlines_chained = true;
                        }

                        continue;
                    }
                }
                hir::OperationKind::DrawDisk(draw_disk) => {
                    let centre = origin + draw_disk.centre;

                    last_point = Some(centre);

                    lir::ElementKind::Disk(lir::Disk {
                        centre,
                        radius: width / 2.,
                    })
                }
                hir::OperationKind::DrawRectangle(draw_rectangle) => {
                    let bottom_left = origin + draw_rectangle.centre - draw_rectangle.sizes;

                    if !draw_rectangle.filled {
                        let top_right = draw_rectangle.centre + draw_rectangle.sizes;
                        last_point = Some(origin + top_right);
                    } else {
                        last_point = None;
                    }

                    lir::ElementKind::Rectangle(lir::Rectangle {
                        bottom_left,
                        sizes: draw_rectangle.sizes * 2.,
                        filled: draw_rectangle.filled,
                        outline_width: width,
                    })
                }
                hir::OperationKind::DrawTriangle(draw_triangle) => {
                    last_point = None;

                    if draw_triangle.a == draw_triangle.b && draw_triangle.b == draw_triangle.c {
                        lir::ElementKind::Disk(lir::Disk {
                            centre: origin + draw_triangle.a,
                            radius: width / 2.,
                        })
                    } else if draw_triangle.a == draw_triangle.b {
                        lir::ElementKind::Line(lir::Line {
                            start: origin + draw_triangle.a,
                            end: origin + draw_triangle.c,
                            width,
                        })
                    } else if draw_triangle.b == draw_triangle.c {
                        lir::ElementKind::Line(lir::Line {
                            start: origin + draw_triangle.b,
                            end: origin + draw_triangle.a,
                            width,
                        })
                    } else if draw_triangle.c == draw_triangle.a {
                        lir::ElementKind::Line(lir::Line {
                            start: origin + draw_triangle.c,
                            end: origin + draw_triangle.b,
                            width,
                        })
                    } else {
                        lir::ElementKind::Triangle(lir::Triangle {
                            a: origin + draw_triangle.a,
                            b: origin + draw_triangle.b,
                            c: origin + draw_triangle.c,
                            outline_width: width,
                        })
                    }
                }
                hir::OperationKind::MoveCentre(move_centre) => {
                    origin += move_centre.change;
                    continue;
                }
                hir::OperationKind::ResetCentre(hir::ResetCentre) => {
                    origin = Vector::ZERO;
                    continue;
                }
                hir::OperationKind::DrawArc(draw_arc) => {
                    let direction = draw_arc.direction * std::f32::consts::PI / 18.;
                    let arm_angle = draw_arc.arm_angle * std::f32::consts::PI / 180.;

                    let end_angle = std::f32::consts::FRAC_PI_2 - (direction - arm_angle);
                    let start_angle = std::f32::consts::FRAC_PI_2 - (direction + arm_angle);

                    let centre = origin + draw_arc.centre;

                    let start_point =
                        centre + Vector::new_from_length(draw_arc.radius, start_angle);
                    last_point = Some(start_point);

                    if draw_arc.arm_angle == 180. {
                        lir::ElementKind::Circle(lir::Circle {
                            centre,
                            radius: draw_arc.radius,
                            width,
                        })
                    } else if draw_arc.arm_angle == 0. {
                        lir::ElementKind::Disk(lir::Disk {
                            centre: start_point,
                            radius: width / 2.,
                        })
                    } else {
                        lir::ElementKind::Arc(lir::Arc {
                            centre,
                            radius: draw_arc.radius,
                            width,
                            start_angle,
                            end_angle,
                        })
                    }
                }
                hir::OperationKind::DrawEllipse(draw_ellipse) => {
                    let height = draw_ellipse.width * draw_ellipse.ratio;
                    let direction = -draw_ellipse.direction * std::f32::consts::PI / 180.;

                    let centre = origin + draw_ellipse.centre;

                    last_point = Some(
                        centre
                            + Vector::new_from_length(
                                height,
                                direction + std::f32::consts::FRAC_PI_2,
                            ),
                    );

                    lir::ElementKind::Ellipse(lir::Ellipse {
                        centre,
                        sizes: Vector {
                            x: draw_ellipse.width,
                            y: height,
                        },
                        direction,
                        outline_width: width,
                    })
                }
                hir::OperationKind::DrawCurve(draw_curve) => {
                    let end = origin + draw_curve.end;

                    last_point = Some(end);

                    lir::ElementKind::Curve(lir::Curve {
                        start: origin + draw_curve.start,
                        end,
                        control: origin + draw_curve.control,
                        width,
                    })
                }
            },
        };

        elements.push(el);
    }

    (lir::IconLir { elements }, errors)
}
