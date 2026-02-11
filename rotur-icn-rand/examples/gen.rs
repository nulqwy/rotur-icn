use rand::distr::Distribution as _;

use rotur_icn_lowerer::hir;
use rotur_icn_printer::print_hir;
use rotur_icn_rand::IcnSampler;

fn main() {
    let sampler = IcnSampler::default();

    let mut rng = rand::rng();
    eprint!("genning...");
    let icon: hir::IconHir = sampler.sample(&mut rng);
    eprintln!("done");

    println!("{}", print_hir(&icon, false));
}
