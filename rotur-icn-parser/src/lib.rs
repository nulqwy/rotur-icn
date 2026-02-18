// TODO try rewriting with .scan()

use arrayvec::ArrayVec;

pub mod ast;
mod display;
mod error;

pub use error::Error;

use rotur_icn_lexer::token;

#[derive(Debug, Clone)]
pub struct Parser<'s, L>
where
    L: Iterator<Item = token::PToken<'s>>,
{
    lexer: L,
    command: Option<(&'s str, token::Span)>,
    arguments: ArrayVec<ast::Argument, 6>,
    capturing_error: Option<(token::Loc, CapturedErrorKind)>,
    previous_right_loc: token::Loc,
}

#[derive(Debug, Clone)]
enum CapturedErrorKind {
    Overflow,
    Stranded,
}

impl<'s, L> Parser<'s, L>
where
    L: Iterator<Item = token::PToken<'s>>,
{
    pub fn new(lexer: L) -> Self {
        Self {
            lexer,
            command: None,
            arguments: ArrayVec::new(),
            capturing_error: None,
            previous_right_loc: token::Loc::default(),
        }
    }
}

impl<'s, L> Iterator for Parser<'s, L>
where
    L: Iterator<Item = token::PToken<'s>>,
{
    type Item = (ast::Command<'s>, Option<Error>);

    fn next(&mut self) -> Option<Self::Item> {
        let mut error = None;
        let mut set_error = |e| {
            assert!(error.is_none(), "only 1 error should happen at a time");
            error = Some(e);
        };

        for (l, token, r) in &mut self.lexer {
            match token {
                token::Token::Identifier(ident) => {
                    match self.capturing_error {
                        Some((err_l_loc, CapturedErrorKind::Overflow)) => {
                            set_error(Error::TooManyArguments {
                                keyword_span: self.command.expect("they shouldn't be stranded, as no capture happens during stranded handling").1,
                                overflow_span: (err_l_loc, self.previous_right_loc),
                            });
                        }
                        Some((err_l_loc, CapturedErrorKind::Stranded)) => {
                            set_error(Error::StrandedArguments {
                                stranded_span: (err_l_loc, self.previous_right_loc),
                            });
                        }
                        None => {}
                    }

                    if let Some((cmd, cmd_pos)) = self.command.replace((ident.value, (l, r))) {
                        return Some((
                            ast::Command {
                                name: cmd,
                                name_span: cmd_pos,
                                args: std::mem::take(&mut self.arguments),
                            },
                            error,
                        ));
                    }
                }
                token::Token::Literal(literal) => match self.capturing_error {
                    None if self.command.is_none() => {
                        self.capturing_error = Some((l, CapturedErrorKind::Stranded));
                    }
                    Some(_) => {}
                    None => {
                        let push_res = self.arguments.try_push(ast::Argument {
                            literal,
                            span: (l, r),
                        });
                        if push_res.is_err() {
                            self.capturing_error = Some((l, CapturedErrorKind::Overflow));
                        }
                    }
                },
            }

            self.previous_right_loc = r;
        }

        self.command.take().map(|(cmd, cmd_pos)| {
            (
                ast::Command {
                    name: cmd,
                    name_span: cmd_pos,
                    args: std::mem::take(&mut self.arguments),
                },
                error,
            )
        })
    }
}
