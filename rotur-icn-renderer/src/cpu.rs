use rotur_icn_compiler::resolver::lir;
use rotur_icn_units::{Colour, Number, Vector};

mod disk;

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
            background_colour: Colour::BLACK,
        }
    }
}

impl<'b, 'c> Renderer<'b, 'c> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_buf(buf_size: (usize, usize)) -> Vec<u8> {
        vec![0; buf_size.0 * buf_size.1 * 4]
    }

    pub fn set_buf(&mut self, buf: &'b mut [u8], buf_size: (usize, usize)) {
        assert_eq!(
            buf.len(),
            buf_size.0 * buf_size.1 * 4,
            "buffer length must be for u32-sized pixels"
        );

        self.buf = Some(buf);
        self.buf_size = buf_size;
    }

    pub fn render(&mut self) {
        let buf = self
            .buf
            .as_mut()
            .expect("buffer should have been set by this point");
        let icon = self.icon.expect("icon should have been set by this point");
        let bg_colour = self.background_colour.into();

        let rel_x_offset = (self.buf_size.0 / 2) as Number;
        let rel_y_offset = (self.buf_size.1 / 2) as Number;

        for y in 0..self.buf_size.1 {
            for x in 0..self.buf_size.0 {
                let pixel_i = (y * self.buf_size.0 + x) * 4;
                let pixel = &mut buf[pixel_i..pixel_i + 4];

                let rel_x = (x as Number) - rel_x_offset;
                let rel_y = -(y as Number) + rel_y_offset;

                let new_col = icon
                    .elements
                    .iter()
                    .rev()
                    .find_map(|el| Self::render_element(el, Vector { x: rel_x, y: rel_y }))
                    .unwrap_or(bg_colour);

                let new_pixel = new_col.to_bytes();

                pixel.copy_from_slice(&new_pixel);
            }
        }
    }

    fn render_element(el: &lir::Element, pos: Vector) -> Option<InternalColour> {
        let intersects = match &el.kind {
            lir::ElementKind::Line(line) => todo!(),
            lir::ElementKind::Disk(disk) => disk::test(disk, pos),
            lir::ElementKind::Circle(circle) => todo!(),
            lir::ElementKind::Rectangle(rectangle) => todo!(),
            lir::ElementKind::Triangle(triangle) => todo!(),
            lir::ElementKind::Arc(arc) => todo!(),
            lir::ElementKind::Ellipse(ellipse) => todo!(),
            lir::ElementKind::Curve(curve) => todo!(),
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
