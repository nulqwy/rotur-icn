use rotur_icn_syntax::{lexer, parser::ast};

use hir::{
    ContinueLine, DrawArc, DrawCurve, DrawDisk, DrawEllipse, DrawLine, DrawRectangle, DrawTriangle,
    IconHir, MoveCentre, Operation, OperationKind, ResetCentre, SetColour, SetWidth,
};

mod display;
mod error;
pub mod hir;

pub use error::{Error, ErrorKind};
use rotur_icn_units::{Colour, Number, Vector};

pub fn lower(icon: &ast::Icon) -> (IconHir, Vec<Error>) {
    let mut errors = Vec::new();

    let mut operations = Vec::with_capacity(icon.commands.len());

    for (cmd_index, cmd) in icon.commands.iter().enumerate() {
        operations.push(match cmd.name {
            SetWidth::NAME => {
                validate_arg_count(&mut errors, cmd, cmd_index, 1);

                let mut value = get_number(&mut errors, cmd, cmd_index, 0);

                if !validate_arg_value(&mut errors, cmd, cmd_index, Some((0., true)), None, 0) {
                    value = 0.;
                }

                Operation {
                    cmd_pos: cmd.name_pos,
                    cmd_index,
                    kind: OperationKind::SetWidth(SetWidth { value }),
                }
            }
            SetColour::NAME => {
                validate_arg_count(&mut errors, cmd, cmd_index, 1);

                let value = get_colour(&mut errors, cmd, cmd_index, 0);

                Operation {
                    cmd_pos: cmd.name_pos,
                    cmd_index,
                    kind: OperationKind::SetColour(SetColour { value }),
                }
            }
            DrawLine::NAME => {
                validate_arg_count(&mut errors, cmd, cmd_index, 4);

                let start = get_vector(&mut errors, cmd, cmd_index, 0);
                let end = get_vector(&mut errors, cmd, cmd_index, 2);

                Operation {
                    cmd_pos: cmd.name_pos,
                    cmd_index,
                    kind: OperationKind::DrawLine(DrawLine { start, end }),
                }
            }
            ContinueLine::NAME => {
                validate_arg_count(&mut errors, cmd, cmd_index, 2);

                let next = get_vector(&mut errors, cmd, cmd_index, 0);

                Operation {
                    cmd_pos: cmd.name_pos,
                    cmd_index,
                    kind: OperationKind::ContinueLine(ContinueLine { next }),
                }
            }
            DrawDisk::NAME => {
                validate_arg_count(&mut errors, cmd, cmd_index, 2);

                let centre = get_vector(&mut errors, cmd, cmd_index, 0);

                Operation {
                    cmd_pos: cmd.name_pos,
                    cmd_index,
                    kind: OperationKind::DrawDisk(DrawDisk { centre }),
                }
            }
            kw @ (DrawRectangle::NAME_HOLLOW | DrawRectangle::NAME_FILLED) => {
                validate_arg_count(&mut errors, cmd, cmd_index, 4);

                let centre = get_vector(&mut errors, cmd, cmd_index, 0);
                let mut sizes = get_vector(&mut errors, cmd, cmd_index, 2);

                if !validate_arg_value(&mut errors, cmd, cmd_index, Some((0., true)), None, 2) {
                    sizes.x = 0.;
                }

                if !validate_arg_value(&mut errors, cmd, cmd_index, Some((0., true)), None, 3) {
                    sizes.y = 0.;
                }

                Operation {
                    cmd_pos: cmd.name_pos,
                    cmd_index,
                    kind: OperationKind::DrawRectangle(DrawRectangle {
                        centre,
                        sizes,
                        filled: kw == DrawRectangle::NAME_FILLED,
                    }),
                }
            }
            DrawTriangle::NAME => {
                validate_arg_count(&mut errors, cmd, cmd_index, 6);

                let a = get_vector(&mut errors, cmd, cmd_index, 0);
                let b = get_vector(&mut errors, cmd, cmd_index, 2);
                let c = get_vector(&mut errors, cmd, cmd_index, 4);

                Operation {
                    cmd_pos: cmd.name_pos,
                    cmd_index,
                    kind: OperationKind::DrawTriangle(DrawTriangle { a, b, c }),
                }
            }
            MoveCentre::NAME => {
                validate_arg_count(&mut errors, cmd, cmd_index, 2);

                let change = get_vector(&mut errors, cmd, cmd_index, 0);

                Operation {
                    cmd_pos: cmd.name_pos,
                    cmd_index,
                    kind: OperationKind::MoveCentre(MoveCentre { change }),
                }
            }
            ResetCentre::NAME => {
                validate_arg_count(&mut errors, cmd, cmd_index, 0);

                Operation {
                    cmd_pos: cmd.name_pos,
                    cmd_index,
                    kind: OperationKind::ResetCentre(ResetCentre),
                }
            }
            DrawArc::NAME => {
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

                Operation {
                    cmd_pos: cmd.name_pos,
                    cmd_index,
                    kind: OperationKind::DrawArc(DrawArc {
                        centre,
                        radius,
                        direction,
                        arm_angle,
                    }),
                }
            }
            DrawEllipse::NAME => {
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

                Operation {
                    cmd_pos: cmd.name_pos,
                    cmd_index,
                    kind: OperationKind::DrawEllipse(DrawEllipse {
                        centre,
                        width,
                        ratio,
                        direction,
                    }),
                }
            }
            DrawCurve::NAME => {
                validate_arg_count(&mut errors, cmd, cmd_index, 6);

                let start = get_vector(&mut errors, cmd, cmd_index, 0);
                let end = get_vector(&mut errors, cmd, cmd_index, 2);
                let control = get_vector(&mut errors, cmd, cmd_index, 4);

                Operation {
                    cmd_pos: cmd.name_pos,
                    cmd_index,
                    kind: OperationKind::DrawCurve(DrawCurve {
                        start,
                        control,
                        end,
                    }),
                }
            }
            _ => {
                errors.push(Error {
                    cmd_pos: cmd.name_pos,
                    cmd_index,
                    kind: ErrorKind::InvalidCommand,
                });

                continue;
            }
        });
    }

    (IconHir { operations }, errors)
}

