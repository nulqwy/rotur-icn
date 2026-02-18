use std::cmp::Ordering;

use rotur_icn_lexer::token;
use rotur_icn_parser::ast;
use rotur_icn_units::{Colour, Number, Vector};

mod display;
mod error;
pub mod hir;

pub use error::{Error, ErrorKind};
use hir::{
    ContinueLine, DrawArc, DrawCurve, DrawDisk, DrawEllipse, DrawLine, DrawRectangle, DrawTriangle,
    MoveCentre, Operation, OperationKind, ResetCentre, SetColour, SetWidth,
};

pub fn lower<'s>(
    commands: impl Iterator<Item = ast::Command<'s>>,
) -> impl Iterator<Item = (Option<hir::Operation>, Vec<Error>)> {
    commands
        .enumerate()
        .map(|(cmd_index, cmd)| lower_command(&cmd, cmd_index))
}

fn lower_command(cmd: &ast::Command, cmd_index: usize) -> (Option<hir::Operation>, Vec<Error>) {
    let (kind, errors) = match cmd.name {
        SetWidth::NAME => lower_set_width(cmd, cmd_index),
        SetColour::NAME => lower_set_colour(cmd, cmd_index),
        DrawLine::NAME => lower_draw_line(cmd, cmd_index),
        ContinueLine::NAME => lower_continue_line(cmd, cmd_index),
        DrawDisk::NAME => lower_draw_disk(cmd, cmd_index),
        DrawRectangle::NAME_HOLLOW => lower_draw_rectangle(cmd, cmd_index, false),
        DrawRectangle::NAME_FILLED => lower_draw_rectangle(cmd, cmd_index, true),
        DrawTriangle::NAME => lower_draw_triangle(cmd, cmd_index),
        MoveCentre::NAME => lower_move_centre(cmd, cmd_index),
        ResetCentre::NAME => lower_reset_centre(cmd, cmd_index),
        DrawArc::NAME => lower_draw_arc(cmd, cmd_index),
        DrawEllipse::NAME => lower_draw_ellipse(cmd, cmd_index),
        DrawCurve::NAME => lower_draw_curve(cmd, cmd_index),
        _ => {
            return (
                None,
                vec![Error {
                    cmd_span: cmd.name_span,
                    cmd_index,
                    kind: ErrorKind::InvalidCommand,
                }],
            );
        }
    };

    (
        Some(Operation {
            cmd_span: cmd.name_span,
            kind,
        }),
        errors,
    )
}

fn lower_set_width(cmd: &ast::Command, cmd_index: usize) -> (hir::OperationKind, Vec<Error>) {
    let mut errors = Vec::new();

    validate_arg_count(&mut errors, cmd, cmd_index, 1);

    let mut value = get_number(&mut errors, cmd, cmd_index, 0);

    if !validate_arg_value(&mut errors, cmd, cmd_index, Some((0., true)), None, 0) {
        value = 0.;
    }

    (OperationKind::SetWidth(SetWidth { value }), errors)
}

fn lower_set_colour(cmd: &ast::Command, cmd_index: usize) -> (hir::OperationKind, Vec<Error>) {
    let mut errors = Vec::new();

    validate_arg_count(&mut errors, cmd, cmd_index, 1);

    let value = get_colour(&mut errors, cmd, cmd_index, 0);

    (OperationKind::SetColour(SetColour { value }), errors)
}

fn lower_draw_line(cmd: &ast::Command, cmd_index: usize) -> (hir::OperationKind, Vec<Error>) {
    let mut errors = Vec::new();

    validate_arg_count(&mut errors, cmd, cmd_index, 4);

    let start = get_vector(&mut errors, cmd, cmd_index, 0);
    let end = get_vector(&mut errors, cmd, cmd_index, 2);

    (OperationKind::DrawLine(DrawLine { start, end }), errors)
}

fn lower_continue_line(cmd: &ast::Command, cmd_index: usize) -> (hir::OperationKind, Vec<Error>) {
    let mut errors = Vec::new();

    validate_arg_count(&mut errors, cmd, cmd_index, 2);

    let next = get_vector(&mut errors, cmd, cmd_index, 0);

    (OperationKind::ContinueLine(ContinueLine { next }), errors)
}

fn lower_draw_disk(cmd: &ast::Command, cmd_index: usize) -> (hir::OperationKind, Vec<Error>) {
    let mut errors = Vec::new();

    validate_arg_count(&mut errors, cmd, cmd_index, 2);

    let centre = get_vector(&mut errors, cmd, cmd_index, 0);

    (OperationKind::DrawDisk(DrawDisk { centre }), errors)
}

