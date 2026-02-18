use arrayvec::ArrayVec;

use rotur_icn_lexer::token;

#[derive(Debug, Clone)]
pub struct Icon<'s> {
    pub commands: Vec<Command<'s>>,
}

#[derive(Debug, Clone)]
pub struct Command<'s> {
    pub name: &'s str,
    pub name_span: token::Span,
    pub args: ArrayVec<Argument, 6>,
}

#[derive(Debug, Clone)]
pub struct Argument {
    pub literal: token::Literal,
    pub span: token::Span,
}
