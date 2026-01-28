use std::fmt;

use super::lir;

impl fmt::Display for lir::IconLir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Icon elements ({} total):", self.elements.len())?;

        if self.elements.is_empty() {
            writeln!(f, "<no elements>")?;
        } else {
            for el in &self.elements {
                writeln!(f, "- {el}")?;
            }
        }

        Ok(())
    }
}

impl fmt::Display for lir::Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} col{}", self.kind, self.colour)
    }
}

impl fmt::Display for lir::ElementKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            lir::ElementKind::Line(line) => write!(f, "{line}"),
            lir::ElementKind::Disk(disk) => write!(f, "{disk}"),
            lir::ElementKind::Circle(circle) => write!(f, "{circle}"),
            lir::ElementKind::Rectangle(rectangle) => write!(f, "{rectangle}"),
            lir::ElementKind::Triangle(triangle) => write!(f, "{triangle}"),
            lir::ElementKind::Arc(arc) => write!(f, "{arc}"),
            lir::ElementKind::Ellipse(ellipse) => write!(f, "{ellipse}"),
            lir::ElementKind::Curve(curve) => write!(f, "{curve}"),
        }
    }
}

impl fmt::Display for lir::Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {} - {} w{}", self.start, self.end, self.width)
    }
}

impl fmt::Display for lir::Disk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "disk {} r{}", self.centre, self.radius)
    }
}

impl fmt::Display for lir::Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "circle {} r{} w{}", self.centre, self.radius, self.width)
    }
}

impl fmt::Display for lir::Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "rectangle {} s{} {} ow{}",
            self.bottom_left,
            self.sizes,
            if self.filled { "filled" } else { "hollow" },
            self.outline_width,
        )
    }
}

impl fmt::Display for lir::Triangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "triangle {} - {} - {} ow{}",
            self.a, self.b, self.c, self.outline_width,
        )
    }
}

impl fmt::Display for lir::Arc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "arc {} r{} w{} {:.2}rad - {:.2}rad",
            self.centre, self.radius, self.width, self.start_angle, self.end_angle,
        )
    }
}

impl fmt::Display for lir::Ellipse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ellipse {} s{} dir {:.2}rad ow{}",
            self.centre, self.sizes, self.direction, self.outline_width,
        )
    }
}

impl fmt::Display for lir::Curve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "curve {} - {} - {} w{}",
            self.start, self.control, self.end, self.width,
        )
    }
}
