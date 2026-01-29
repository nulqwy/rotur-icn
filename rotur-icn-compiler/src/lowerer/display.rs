use std::fmt;

use super::hir;

impl fmt::Display for hir::IconHir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Icon instructions ({} total):", self.operations.len())?;

        if self.operations.is_empty() {
            writeln!(f, "<no instructions>")?;
        } else {
            for instr in &self.operations {
                writeln!(f, "- {instr}")?;
            }
        }

        Ok(())
    }
}

impl fmt::Display for hir::Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.kind)
    }
}

impl fmt::Display for hir::OperationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            hir::OperationKind::SetWidth(set_width) => write!(f, "{set_width}"),
            hir::OperationKind::SetColour(set_colour) => write!(f, "{set_colour}"),
            hir::OperationKind::DrawLine(draw_line) => write!(f, "{draw_line}"),
            hir::OperationKind::ContinueLine(continue_line) => write!(f, "{continue_line}"),
            hir::OperationKind::DrawDisk(draw_circle) => write!(f, "{draw_circle}"),
            hir::OperationKind::DrawRectangle(draw_rectangle) => write!(f, "{draw_rectangle}"),
            hir::OperationKind::DrawTriangle(draw_triangle) => write!(f, "{draw_triangle}"),
            hir::OperationKind::MoveCentre(move_centre) => write!(f, "{move_centre}"),
            hir::OperationKind::ResetCentre(reset_centre) => write!(f, "{reset_centre}"),
            hir::OperationKind::DrawArc(draw_arc) => write!(f, "{draw_arc}"),
            hir::OperationKind::DrawEllipse(draw_ellipse) => write!(f, "{draw_ellipse}"),
            hir::OperationKind::DrawCurve(draw_curve) => write!(f, "{draw_curve}"),
        }
    }
}

impl fmt::Display for hir::SetWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "set width {}", self.value)
    }
}

impl fmt::Display for hir::SetColour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "set col {}", self.value)
    }
}

impl fmt::Display for hir::DrawLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "draw line {} - {}", self.start, self.end)
    }
}

impl fmt::Display for hir::ContinueLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "draw cont line >{}", self.next)
    }
}

impl fmt::Display for hir::DrawDisk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "draw dot {}", self.centre)
    }
}

impl fmt::Display for hir::DrawRectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "draw rectangle {} s{} {}",
            self.centre,
            self.sizes,
            if self.filled { "filled" } else { "hollow" },
        )
    }
}

impl fmt::Display for hir::DrawTriangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "draw triangle {} - {} - {}", self.a, self.b, self.c)
    }
}

impl fmt::Display for hir::MoveCentre {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "move centre {}", self.change)
    }
}

impl fmt::Display for hir::ResetCentre {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "reset centre")
    }
}

impl fmt::Display for hir::DrawArc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "draw arc {} r{} dir {}*10deg arms {}deg",
            self.centre, self.radius, self.direction, self.arm_angle,
        )
    }
}

impl fmt::Display for hir::DrawEllipse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "draw ellipse {} s{}/{} dir {}deg",
            self.centre, self.width, self.ratio, self.direction,
        )
    }
}

impl fmt::Display for hir::DrawCurve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "draw curve {} - {} - {}",
            self.start, self.control, self.end,
        )
    }
}
