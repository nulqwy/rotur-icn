use rotur_icn_lexer as lexer;
use rotur_icn_lowerer::{self as lowerer, hir};
use rotur_icn_parser::{self as parser, ast};
use rotur_icn_resolver::{self as resolver, lir};

pub fn process(src: &str) -> (ast::Icon<'_>, hir::IconHir, lir::IconLir, Errors) {
    let mut lexing_errors = Vec::new();
    let tokens = lexer::lex(src).filter_map(|(l, (t, e), r)| {
        if let Some(e) = e {
            lexing_errors.push(e);
        }

        t.map(|t| (l, t, r))
    });

    let mut parsing_errors = Vec::new();
    let mut commands_coll = Vec::new();
    let commands = parser::Parser::new(tokens)
        .map(|(c, e)| {
            if let Some(e) = e {
                parsing_errors.push(e);
            }

            c
        })
        .inspect(|c| commands_coll.push(c.clone()));

    let mut lowering_errors = Vec::new();
    let mut operations_coll = Vec::new();
    let operations = lowerer::lower(commands)
        .filter_map(|(o, mut e)| {
            lowering_errors.append(&mut e);

            o
        })
        .inspect(|o| operations_coll.push(o.clone()));

    let mut resolving_errors = Vec::new();
    let elements = resolver::resolve(operations)
        .filter_map(|(el, e)| {
            if let Some(e) = e {
                resolving_errors.push(e);
            }

            el
        })
        .collect();

    (
        ast::Icon {
            commands: commands_coll,
        },
        hir::IconHir {
            operations: operations_coll,
        },
        lir::IconLir { elements },
        Errors {
            lexing: lexing_errors,
            parsing: parsing_errors,
            lowering: lowering_errors,
            resolving: resolving_errors,
        },
    )
}

pub fn process_final(src: &str) -> (lir::IconLir, Errors) {
    let mut lexing_errors = Vec::new();
    let tokens = lexer::lex(src).filter_map(|(l, (t, e), r)| {
        if let Some(e) = e {
            lexing_errors.push(e);
        }

        t.map(|t| (l, t, r))
    });

    let mut parsing_errors = Vec::new();
    let commands = parser::Parser::new(tokens).map(|(c, e)| {
        if let Some(e) = e {
            parsing_errors.push(e);
        }

        c
    });

    let mut lowering_errors = Vec::new();
    let operations = lowerer::lower(commands).filter_map(|(o, mut e)| {
        lowering_errors.append(&mut e);

        o
    });

    let mut resolving_errors = Vec::new();
    let elements = resolver::resolve(operations)
        .filter_map(|(el, e)| {
            if let Some(e) = e {
                resolving_errors.push(e);
            }

            el
        })
        .collect();

    (
        lir::IconLir { elements },
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
