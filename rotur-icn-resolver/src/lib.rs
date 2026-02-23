use rotur_icn_lexer::token;
use rotur_icn_units::{Colour, Vector};

use rotur_icn_lowerer::hir;

mod display;
mod error;
pub mod lir;

pub use error::{Error, ErrorKind};

pub fn resolve(
    operations: impl Iterator<Item = hir::Operation>,
) -> impl Iterator<Item = (Option<lir::Element>, Option<Error>)> {
    operations
        .enumerate()
        .scan(State::default(), |s, (op_i, op)| {
            let (kind, err) = match &op.kind {
                hir::OperationKind::SetWidth(set_width) => {
                    resolve_set_width(s, set_width);
                    (None, None)
                }
                hir::OperationKind::SetColour(set_colour) => {
                    resolve_set_colour(s, set_colour);
                    (None, None)
                }
                hir::OperationKind::DrawLine(draw_line) => {
                    (Some(resolve_draw_line(s, draw_line)), None)
                }
                hir::OperationKind::ContinueLine(continue_line) => {
                    resolve_continue_line(s, continue_line, op.cmd_span, op_i)
                        .map_or_else(|err| (None, err), |kind| (Some(kind), None))
                }
                hir::OperationKind::DrawDisk(draw_disk) => {
                    (Some(resolve_draw_disk(s, draw_disk)), None)
                }
                hir::OperationKind::DrawRectangle(draw_rectangle) => {
                    (Some(resolve_draw_rectangle(s, draw_rectangle)), None)
                }
                hir::OperationKind::DrawTriangle(draw_triangle) => {
                    (Some(resolve_draw_triangle(s, draw_triangle)), None)
                }
                hir::OperationKind::MoveCentre(move_centre) => {
                    resolve_move_centre(s, move_centre);
                    (None, None)
                }
                hir::OperationKind::ResetCentre(reset_centre) => {
                    resolve_reset_centre(s, reset_centre);
                    (None, None)
                }
                hir::OperationKind::DrawArc(draw_arc) => {
                    (Some(resolve_draw_arc(s, draw_arc)), None)
                }
                hir::OperationKind::DrawEllipse(draw_ellipse) => {
                    (Some(resolve_draw_ellipse(s, draw_ellipse)), None)
                }
                hir::OperationKind::DrawCurve(draw_curve) => {
                    (Some(resolve_draw_curve(s, draw_curve)), None)
                }
            };

            Some((
                kind.map(|kind| lir::Element {
                    colour: s.colour,
                    kind,
                }),
                err,
            ))
        })
}

struct State {
    origin: Vector,
    colour: Colour,
    width: f32,
    last_point: Option<Vector>,
    dangling_contlines_chained: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            origin: Vector::ZERO,
            colour: Colour::WHITE,
            width: 5.,
            last_point: None,
            dangling_contlines_chained: false,
        }
    }
}

impl State {
    fn set_last_point(&mut self, last_point: Option<Vector>) {
        self.last_point = last_point;
        self.dangling_contlines_chained = false;
    }
}

fn resolve_set_width(s: &mut State, op: &hir::SetWidth) {
    s.width = op.value;
}

fn resolve_set_colour(s: &mut State, op: &hir::SetColour) {
    s.colour = op.value;
}

fn resolve_draw_line(s: &mut State, op: &hir::DrawLine) -> lir::ElementKind {
    let end = s.origin + op.end;

    s.set_last_point(Some(end));

    // FIXME proper f32 comp
    if op.start == op.end {
        lir::ElementKind::Disk(lir::Disk {
            centre: end,
            radius: s.width / 2.,
        })
    } else {
        lir::ElementKind::Line(lir::Line {
            start: s.origin + op.start,
            end,
            width: s.width,
        })
    }
}

fn resolve_continue_line(
    s: &mut State,
    op: &hir::ContinueLine,
    span: token::Span,
    index: usize,
) -> Result<lir::ElementKind, Option<Error>> {
    if let Some(start) = s.last_point {
        debug_assert!(
            !s.dangling_contlines_chained,
            "this shouldn't happen as dangling continued lines don't define a last point",
        );

        let end = s.origin + op.next;

        s.set_last_point(Some(end));

        // FIXME proper f32 comp
        Ok(if start == op.next {
            lir::ElementKind::Disk(lir::Disk {
                centre: end,
                radius: s.width / 2.,
            })
        } else {
            lir::ElementKind::Line(lir::Line {
                start,
                end,
                width: s.width,
            })
        })
    } else {
        Err((!s.dangling_contlines_chained).then(|| {
            s.dangling_contlines_chained = true;

            Error {
                cmd_span: span,
                cmd_index: index,
                kind: ErrorKind::DanglingContinuedLine,
            }
        }))
    }
}

