use rand::{Rng, distr::Distribution, seq::IndexedRandom};

use rotur_icn_lexer::token;
use rotur_icn_lowerer::hir;

use super::IcnSampler;

impl Distribution<hir::IconHir> for IcnSampler {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> hir::IconHir {
        let count = rng.random_range(self.count_range.clone());

        hir::IconHir {
            operations: self
                .map(|kind| hir::Operation {
                    cmd_pos: (token::Loc::default(), token::Loc::default()),
                    kind,
                })
                .sample_iter(rng)
                .take(count)
                .collect(),
        }
    }
}

impl Distribution<hir::OperationKind> for IcnSampler {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> hir::OperationKind {
        match self.sample(rng) {
            hir::OperationKindTag::SetWidth => hir::OperationKind::SetWidth(self.sample(rng)),
            hir::OperationKindTag::SetColour => hir::OperationKind::SetColour(self.sample(rng)),
            hir::OperationKindTag::DrawLine => hir::OperationKind::DrawLine(self.sample(rng)),
            hir::OperationKindTag::ContinueLine => {
                hir::OperationKind::ContinueLine(self.sample(rng))
            }
            hir::OperationKindTag::DrawDisk => hir::OperationKind::DrawDisk(self.sample(rng)),
            hir::OperationKindTag::DrawRectangle => {
                hir::OperationKind::DrawRectangle(self.sample(rng))
            }
            hir::OperationKindTag::DrawTriangle => {
                hir::OperationKind::DrawTriangle(self.sample(rng))
            }
            hir::OperationKindTag::MoveCentre => hir::OperationKind::MoveCentre(self.sample(rng)),
            hir::OperationKindTag::ResetCentre => hir::OperationKind::ResetCentre(self.sample(rng)),
            hir::OperationKindTag::DrawArc => hir::OperationKind::DrawArc(self.sample(rng)),
            hir::OperationKindTag::DrawEllipse => hir::OperationKind::DrawEllipse(self.sample(rng)),
            hir::OperationKindTag::DrawCurve => hir::OperationKind::DrawCurve(self.sample(rng)),
        }
    }
}

impl Distribution<hir::OperationKindTag> for IcnSampler {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> hir::OperationKindTag {
        *self
            .operations_enabled_vec
            .choose(rng)
            .expect("there should be at least one op enabled")
    }
}
