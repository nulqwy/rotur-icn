use std::ops::RangeInclusive;

use arrayvec::ArrayVec;

use rotur_icn_compiler::lowerer::hir::OperationKindTag;
use rotur_icn_units::Vector;

mod icon;
mod ops;
mod ops_store;
mod units;

pub use ops_store::{Operations, OperationsIterator};

#[derive(Debug, Clone)]
pub struct IcnSampler {
    pub space: (Vector, Vector),
    pub width_log_range: RangeInclusive<f32>,
    pub filled_radius_range: RangeInclusive<f32>,
    pub count_range: RangeInclusive<usize>,
    pub full_colour: bool,
    operations_enabled: Operations,
    operations_enabled_vec: ArrayVec<OperationKindTag, 12>,
}

impl Default for IcnSampler {
    fn default() -> Self {
        let operations_enabled = Operations::default();

        Self {
            space: (Vector::new(-10.), Vector::new(10.)),
            width_log_range: 0.1f32.ln()..=2.0f32.ln(),
            filled_radius_range: 0.0..=1.5,
            count_range: 80..=200,
            full_colour: false,
            operations_enabled,
            operations_enabled_vec: operations_enabled.into_iter().collect(),
        }
    }
}

impl IcnSampler {
    pub fn operations_enabled(&self) -> Operations {
        self.operations_enabled
    }

    pub fn set_operations_enabled(&mut self, ops: Operations) {
        self.operations_enabled = ops;
        self.operations_enabled_vec = ops.into_iter().collect();
    }

    pub fn space_x(&self) -> RangeInclusive<f32> {
        self.space.0.x..=self.space.1.x
    }

    pub fn space_y(&self) -> RangeInclusive<f32> {
        self.space.0.y..=self.space.1.y
    }
}