fn resolve_draw_disk(s: &mut State, op: &hir::DrawDisk) -> lir::ElementKind {
    let centre = s.origin + op.centre;

    s.set_last_point(Some(centre));

    lir::ElementKind::Disk(lir::Disk {
        centre,
        radius: s.width / 2.,
    })
}

fn resolve_draw_rectangle(s: &mut State, op: &hir::DrawRectangle) -> lir::ElementKind {
    let bottom_left = s.origin + op.centre - op.sizes;

    if op.filled {
        s.set_last_point(None);
    } else {
        let top_right = op.centre + op.sizes;
        s.set_last_point(Some(s.origin + top_right));
    }

    lir::ElementKind::Rectangle(lir::Rectangle {
        bottom_left,
        sizes: op.sizes * 2.,
        filled: op.filled,
        outline_width: s.width,
    })
}

fn resolve_draw_triangle(s: &mut State, op: &hir::DrawTriangle) -> lir::ElementKind {
    s.set_last_point(None);

    // FIXME proper f32 comp
    if op.a == op.b && op.b == op.c {
        lir::ElementKind::Disk(lir::Disk {
            centre: s.origin + op.a,
            radius: s.width / 2.,
        })
    } else if op.a == op.b {
        lir::ElementKind::Line(lir::Line {
            start: s.origin + op.a,
            end: s.origin + op.c,
            width: s.width,
        })
    } else if op.b == op.c {
        lir::ElementKind::Line(lir::Line {
            start: s.origin + op.b,
            end: s.origin + op.a,
            width: s.width,
        })
    } else if op.c == op.a {
        lir::ElementKind::Line(lir::Line {
            start: s.origin + op.c,
            end: s.origin + op.b,
            width: s.width,
        })
    } else {
        lir::ElementKind::Triangle(lir::Triangle {
            a: s.origin + op.a,
            b: s.origin + op.b,
            c: s.origin + op.c,
            outline_width: s.width,
        })
    }
}

fn resolve_move_centre(s: &mut State, op: &hir::MoveCentre) {
    s.origin += op.change;
}

fn resolve_reset_centre(s: &mut State, _op: &hir::ResetCentre) {
    s.origin = Vector::ZERO;
}

fn resolve_draw_arc(s: &mut State, op: &hir::DrawArc) -> lir::ElementKind {
    let direction = (op.direction * 10.).to_radians();
    let arm_angle = op.arm_angle.to_radians();

    let start_angle = std::f32::consts::FRAC_PI_2 - (direction + arm_angle);
    let end_angle = std::f32::consts::FRAC_PI_2 - (direction - arm_angle);

    let centre = s.origin + op.centre;

    let start_point = centre + Vector::new_from_length(op.radius, start_angle);
    s.set_last_point(Some(start_point));

    // FIXME do relative margin
    if (op.arm_angle - 180.).abs() < 1e-7 {
        lir::ElementKind::Circle(lir::Circle {
            centre,
            radius: op.radius,
            width: s.width,
        })
    } else if op.radius.abs() < 1e-9 || op.arm_angle.abs() < 1e-9 {
        lir::ElementKind::Disk(lir::Disk {
            centre: start_point,
            radius: s.width / 2.,
        })
    } else {
        lir::ElementKind::Arc(lir::Arc {
            centre,
            radius: op.radius,
            width: s.width,
            start_angle,
            end_angle,
        })
    }
}

fn resolve_draw_ellipse(s: &mut State, op: &hir::DrawEllipse) -> lir::ElementKind {
    let minor = op.major * op.ratio;

    let direction = -op.direction.to_radians();

    let centre = s.origin + op.centre;
    s.set_last_point(Some(
        centre + Vector::new_from_length(minor, direction + std::f32::consts::FRAC_PI_2),
    ));

    if op.major < 1e-9 {
        lir::ElementKind::Disk(lir::Disk {
            centre,
            radius: s.width / 2.,
        })
    } else if op.ratio < 1e-9 {
        let to_end = Vector::new_from_length(op.major, direction);

        lir::ElementKind::Line(lir::Line {
            start: centre - to_end,
            end: centre + to_end,
            width: s.width,
        })
    } else {
        lir::ElementKind::Ellipse(lir::Ellipse {
            centre,
            axis: Vector {
                x: op.major,
                y: minor,
            },
            direction,
            outline_width: s.width,
        })
    }
}

fn resolve_draw_curve(s: &mut State, op: &hir::DrawCurve) -> lir::ElementKind {
    let end = s.origin + op.end;

    s.set_last_point(Some(end));

    lir::ElementKind::Curve(lir::Curve {
        start: s.origin + op.start,
        end,
        control: s.origin + op.control,
        width: s.width,
    })
}
