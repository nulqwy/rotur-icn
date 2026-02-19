#[derive(Debug, Clone)]
pub struct Region {
    /// [start; end)
    pub x: (usize, usize),
    /// [start; end)
    pub y: (usize, usize),
}

impl Region {
    pub fn new_from_points(start: (usize, usize), end: (usize, usize)) -> Self {
        Self {
            x: (start.0, end.0),
            y: (start.1, end.1),
        }
    }

    pub fn new_from_zero(end: (usize, usize)) -> Self {
        Self::new_from_points((0, 0), end)
    }

    pub fn start(&self) -> (usize, usize) {
        (self.x.0, self.y.0)
    }

    pub fn end(&self) -> (usize, usize) {
        (self.x.1, self.y.1)
    }

    pub fn lengths(&self) -> (usize, usize) {
        (self.x.1 - self.x.0, self.y.1 - self.y.0)
    }

    pub fn split(&self, (side_x, side_y): (usize, usize)) -> impl Iterator<Item = Region> {
        Self::split_side(self.y, side_y)
            .flat_map(move |y_axis| {
                Self::split_side(self.x, side_x).map(move |x_axis| (x_axis, y_axis))
            })
            .map(|(x, y)| Self { x, y })
    }

    fn split_side(side: (usize, usize), length: usize) -> impl Iterator<Item = (usize, usize)> {
        (side.0..side.1)
            .step_by(length)
            .map(move |start| (start, side.1.min(start + length)))
    }
}
