use rotur_icn_compiler::{
    lowerer::{self, hir},
    resolver::{self, lir},
};
use rotur_icn_syntax::{
    lexer,
    parser::{self, ast},
};

#[cfg(feature = "diagnostics")]
mod diagnostics;

pub fn process(src: &'_ str) -> (ast::Icon<'_>, hir::IconHir, lir::IconLir, Errors) {
    let mut lexing_errors = Vec::new();

    let lexer = lexer::lex(&mut lexing_errors, src);
    let (icon_ast, parsing_errors) = parser::parse(lexer);
    let (icon_hir, lowering_errors) = lowerer::lower(&icon_ast);
    let (icon_lir, resolving_errors) = resolver::resolve(&icon_hir);

    (
        icon_ast,
        icon_hir,
        icon_lir,
        Errors {
            lexing: lexing_errors,
            parsing: parsing_errors,
            lowering: lowering_errors,
            resolving: resolving_errors,
        },
    )
}

pub struct Errors {
    pub lexing: Vec<lexer::Error>,
    pub parsing: Vec<parser::Error>,
    pub lowering: Vec<lowerer::Error>,
    pub resolving: Vec<resolver::Error>,
}

impl Errors {
    pub fn is_empty(&self) -> bool {
        self.lexing.is_empty()
            && self.parsing.is_empty()
            && self.lowering.is_empty()
            && self.resolving.is_empty()
    }

    pub fn len(&self) -> usize {
        self.lexing.len() + self.parsing.len() + self.lowering.len() + self.resolving.len()
    }
}
