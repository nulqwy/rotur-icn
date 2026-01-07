use arrayvec::ArrayVec;
use ast::*;

pub mod ast;
mod display;
mod error;

pub use error::Error;

use crate::lexer;

pub fn parse<'s>(tokens: impl Iterator<Item = lexer::PToken<'s>>) -> (Icon<'s>, Vec<Error>) {
    let mut errors = Vec::new();

    let mut commands = Vec::new();

    let mut keyword: Option<(&'s str, lexer::Pos)> = None;
    let mut arguments = ArrayVec::new();
    let mut is_capturing_error = false;
    let mut is_capturing_overflow = false;
    let mut err_l_loc = None;
    let mut prev_r_loc = lexer::Loc::default();
    for (l, token, r) in tokens {
        match token {
            lexer::Token::Keyword(kw) => {
                if let Some(err_l_loc) = err_l_loc.take() {
                    if is_capturing_overflow {
                        errors.push(Error::TooManyArguments {
                            keyword_pos: keyword.expect("they shouldn't be stranded").1,
                            overflow_pos: (err_l_loc, prev_r_loc),
                        });
                    } else {
                        errors.push(Error::StrandedArguments {
                            stranded_pos: (err_l_loc, prev_r_loc),
                        });
                    }

                    is_capturing_error = false;
                }

                if let Some((kw, kw_pos)) = keyword.take() {
                    commands.push(Command {
                        name: kw,
                        name_pos: kw_pos,
                        args: std::mem::take(&mut arguments),
                    });
                }

                // just in case the code above gets changed and i forget
                assert!(keyword.is_none(), "command should have been pushed");

                keyword = Some((kw.value, (l, r)));
            }
            lexer::Token::Literal(lit) => {
                if keyword.is_none() && !is_capturing_error {
                    assert!(
                        err_l_loc.is_none(),
                        "no other error should be getting captured atm"
                    );
                    err_l_loc = Some(l);
                    is_capturing_overflow = false;
                    is_capturing_error = true;
                } else {
                    let push_res = arguments.try_push(Argument { lit, pos: (l, r) });
                    if push_res.is_err() && !is_capturing_error {
                        assert!(
                            err_l_loc.is_none(),
                            "no other error should be getting captured atm"
                        );
                        err_l_loc = Some(l);
                        is_capturing_overflow = true;
                        is_capturing_error = true;
                    }
                }
            }
        }

        prev_r_loc = r;
    }

    if let Some((kw, kw_pos)) = keyword {
        commands.push(Command {
            name: kw,
            name_pos: kw_pos,
            args: arguments,
        });
    }

    (Icon { commands }, errors)
}
