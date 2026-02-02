use arrayvec::ArrayVec;

use crate::lexer::token;

#[derive(Debug, Clone)]
pub struct Icon<'s> {
    pub commands: Vec<Command<'s>>,
}

#[derive(Debug, Clone)]
pub struct Command<'s> {
    pub name: &'s str,
    pub name_pos: token::Pos,
    pub args: ArrayVec<Argument, 6>,
}

#[derive(Debug, Clone)]
pub struct Argument {
    pub lit: token::Literal,
    pub pos: token::Pos,
}
