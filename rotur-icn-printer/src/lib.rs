use rotur_icn_compiler::resolver::lir;

mod converter;
mod stringifier;
mod transformer;

pub use converter::convert;
pub use stringifier::stringify;
pub use transformer::transform;

pub fn print(icon: &lir::IconLir, oneline: bool) -> String {
    stringify(&convert(&transform(icon)), oneline)
}
