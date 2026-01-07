use arrayvec::ArrayVec;

use crate::lexer;

#[derive(Debug, Clone)]
pub struct Icon<'s> {
    pub commands: Vec<Command<'s>>,
}

#[derive(Debug, Clone)]
pub struct Command<'s> {
    pub name: &'s str,
    pub name_pos: lexer::Pos,
    pub args: ArrayVec<Argument, 6>,
}

#[derive(Debug, Clone)]
pub struct Argument {
    pub lit: lexer::Literal,
    pub pos: lexer::Pos,
}
