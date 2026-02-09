use rotur_icn_compiler::resolver::lir;
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

    pub fn scaled_buf_size(&self) -> (usize, usize) {
        let scaled = self.canvas * self.scaling;
        (scaled.x.round() as usize, scaled.y.round() as usize)
    }

    pub fn scaled_buf_size_linear(&self) -> usize {
        let scaled = self.scaled_buf_size();
        scaled.0 * scaled.1 * 4
    }

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

        let rel_x_offset = (scaled_buf_size.0 / 2) as Number;
        let rel_y_offset = (scaled_buf_size.1 / 2) as Number;

        for y in 0..scaled_buf_size.1 {
            for x in 0..scaled_buf_size.0 {
                let pixel_i = (y * scaled_buf_size.0 + x) * 4;
                let pixel = &mut buf[pixel_i..pixel_i + 4];

                let rel_x = (x as Number) - rel_x_offset;
                let rel_y = -(y as Number) + rel_y_offset;

                let rel_pos = Vector { x: rel_x, y: rel_y } / self.scaling + self.camera_pos;

                let new_col = icon
                    .shapes
                    .iter()
                    .rev()
                    .find_map(|sp| sp.test_with_colour(rel_pos))
                    .unwrap_or(bg_colour);

                let new_pixel = new_col.to_bytes();

                pixel.copy_from_slice(&new_pixel);
            }
        }
    }
}
