use rotur_icn_compiler::resolver::lir;
use rotur_icn_units::{Colour, Number, Vector};

mod arc;
mod circle;
mod disk;
mod line;
mod rectangle;
mod triangle;

pub struct Renderer<'b, 'c> {
    buf: Option<&'b mut [u8]>,
    buf_size: (usize, usize),
    pub icon: Option<&'c lir::IconLir>,
    pub background_colour: Colour,
    pub camera_pos: Vector,
    pub scaling: Number,
}

impl Default for Renderer<'_, '_> {
    fn default() -> Self {
        Self {
            buf: None,
            buf_size: (0, 0),
            camera_pos: Vector::ZERO,
            scaling: 1.,
            icon: None,
            background_colour: Colour::ZERO,
        }
    }
}

impl<'b, 'c> Renderer<'b, 'c> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn scaled_buf_size(&self, buf_size: (usize, usize)) -> (usize, usize) {
        (
            (buf_size.0 as Number * self.scaling).round() as usize,
            (buf_size.1 as Number * self.scaling).round() as usize,
        )
    }

    pub fn scaled_buf_size_linear(&self, buf_size: (usize, usize)) -> usize {
        let scaled = self.scaled_buf_size(buf_size);
        scaled.0 * scaled.1 * 4
    }

    pub fn new_buf(&self, buf_size: (usize, usize)) -> Vec<u8> {
        vec![0; self.scaled_buf_size_linear(buf_size)]
    }

    pub fn set_buf(&mut self, buf: &'b mut [u8], buf_size: (usize, usize)) {
        assert_eq!(
            buf.len(),
            self.scaled_buf_size_linear(buf_size),
            "buffer length must be for u32-sized pixels"
        );

        self.buf = Some(buf);
        self.buf_size = buf_size;
    }

    pub fn render(&mut self) {
        // FIXME cache this one
        let scaled_buf_size = self.scaled_buf_size(self.buf_size);
        let buf = self
            .buf
            .as_mut()
            .expect("buffer should have been set by this point");
        let icon = self.icon.expect("icon should have been set by this point");
        let bg_colour = self.background_colour.into();

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
            lir::ElementKind::Ellipse(_ellipse) => {
                // TODO
                eprintln!("WARNING IGNORING ELLIPSE - NOT IMPLEMENTED");
                false
            }
            lir::ElementKind::Curve(_curve) => {
                // TODO
                eprintln!("WARNING IGNORING CURVE - NOT IMPLEMENTED");
                false
            }
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