fn validate_arg_count(
    errors: &mut Vec<Error>,
    cmd: &ast::Command,
    cmd_index: usize,
    count: usize,
) -> bool {
    if cmd.args.len() > count {
        errors.push(Error {
            cmd_pos: cmd.name_pos,
            cmd_index,
            kind: ErrorKind::TooManyArguments {
                overflow_pos: (cmd.args[count].pos.0, cmd.args.last().unwrap().pos.1),
                exp: count,
                got: cmd.args.len(),
            },
        });
        true
    } else if cmd.args.len() < count {
        errors.push(Error {
            cmd_pos: cmd.name_pos,
            cmd_index,
            kind: ErrorKind::TooFewArguments {
                args_end_loc: cmd
                    .args
                    .last()
                    .map(|arg| arg.pos.1)
                    .unwrap_or(cmd.name_pos.1),
                exp: count,
                got: cmd.args.len(),
            },
        });
        true
    } else {
        false
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
        lit: lexer::Literal::Number(value),
        pos: arg_pos,
    }) = cmd.args.get(i)
    else {
        return false;
    };

    let mut valid = true;

    if let Some((bound, inclusive)) = range_start
        && (value < &bound || (!inclusive && value == &bound))
    {
        valid = false;
    }

    if let Some((bound, inclusive)) = range_end
        && (value > &bound || (!inclusive && value == &bound))
    {
        valid = false;
    }

    if !valid {
        errors.push(Error {
            cmd_pos: cmd.name_pos,
            cmd_index,
            kind: ErrorKind::ArgOutOfRange {
                arg_pos: *arg_pos,
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
        .map(|lit| match lit.lit {
            lexer::Literal::Number(n) => n,
            lexer::Literal::Colour(col) => {
                errors.push(Error {
                    cmd_pos: cmd.name_pos,
                    cmd_index,
                    kind: ErrorKind::UnexpectedLiteralKind {
                        arg_pos: lit.pos,
                        arg_index: i,
                        exp: lexer::LiteralKind::Number,
                        got: lexer::LiteralKind::Colour,
                    },
                });

                let n: u32 = col.into();

                n as Number
            }
        })
        .unwrap_or_default()
}

fn get_colour(errors: &mut Vec<Error>, cmd: &ast::Command, cmd_index: usize, i: usize) -> Colour {
    cmd.args
        .get(i)
        .map(|lit| match lit.lit {
            lexer::Literal::Colour(col) => Colour {
                r: col.r,
                g: col.g,
                b: col.b,
                a: 0xff,
            },
            lexer::Literal::Number(n) => {
                let int = n as u32;
                let [b, g, r, overflow] = int.to_le_bytes();

                if overflow != 0 {
                    errors.push(Error {
                        cmd_pos: cmd.name_pos,
                        cmd_index,
                        kind: ErrorKind::InvalidNumericColour {
                            arg_pos: lit.pos,
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
