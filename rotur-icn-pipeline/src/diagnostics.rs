use std::ops::Range;

use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::Errors;

impl Errors {
    pub fn into_diagnostics(&self) -> impl Iterator<Item = Diagnostic<()>> {
        self.resolving
            .iter()
            .map(|err| ResolverErrorDiagnostic(err).into())
            .chain(
                self.lowering
                    .iter()
                    .map(|err| LowererErrorDiagnostic(err).into()),
            )
            .chain(
                self.parsing
                    .iter()
                    .map(|err| ParserErrorDiagnostic(err).into()),
            )
            .chain(
                self.lexing
                    .iter()
                    .map(|err| LexerErrorDiagnostic(err).into()),
            )
    }
}

struct LexerErrorDiagnostic<'err>(pub &'err rotur_icn_syntax::lexer::Error);

impl From<LexerErrorDiagnostic<'_>> for Diagnostic<()> {
    fn from(LexerErrorDiagnostic(error): LexerErrorDiagnostic) -> Self {
        Self::error()
            .with_code(error.kind.code())
            .with_message(&error.kind)
            .with_label(
                Label::primary((), LexerPosRange(&error.pos))
                    .with_message(format!("help: {}", error.kind.help())),
            )
    }
}

struct ParserErrorDiagnostic<'err>(pub &'err rotur_icn_syntax::parser::Error);

impl From<ParserErrorDiagnostic<'_>> for Diagnostic<()> {
    fn from(ParserErrorDiagnostic(error): ParserErrorDiagnostic) -> Self {
        match error {
            error @ rotur_icn_syntax::parser::Error::TooManyArguments {
                keyword_pos,
                overflow_pos,
            } => Self::error()
                .with_code(error.code())
                .with_message(error.message())
                .with_labels_iter([
                    Label::primary((), LexerPosRange(overflow_pos))
                        .with_message(format!("help: {}", error.help())),
                    Label::secondary((), LexerPosRange(keyword_pos))
                        .with_message("while parsing this command"),
                ]),
            error @ rotur_icn_syntax::parser::Error::StrandedArguments { stranded_pos } => {
                Self::error()
                    .with_code(error.code())
                    .with_message(error.message())
                    .with_labels_iter([
                        Label::primary((), LexerPosRange(stranded_pos)).with_message(error.help())
                    ])
            }
        }
    }
}

struct LowererErrorDiagnostic<'err>(pub &'err rotur_icn_compiler::lowerer::Error);

impl From<LowererErrorDiagnostic<'_>> for Diagnostic<()> {
    fn from(LowererErrorDiagnostic(error): LowererErrorDiagnostic) -> Self {
        match &error.kind {
            error_kind @ rotur_icn_compiler::lowerer::ErrorKind::TooManyArguments {
                overflow_pos,
                exp: _,
                got: _,
            } => Self::error()
                .with_code(error_kind.code())
                .with_message(error_kind)
                .with_labels_iter([
                    Label::primary((), LexerPosRange(overflow_pos))
                        .with_message(error_kind.help().unwrap_or("")),
                    Label::secondary((), LexerPosRange(&error.cmd_pos))
                        .with_message("while lowering this command"),
                ]),
            error_kind @ rotur_icn_compiler::lowerer::ErrorKind::TooFewArguments {
                args_end_loc,
                exp: _,
                got: _,
            } => Self::error()
                .with_code(error_kind.code())
                .with_message(error_kind)
                .with_labels_iter([
                    Label::primary((), LexerLocRange(args_end_loc))
                        .with_message(error_kind.help().unwrap_or("")),
                    Label::secondary((), LexerPosRange(&error.cmd_pos))
                        .with_message("while lowering this command"),
                ]),
            error_kind @ rotur_icn_compiler::lowerer::ErrorKind::UnexpectedLiteralKind {
                arg_pos,
                arg_index: _,
                exp: _,
                got: _,
            } => Self::error()
                .with_code(error_kind.code())
                .with_message(error_kind)
                .with_labels_iter([
                    Label::primary((), LexerPosRange(arg_pos))
                        .with_message(error_kind.help().unwrap_or("")),
                    Label::secondary((), LexerPosRange(&error.cmd_pos))
                        .with_message("while lowering this command"),
                ]),
            error_kind @ rotur_icn_compiler::lowerer::ErrorKind::InvalidNumericColour {
                arg_pos,
                arg_index: _,
            } => Self::error()
                .with_code(error_kind.code())
                .with_message(error_kind)
                .with_labels_iter([
                    Label::primary((), LexerPosRange(arg_pos))
                        .with_message(error_kind.help().unwrap_or("")),
                    Label::secondary((), LexerPosRange(&error.cmd_pos))
                        .with_message("while lowering this command"),
                ]),
            error_kind @ rotur_icn_compiler::lowerer::ErrorKind::ArgOutOfRange {
                arg_pos,
                arg_index: _,
                range_start: _,
                range_end: _,
            } => Self::error()
                .with_code(error_kind.code())
                .with_message(error_kind)
                .with_labels_iter([
                    Label::primary((), LexerPosRange(arg_pos))
                        .with_message(error_kind.help().unwrap_or("")),
                    Label::secondary((), LexerPosRange(&error.cmd_pos))
                        .with_message("while lowering this command"),
                ]),
            error_kind @ rotur_icn_compiler::lowerer::ErrorKind::InvalidCommand => Self::error()
                .with_code(error_kind.code())
                .with_message(error_kind)
                .with_labels_iter([Label::primary((), LexerPosRange(&error.cmd_pos))
                    .with_message(error_kind.help().unwrap_or(""))]),
        }
    }
}

struct ResolverErrorDiagnostic<'err>(pub &'err rotur_icn_compiler::resolver::Error);

impl From<ResolverErrorDiagnostic<'_>> for Diagnostic<()> {
    fn from(ResolverErrorDiagnostic(error): ResolverErrorDiagnostic<'_>) -> Self {
        match &error.kind {
            error_kind @ rotur_icn_compiler::resolver::ErrorKind::DanglingContinuedLine => {
                Self::error()
                    .with_code(error_kind.code())
                    .with_message(error_kind)
                    .with_labels_iter([Label::primary((), LexerPosRange(&error.cmd_pos))
                        .with_message("while resolving this command")])
            }
        }
    }
}

struct LexerPosRange<'p>(&'p rotur_icn_syntax::lexer::Pos);

impl From<LexerPosRange<'_>> for Range<usize> {
    fn from(LexerPosRange((start, end)): LexerPosRange) -> Self {
        start.byte_idx..end.byte_idx
    }
}

struct LexerLocRange<'l>(&'l rotur_icn_syntax::lexer::Loc);

impl From<LexerLocRange<'_>> for Range<usize> {
    fn from(LexerLocRange(loc): LexerLocRange) -> Self {
        loc.byte_idx..loc.byte_idx
    }
}
