use rotur_icn_compiler::resolver::lir;
use rotur_icn_units::{Colour, Number, Vector};

mod arc;
mod circle;
mod curve;
mod disk;
mod ellipse;
mod line;
mod maths;
mod rectangle;
mod triangle;

pub struct Renderer<'i> {
    icon: Option<&'i lir::IconLir>,
    pub background_colour: Colour,
    pub canvas: Vector,
    pub scaling: Number,
    pub camera_pos: Vector,
}

impl Default for Renderer<'_> {
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

impl<'i> Renderer<'i> {
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

    pub fn load(&mut self, icon: &'i lir::IconLir) {
        self.icon = Some(icon);
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
                    .elements
                    .iter()
                    .rev()
                    .find_map(|el| Self::render_element(el, rel_pos))
                    .unwrap_or(bg_colour);

                let new_pixel = new_col.to_bytes();

                pixel.copy_from_slice(&new_pixel);
            }
        }
    }

    fn render_element(el: &lir::Element, pos: Vector) -> Option<InternalColour> {
        let intersects = match &el.kind {
            lir::ElementKind::Line(line) => line::test(line, pos),
            lir::ElementKind::Disk(disk) => disk::test(disk, pos),
            lir::ElementKind::Circle(circle) => circle::test(circle, pos),
            lir::ElementKind::Rectangle(rectangle) => rectangle::test(rectangle, pos),
            lir::ElementKind::Triangle(triangle) => triangle::test(triangle, pos),
            lir::ElementKind::Arc(arc) => arc::test(arc, pos),
            lir::ElementKind::Ellipse(ellipse) => ellipse::test(ellipse, pos),
            lir::ElementKind::Curve(curve) => curve::test(curve, pos),
        };

        intersects.then_some(el.colour.into())
    }
}

#[derive(Debug, Clone, Copy)]
struct InternalColour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl InternalColour {
    pub fn to_bytes(self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl From<Colour> for InternalColour {
    fn from(Colour { r, g, b, a }: Colour) -> Self {
        Self { r, g, b, a }
    }
}
