use rotur_icn_syntax::{lexer, parser::ast};

use ir::{
    Colour, ContinueLine, DrawArc, DrawCurve, DrawDot, DrawEllipse, DrawLine, DrawRectangle,
    DrawTriangle, IconIr, Instruction, MoveCentre, ResetCentre, SetColour, SetWidth, Vector,
};

mod display;
mod error;
pub mod ir;

pub use error::{Error, ErrorKind};

pub fn lower(icon: ast::Icon) -> (IconIr, Vec<Error>) {
    let mut errors = Vec::new();

    let mut instructions = Vec::with_capacity(icon.commands.len());

    for cmd in icon.commands {
        instructions.push(match cmd.name {
            SetWidth::NAME => {
                validate_arg_count(&mut errors, &cmd, 1);

                let value = get_number(&mut errors, &cmd, 0);

                Instruction::SetWidth(SetWidth { value })
            }
            SetColour::NAME => {
                validate_arg_count(&mut errors, &cmd, 1);

                let value = get_colour(&mut errors, &cmd, 0);

                Instruction::SetColour(SetColour { value })
            }
            DrawLine::NAME => {
                validate_arg_count(&mut errors, &cmd, 4);

                let start = get_vector(&mut errors, &cmd, 0);
                let end = get_vector(&mut errors, &cmd, 2);

                Instruction::DrawLine(DrawLine { start, end })
            }
            ContinueLine::NAME => {
                validate_arg_count(&mut errors, &cmd, 2);

                let next = get_vector(&mut errors, &cmd, 0);

                Instruction::ContinueLine(ContinueLine { next })
            }
            DrawDot::NAME => {
                validate_arg_count(&mut errors, &cmd, 2);

                let pos = get_vector(&mut errors, &cmd, 0);

                Instruction::DrawDot(DrawDot { pos })
            }
            kw @ (DrawRectangle::NAME_HOLLOW | DrawRectangle::NAME_FILLED) => {
                validate_arg_count(&mut errors, &cmd, 4);

                let centre = get_vector(&mut errors, &cmd, 0);
                let sizes = get_vector(&mut errors, &cmd, 2);

                Instruction::DrawRectangle(DrawRectangle {
                    centre,
                    sizes,
                    filled: kw == DrawRectangle::NAME_FILLED,
                })
            }
            DrawTriangle::NAME => {
                validate_arg_count(&mut errors, &cmd, 6);

                let a = get_vector(&mut errors, &cmd, 0);
                let b = get_vector(&mut errors, &cmd, 2);
                let c = get_vector(&mut errors, &cmd, 4);

                Instruction::DrawTriangle(DrawTriangle { a, b, c })
            }
            MoveCentre::NAME => {
                validate_arg_count(&mut errors, &cmd, 2);

                let change = get_vector(&mut errors, &cmd, 0);

                Instruction::MoveCentre(MoveCentre { change })
            }
            ResetCentre::NAME => {
                validate_arg_count(&mut errors, &cmd, 0);

                Instruction::ResetCentre(ResetCentre)
            }
            DrawArc::NAME => {
                validate_arg_count(&mut errors, &cmd, 5);

                let centre = get_vector(&mut errors, &cmd, 0);
                let radius = get_number(&mut errors, &cmd, 2);
                let angle = get_number(&mut errors, &cmd, 3) * std::f64::consts::PI / 18.0;
                let filled = get_number(&mut errors, &cmd, 4) * std::f64::consts::PI / 180.0;

                let dist_angle = angle - filled;

                let end_angle = std::f64::consts::FRAC_PI_2 - dist_angle;
                let start_angle = end_angle - (filled * 2.0);

                Instruction::DrawArc(DrawArc {
                    centre,
                    radius,
                    start_angle,
                    end_angle,
                })
            }
            DrawEllipse::NAME => {
                validate_arg_count(&mut errors, &cmd, 5);

                let centre = get_vector(&mut errors, &cmd, 0);
                let width = get_number(&mut errors, &cmd, 2);
                let ratio = get_number(&mut errors, &cmd, 3);
                let facing = -get_number(&mut errors, &cmd, 4) * std::f64::consts::PI / 180.0;

                let sizes = Vector {
                    x: width,
                    y: width * ratio,
                };

                Instruction::DrawEllipse(DrawEllipse {
                    centre,
                    sizes,
                    facing,
                })
            }
            DrawCurve::NAME => {
                validate_arg_count(&mut errors, &cmd, 6);

                let start = get_vector(&mut errors, &cmd, 0);
                let end = get_vector(&mut errors, &cmd, 2);
                let control = get_vector(&mut errors, &cmd, 4);

                Instruction::DrawCurve(DrawCurve {
                    start,
                    end,
                    control,
                })
            }
            _ => {
                errors.push(Error {
                    cmd_pos: cmd.name_pos,
                    kind: ErrorKind::InvalidCommand,
                });

                continue;
            }
        });
    }

    (IconIr { instructions }, errors)
}

fn validate_arg_count(errors: &mut Vec<Error>, cmd: &ast::Command, count: usize) -> bool {
    if cmd.args.len() > count {
        errors.push(Error {
            cmd_pos: cmd.name_pos,
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

fn get_number(errors: &mut Vec<Error>, cmd: &ast::Command, i: usize) -> f64 {
    cmd.args
        .get(i)
        .map(|lit| match lit.lit {
            lexer::Literal::Number(n) => n.value,
            lexer::Literal::Colour(col) => {
                errors.push(Error {
                    cmd_pos: cmd.name_pos,
                    kind: ErrorKind::UnexpectedLiteralKind {
                        arg_pos: lit.pos,
                        arg_index: i,
                        exp: lexer::LiteralKind::Number,
                        got: lexer::LiteralKind::Colour,
                    },
                });

                let n = u32::from_le_bytes([col.b, col.g, col.r, 0]);
                n as f64
            }
        })
        .unwrap_or(0.0)
}

fn get_colour(errors: &mut Vec<Error>, cmd: &ast::Command, i: usize) -> Colour {
    cmd.args
        .get(i)
        .map(|lit| match lit.lit {
            lexer::Literal::Colour(col) => Colour {
                r: col.r,
                g: col.g,
                b: col.b,
            },
            lexer::Literal::Number(n) => {
                let int = n.value as u32;
                let [b, g, r, overflow] = int.to_le_bytes();

                if overflow != 0 {
                    errors.push(Error {
                        cmd_pos: cmd.name_pos,
                        kind: ErrorKind::InvalidNumericColour {
                            arg_pos: lit.pos,
                            arg_index: i,
                        },
                    });
                }

                Colour { r, g, b }
            }
        })
        .unwrap_or_default()
}

fn get_vector(errors: &mut Vec<Error>, cmd: &ast::Command, i: usize) -> Vector {
    Vector {
        x: get_number(errors, cmd, i),
        y: get_number(errors, cmd, i + 1),
    }
}
