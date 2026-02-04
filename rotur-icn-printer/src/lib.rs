use rotur_icn_compiler::{lowerer::hir, resolver::lir};

mod converter;
mod stringifier;
mod transformer;

pub use converter::convert;
pub use stringifier::stringify;
pub use transformer::transform;

pub fn print_lir(icon: &lir::IconLir, oneline: bool) -> String {
    print_hir(&transform(icon), oneline)
}

pub fn print_hir(icon: &hir::IconHir, oneline: bool) -> String {
    stringify(&convert(icon), oneline)
}
