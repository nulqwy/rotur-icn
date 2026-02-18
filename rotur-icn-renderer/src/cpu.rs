use rotur_icn_resolver::lir;
use rotur_icn_units::{Colour, Number, Vector};

use crate::cpu::shape::ComputedShapesBundle;

mod arc;
mod circle;
mod colour;
mod curve;
mod disk;
mod ellipse;
mod line;
mod maths;
mod rectangle;
mod shape;
mod triangle;

pub struct Renderer {
    icon: Option<ComputedShapesBundle>,
    pub background_colour: Colour,
    pub canvas: Vector,
    pub scaling: Number,
    pub camera_pos: Vector,
}

impl Default for Renderer {
    fn default() -> Self {
        Self {
            icon: None,
            background_colour: Colour::ZERO,
            canvas: Vector { x: 20., y: 20. },
            scaling: 1.,
            camera_pos: Vector::ZERO,
        }
    }
}

impl Renderer {
    pub fn new(
        canvas: Vector,
        scaling: Number,
        camera_pos: Vector,
        background_colour: Colour,
    ) -> Self {
        Self {
            canvas,
            scaling,
            camera_pos,
            background_colour,
            icon: None,
        }
    }

    pub fn load(&mut self, icon: &lir::IconLir) {
        self.icon = Some(ComputedShapesBundle::new(icon));
    }

    pub fn new_buf(&self) -> (Vec<u8>, (usize, usize)) {
        (
            vec![0; self.scaled_buf_size_linear()],
            self.scaled_buf_size(),
        )
    }

    #[expect(clippy::cast_possible_truncation)]
    #[expect(clippy::cast_sign_loss)]
    pub fn scaled_buf_size(&self) -> (usize, usize) {
        let scaled = self.canvas * self.scaling;
        // FIXME forbid too large buf sizes
        (
            scaled.x.abs().round() as usize,
            scaled.y.abs().round() as usize,
        )
    }

    pub fn scaled_buf_size_linear(&self) -> usize {
        let scaled = self.scaled_buf_size();
        scaled.0 * scaled.1 * 4
    }

    /// Render the loaded ICN into an RGBA buffer
    ///
    /// # Panics
    ///
    /// - If buffer length is not of correct size (4 bytes per pixel)
    ///
    /// - If no ICN is loaded
    #[expect(clippy::cast_precision_loss)]
    pub fn render(&mut self, buf: &mut [u8]) {
        assert_eq!(
            buf.len(),
            self.scaled_buf_size_linear(),
            "buffer must be of correct size"
        );

        let icon = self
            .icon
            .as_ref()
            .expect("icon should have been loaded by this point");
        let bg_colour = self.background_colour.into();

        let scaled_buf_size = self.scaled_buf_size();

        // FIXME forbid too large buf sizes
        let rel_offset = Vector {
            x: -(scaled_buf_size.0 as Number),
            y: scaled_buf_size.1 as Number,
        } / 2.;

        let inv_scaling = 1. / self.scaling;

        let coords = (0..scaled_buf_size.1)
            .flat_map(|y| (0..scaled_buf_size.0).map(move |x| (x, y)))
            .map(|(x, y)| Vector {
                x: x as _,
                y: y as _,
            });

        buf.as_chunks_mut::<4>()
            .0
            .iter_mut()
            .zip(coords)
            .for_each(|(pixel, coord)| {
                // XXX interestingly, if i distribute the mul and precompute the constant part,
                // the performance worsens (by noise levels, but consisently)
                let rel_pos = (coord.conj() + rel_offset) * inv_scaling + self.camera_pos;

                let new_col = icon
                    .shapes
                    .iter()
                    .rev()
                    .find_map(|sp| sp.test_with_colour(rel_pos))
                    .unwrap_or(bg_colour);

                let new_pixel = new_col.to_bytes();

                pixel.copy_from_slice(&new_pixel);
            });
    }
}