fn lower_draw_rectangle(
    cmd: &ast::Command,
    cmd_index: usize,
    filled: bool,
) -> (hir::OperationKind, Vec<Error>) {
    let mut errors = Vec::new();

    validate_arg_count(&mut errors, cmd, cmd_index, 4);

    let centre = get_vector(&mut errors, cmd, cmd_index, 0);
    let mut sizes = get_vector(&mut errors, cmd, cmd_index, 2);

    if !validate_arg_value(&mut errors, cmd, cmd_index, Some((0., true)), None, 2) {
        sizes.x = 0.;
    }

    if !validate_arg_value(&mut errors, cmd, cmd_index, Some((0., true)), None, 3) {
        sizes.y = 0.;
    }

    (
        OperationKind::DrawRectangle(DrawRectangle {
            centre,
            sizes,
            filled,
        }),
        errors,
    )
}

fn lower_draw_triangle(cmd: &ast::Command, cmd_index: usize) -> (hir::OperationKind, Vec<Error>) {
    let mut errors = Vec::new();

    validate_arg_count(&mut errors, cmd, cmd_index, 6);

    let a = get_vector(&mut errors, cmd, cmd_index, 0);
    let b = get_vector(&mut errors, cmd, cmd_index, 2);
    let c = get_vector(&mut errors, cmd, cmd_index, 4);

    (
        OperationKind::DrawTriangle(DrawTriangle { a, b, c }),
        errors,
    )
}

fn lower_move_centre(cmd: &ast::Command, cmd_index: usize) -> (hir::OperationKind, Vec<Error>) {
    let mut errors = Vec::new();

    validate_arg_count(&mut errors, cmd, cmd_index, 2);

    let change = get_vector(&mut errors, cmd, cmd_index, 0);

    (OperationKind::MoveCentre(MoveCentre { change }), errors)
}

fn lower_reset_centre(cmd: &ast::Command, cmd_index: usize) -> (hir::OperationKind, Vec<Error>) {
    let mut errors = Vec::new();

    validate_arg_count(&mut errors, cmd, cmd_index, 0);

    (OperationKind::ResetCentre(ResetCentre), errors)
}

fn lower_draw_arc(cmd: &ast::Command, cmd_index: usize) -> (hir::OperationKind, Vec<Error>) {
    let mut errors = Vec::new();

    validate_arg_count(&mut errors, cmd, cmd_index, 5);

    let centre = get_vector(&mut errors, cmd, cmd_index, 0);
    let mut radius = get_number(&mut errors, cmd, cmd_index, 2);
    let direction = get_number(&mut errors, cmd, cmd_index, 3);
    let mut arm_angle = get_number(&mut errors, cmd, cmd_index, 4);

    if !validate_arg_value(&mut errors, cmd, cmd_index, Some((0., true)), None, 2) {
        radius = 0.;
    }

    if !validate_arg_value(
        &mut errors,
        cmd,
        cmd_index,
        Some((0., true)),
        Some((180.0, true)),
        4,
    ) {
        arm_angle = arm_angle.clamp(0.0, 180.0);
    }

    (
        OperationKind::DrawArc(DrawArc {
            centre,
            radius,
            direction,
            arm_angle,
        }),
        errors,
    )
}

fn lower_draw_ellipse(cmd: &ast::Command, cmd_index: usize) -> (hir::OperationKind, Vec<Error>) {
    let mut errors = Vec::new();

    validate_arg_count(&mut errors, cmd, cmd_index, 5);

    let centre = get_vector(&mut errors, cmd, cmd_index, 0);
    let mut width = get_number(&mut errors, cmd, cmd_index, 2);
    let mut ratio = get_number(&mut errors, cmd, cmd_index, 3);
    let direction = get_number(&mut errors, cmd, cmd_index, 4);

    if !validate_arg_value(&mut errors, cmd, cmd_index, Some((0., true)), None, 2) {
        width = 0.;
    }

    if !validate_arg_value(&mut errors, cmd, cmd_index, Some((0., true)), None, 2) {
        ratio = 0.;
    }

    (
        OperationKind::DrawEllipse(DrawEllipse {
            centre,
            major: width,
            ratio,
            direction,
        }),
        errors,
    )
}

