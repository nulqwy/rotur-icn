// FIXME rewrite to use std::fmt

use std::fmt::Write as _;

use rotur_icn_syntax::{lexer::token, parser::ast};

pub fn stringify(ast: &ast::Icon, oneline: bool) -> String {
    // TODO with capacity? maybe based on some estimates of bytes/command
    let mut buf = String::new();

    for cmd in &ast.commands {
        write!(buf, "{}", &cmd.name).unwrap();

        for arg in &cmd.args {
            match &arg.lit {
                token::Literal::Number(n) => write!(buf, " {n}").unwrap(),
                // TODO handle those which are representable as #fff
                // TODO handle alpha somehow, maybe panic?
                token::Literal::Colour(col) => {
                    write!(buf, " #{:0>2x}{:0>2x}{:0>2x}", col.r, col.g, col.b).unwrap()
                }
            }
        }

        if oneline {
            write!(buf, " ").unwrap();
        } else {
            writeln!(buf).unwrap();
        }
    }

    buf
}
