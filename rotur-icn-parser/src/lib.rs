use arrayvec::ArrayVec;
use ast::{Argument, Command, Icon};

pub mod ast;
mod display;
mod error;

pub use error::Error;

use rotur_icn_lexer::token;

pub fn parse<'s>(tokens: impl Iterator<Item = token::PToken<'s>>) -> (Icon<'s>, Vec<Error>) {
    let mut errors = Vec::new();

    let mut commands = Vec::new();

    let mut command: Option<(&'s str, token::Pos)> = None;
    let mut arguments = ArrayVec::new();
    let mut is_capturing_error = false;
    let mut is_capturing_overflow = false;
    let mut err_l_loc = None;
    let mut prev_r_loc = token::Loc::default();
    for (l, token, r) in tokens {
        match token {
            token::Token::Identifier(ident) => {
                if let Some(err_l_loc) = err_l_loc.take() {
                    if is_capturing_overflow {
                        errors.push(Error::TooManyArguments {
                            #[expect(clippy::missing_panics_doc, reason = "for bug catching")]
                            keyword_pos: command.expect("they shouldn't be stranded, as no capture happens during stranded handling").1,
                            overflow_pos: (err_l_loc, prev_r_loc),
                        });
                    } else {
                        errors.push(Error::StrandedArguments {
                            stranded_pos: (err_l_loc, prev_r_loc),
                        });
                    }

                    is_capturing_error = false;
                }

                if let Some((cmd, cmd_pos)) = command.take() {
                    commands.push(Command {
                        name: cmd,
                        name_pos: cmd_pos,
                        args: std::mem::take(&mut arguments),
                    });
                }

                #[expect(clippy::missing_panics_doc, reason = "for bug catching")]
                {
                    assert!(command.is_none(), "command should have been pushed");
                }

                command = Some((ident.value, (l, r)));
            }
            token::Token::Literal(lit) => {
                if command.is_none() && !is_capturing_error {
                    #[expect(clippy::missing_panics_doc, reason = "for bug catching")]
                    {
                        assert!(
                            err_l_loc.is_none(),
                            "no other error should be getting captured atm"
                        );
                    }
                    err_l_loc = Some(l);
                    is_capturing_overflow = false;
                    is_capturing_error = true;
                } else {
                    let push_res = arguments.try_push(Argument { lit, pos: (l, r) });
                    if push_res.is_err() && !is_capturing_error {
                        #[expect(clippy::missing_panics_doc, reason = "for bug catching")]
                        {
                            assert!(
                                err_l_loc.is_none(),
                                "no other error should be getting captured atm"
                            );
                        }
                        err_l_loc = Some(l);
                        is_capturing_overflow = true;
                        is_capturing_error = true;
                    }
                }
            }
        }

        prev_r_loc = r;
    }

    if let Some((cmd, cmd_pos)) = command {
        commands.push(Command {
            name: cmd,
            name_pos: cmd_pos,
            args: arguments,
        });
    }

    (Icon { commands }, errors)
}
