use std::fmt;

use super::ir::{
    Colour, ContinueLine, DrawArc, DrawCurve, DrawDot, DrawEllipse, DrawLine, DrawRectangle,
    DrawTriangle, IconIr, Instruction, MoveCentre, ResetCentre, SetColour, SetWidth, Vector,
};

impl fmt::Display for IconIr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Icon instructions:")?;

        for instr in &self.instructions {
            writeln!(f, "- {instr}")?;
        }

        Ok(())
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::SetWidth(set_width) => write!(f, "{set_width}"),
            Instruction::SetColour(set_colour) => write!(f, "{set_colour}"),
            Instruction::DrawLine(draw_line) => write!(f, "{draw_line}"),
            Instruction::ContinueLine(continue_line) => write!(f, "{continue_line}"),
            Instruction::DrawDot(draw_dot) => write!(f, "{draw_dot}"),
            Instruction::DrawRectangle(draw_rectangle) => write!(f, "{draw_rectangle}"),
            Instruction::DrawTriangle(draw_triangle) => write!(f, "{draw_triangle}"),
            Instruction::MoveCentre(move_centre) => write!(f, "{move_centre}"),
            Instruction::ResetCentre(reset_centre) => write!(f, "{reset_centre}"),
            Instruction::DrawArc(draw_arc) => write!(f, "{draw_arc}"),
            Instruction::DrawEllipse(draw_ellipse) => write!(f, "{draw_ellipse}"),
            Instruction::DrawCurve(draw_curve) => write!(f, "{draw_curve}"),
        }
    }
}

impl fmt::Display for SetWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "set width to {}", self.value)
    }
}

impl fmt::Display for SetColour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "set colour to {}", self.value)
    }
}

impl fmt::Display for DrawLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "draw a line from {} to {}", self.start, self.end)
    }
}

impl fmt::Display for ContinueLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "continue the line to {}", self.next)
    }
}

impl fmt::Display for DrawDot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "draw a dot at {}", self.pos)
    }
}

impl fmt::Display for DrawRectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "draw a {} rectangle at {} with sizes {}",
            if self.filled { "filled" } else { "hollow" },
            self.centre,
            self.sizes
        )
    }
}

impl fmt::Display for DrawTriangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "draw a triangle bound by {}, {} & {}",
            self.a, self.b, self.c
        )
    }
}

impl fmt::Display for MoveCentre {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "move centre by {}", self.change)
    }
}

impl fmt::Display for ResetCentre {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "reset centre to absolute origin")
    }
}

impl fmt::Display for DrawArc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "draw an arc at {} of radius {} from {} to {}",
            self.centre, self.radius, self.start_angle, self.end_angle
        )
    }
}

impl fmt::Display for DrawEllipse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "draw an ellipse at {} with sizes {} facing {}",
            self.centre, self.sizes, self.facing
        )
    }
}

impl fmt::Display for DrawCurve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "draw a curve passing {}, {} & {}",
            self.start, self.control, self.end
        )
    }
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:0>2x}{:0>2x}{:0>2x}", self.r, self.g, self.b)
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
