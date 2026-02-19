use rotur_icn_resolver::lir;
use rotur_icn_units::{Colour, Number, Vector};
use smallvec::SmallVec;

use crate::{cpu::shape::Shape, divider::Region};

use colour::InternalColour;
use shape::ComputedShapesBundle;

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
    const REGION_SIZE: usize = 64;
    const REGION_ELEMENTS_STACK_BUFFER_SIZE: usize = 64;

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
    pub fn render(&self, buf: &mut [u8]) {
        assert_eq!(
            buf.len(),
            self.scaled_buf_size_linear(),
            "buffer must be of correct size"
        );

        let buf_size = self.scaled_buf_size();
        let icon = self
            .icon
            .as_ref()
            .expect("icon should have been loaded by this point");
        #[expect(clippy::cast_precision_loss)]
        let rel_offset = Vector {
            x: -(buf_size.0 as Number),
            y: buf_size.1 as Number,
        } / 2.;

        Region::new_from_zero(buf_size)
            .split((Self::REGION_SIZE, Self::REGION_SIZE))
            .for_each(|region| {
                self.render_region(
                    (buf, buf_size),
                    icon,
                    rel_offset,
                    self.background_colour.into(),
                    1. / self.scaling,
                    &region,
                );
            });
    }

    fn render_region(
        &self,
        (buf, buf_size): (&mut [u8], (usize, usize)),
        icon: &ComputedShapesBundle,
        rel_offset: Vector,
        bg_colour: InternalColour,
        inv_scaling: f32,
        region: &Region,
    ) {
        // FIXME forbid too large canvas sizes
        #[expect(clippy::cast_precision_loss)]
        let to_vec = |coord: (usize, usize)| Vector {
            x: coord.0 as _,
            y: coord.1 as _,
        };
        // XXX interestingly, if i distribute the mul and precompute the constant part,
        // the performance worsens (by noise levels, but consisently)
        let transform_pos = |pos: Vector| (pos.conj() + rel_offset) * inv_scaling + self.camera_pos;

        let region_points = (
            transform_pos(to_vec(region.start())),
            transform_pos(to_vec(region.end())),
        );

        let region_bounds = (
            Vector {
                x: region_points.0.x,
                y: region_points.1.y,
            },
            Vector {
                x: region_points.1.x,
                y: region_points.0.y,
            },
        );

        let possible_els = icon
            .shapes
            .iter()
            .filter(|s| s.possibly_within(region_bounds))
            .collect::<SmallVec<[_; Self::REGION_ELEMENTS_STACK_BUFFER_SIZE]>>();

        (region.y.0..region.y.1)
            .flat_map(|y| (region.x.0..region.x.1).map(move |x| (x, y)))
            .map(|coord| {
                (
                    {
                        let pixel_i = (coord.0 + coord.1 * buf_size.0) * 4;
                        pixel_i..pixel_i + 4
                    },
                    transform_pos(to_vec(coord)),
                )
            })
            .for_each(|(pixel, pos)| {
                let new_col = possible_els
                    .iter()
                    .rev()
                    .find_map(|sp| sp.test_with_colour(pos))
                    .unwrap_or(bg_colour);

                let new_pixel = new_col.to_bytes();

                buf[pixel].copy_from_slice(&new_pixel);
            });
    }
}
