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

pub struct Renderer {
    icon: Option<ComputedIcon>,
    pub background_colour: Colour,
    pub canvas: Vector,
    pub scaling: Number,
    pub camera_pos: Vector,
}

struct ComputedIcon {
    elements: Vec<ComputedElement>,
}

struct ComputedElement {
    colour: InternalColour,
    data: ComputedElementData,
}

enum ComputedElementData {
    Line(line::Line),
    Disk(disk::Disk),
    Circle(circle::Circle),
    Rectangle(rectangle::Rectangle),
    Triangle(triangle::Triangle),
    Arc(arc::Arc),
    Ellipse(ellipse::Ellipse),
    Curve(curve::Curve),
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
        self.icon = Some(ComputedIcon {
            elements: icon
                .elements
                .iter()
                .map(|el| ComputedElement {
                    colour: el.colour.into(),
                    data: match &el.kind {
                        lir::ElementKind::Line(line) => {
                            ComputedElementData::Line(line::Line::new(line))
                        }
                        lir::ElementKind::Disk(disk) => {
                            ComputedElementData::Disk(disk::Disk::new(disk))
                        }
                        lir::ElementKind::Circle(circle) => {
                            ComputedElementData::Circle(circle::Circle::new(circle))
                        }
                        lir::ElementKind::Rectangle(rectangle) => {
                            ComputedElementData::Rectangle(rectangle::Rectangle::new(rectangle))
                        }
                        lir::ElementKind::Triangle(triangle) => {
                            ComputedElementData::Triangle(triangle::Triangle::new(triangle))
                        }
                        lir::ElementKind::Arc(arc) => ComputedElementData::Arc(arc::Arc::new(arc)),
                        lir::ElementKind::Ellipse(ellipse) => {
                            ComputedElementData::Ellipse(ellipse::Ellipse::new(ellipse))
                        }
                        lir::ElementKind::Curve(curve) => {
                            ComputedElementData::Curve(curve::Curve::new(curve))
                        }
                    },
                })
                .collect(),
        });
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

    fn render_element(el: &ComputedElement, pos: Vector) -> Option<InternalColour> {
        let intersects = match &el.data {
            ComputedElementData::Line(line) => line.test(pos),
            ComputedElementData::Disk(disk) => disk.test(pos),
            ComputedElementData::Circle(circle) => circle.test(pos),
            ComputedElementData::Rectangle(rectangle) => rectangle.test(pos),
            ComputedElementData::Triangle(triangle) => triangle.test(pos),
            ComputedElementData::Arc(arc) => arc.test(pos),
            ComputedElementData::Ellipse(ellipse) => ellipse.test(pos),
            ComputedElementData::Curve(curve) => curve.test(pos),
        };

        intersects.then_some(el.colour)
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