fn lower_draw_curve(cmd: &ast::Command, cmd_index: usize) -> (hir::OperationKind, Vec<Error>) {
    let mut errors = Vec::new();

    validate_arg_count(&mut errors, cmd, cmd_index, 6);

    let start = get_vector(&mut errors, cmd, cmd_index, 0);
    let end = get_vector(&mut errors, cmd, cmd_index, 2);
    let control = get_vector(&mut errors, cmd, cmd_index, 4);

    (
        OperationKind::DrawCurve(DrawCurve {
            start,
            control,
            end,
        }),
        errors,
    )
}

fn validate_arg_count(
    errors: &mut Vec<Error>,
    cmd: &ast::Command,
    cmd_index: usize,
    count: usize,
) -> bool {
    match cmd.args.len().cmp(&count) {
        Ordering::Greater => {
            errors.push(Error {
                cmd_span: cmd.name_span,
                cmd_index,
                kind: ErrorKind::TooManyArguments {
                    overflow_span: (cmd.args[count].span.0, cmd.args.last().unwrap().span.1),
                    exp: count,
                    got: cmd.args.len(),
                },
            });
            true
        }
        Ordering::Less => {
            errors.push(Error {
                cmd_span: cmd.name_span,
                cmd_index,
                kind: ErrorKind::TooFewArguments {
                    args_end_loc: cmd.args.last().map_or(cmd.name_span.1, |arg| arg.span.1),
                    exp: count,
                    got: cmd.args.len(),
                },
            });
            true
        }
        Ordering::Equal => false,
    }
}

fn validate_arg_value(
    errors: &mut Vec<Error>,
    cmd: &ast::Command,
    cmd_index: usize,
    range_start: Option<(Number, bool)>,
    range_end: Option<(Number, bool)>,
    i: usize,
) -> bool {
    let Some(ast::Argument {
        literal: token::Literal::Number(value),
        span: arg_span,
    }) = cmd.args.get(i)
    else {
        return false;
    };

    let mut valid = true;

    // FIXME do relative margin
    if let Some((bound, inclusive)) = range_start
        && (value < &bound || (!inclusive && (value - bound).abs() < 1e-7))
    {
        valid = false;
    }

    // FIXME [above]
    if let Some((bound, inclusive)) = range_end
        && (value > &bound || (!inclusive && (value - bound).abs() < 1e-7))
    {
        valid = false;
    }

    if !valid {
        errors.push(Error {
            cmd_span: cmd.name_span,
            cmd_index,
            kind: ErrorKind::ArgOutOfRange {
                arg_span: *arg_span,
                arg_index: i,
                range_start,
                range_end,
            },
        });
    }

    valid
}

fn get_number(errors: &mut Vec<Error>, cmd: &ast::Command, cmd_index: usize, i: usize) -> Number {
    cmd.args
        .get(i)
        .map(|lit| match lit.literal {
            token::Literal::Number(n) => n,
            token::Literal::Colour(col) => {
                errors.push(Error {
                    cmd_span: cmd.name_span,
                    cmd_index,
                    kind: ErrorKind::UnexpectedLiteralKind {
                        arg_span: lit.span,
                        arg_index: i,
                        exp: token::LiteralKind::Number,
                        got: token::LiteralKind::Colour,
                    },
                });

                let n: u32 = col.into();

                #[expect(clippy::cast_precision_loss, reason = "it should just fit")]
                let n = n as Number;
                n
            }
        })
        .unwrap_or_default()
}

fn get_colour(errors: &mut Vec<Error>, cmd: &ast::Command, cmd_index: usize, i: usize) -> Colour {
    cmd.args
        .get(i)
        .map(|lit| match lit.literal {
            token::Literal::Colour(col) => Colour {
                r: col.r,
                g: col.g,
                b: col.b,
                a: 0xff,
            },
            token::Literal::Number(num) => {
                #[expect(
                    clippy::cast_possible_truncation,
                    reason = "colours shouldn't be that large anyways"
                )]
                #[expect(clippy::cast_sign_loss, reason = "absolute is taken")]
                let int = num.round().abs() as u32;
                let [b, g, r, overflow] = int.to_le_bytes();

                if overflow != 0 {
                    errors.push(Error {
                        cmd_span: cmd.name_span,
                        cmd_index,
                        kind: ErrorKind::InvalidNumericColour {
                            arg_span: lit.span,
                            arg_index: i,
                        },
                    });
                }

                Colour { r, g, b, a: 0xff }
            }
        })
        .unwrap_or_default()
}

fn get_vector(errors: &mut Vec<Error>, cmd: &ast::Command, cmd_index: usize, i: usize) -> Vector {
    Vector {
        x: get_number(errors, cmd, cmd_index, i),
        y: get_number(errors, cmd, cmd_index, i + 1),
    }
}
