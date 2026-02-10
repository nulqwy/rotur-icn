use rotur_icn_compiler::lowerer::hir;

// TODO represent as a bitwise flag store
#[expect(
    clippy::struct_excessive_bools,
    reason = "a flag store, not a state machine"
)]
#[derive(Debug, Clone, Copy)]
pub struct Operations {
    pub set_width: bool,
    pub set_colour: bool,
    pub draw_line: bool,
    pub continue_line: bool,
    pub draw_disk: bool,
    pub draw_rectangle: bool,
    pub draw_triangle: bool,
    pub move_centre: bool,
    pub reset_centre: bool,
    pub draw_arc: bool,
    pub draw_ellipse: bool,
    pub draw_curve: bool,
}

impl Default for Operations {
    fn default() -> Self {
        Self {
            set_width: true,
            set_colour: true,
            draw_line: true,
            continue_line: false,
            draw_disk: true,
            draw_rectangle: true,
            draw_triangle: true,
            move_centre: false,
            reset_centre: false,
            draw_arc: true,
            draw_ellipse: true,
            draw_curve: true,
        }
    }
}

impl Operations {
    pub const FULL: Self = Self {
        set_width: true,
        set_colour: true,
        draw_line: true,
        continue_line: true,
        draw_disk: true,
        draw_rectangle: true,
        draw_triangle: true,
        move_centre: true,
        reset_centre: true,
        draw_arc: true,
        draw_ellipse: true,
        draw_curve: true,
    };

    pub fn count_enabled(self) -> usize {
        usize::from(self.set_width)
            + usize::from(self.set_colour)
            + usize::from(self.draw_line)
            + usize::from(self.continue_line)
            + usize::from(self.draw_disk)
            + usize::from(self.draw_rectangle)
            + usize::from(self.draw_triangle)
            + usize::from(self.move_centre)
            + usize::from(self.reset_centre)
            + usize::from(self.draw_arc)
            + usize::from(self.draw_ellipse)
            + usize::from(self.draw_curve)
    }

    pub fn as_bools(self) -> [bool; 12] {
        [
            self.set_width,
            self.set_colour,
            self.draw_line,
            self.continue_line,
            self.draw_disk,
            self.draw_rectangle,
            self.draw_triangle,
            self.move_centre,
            self.reset_centre,
            self.draw_arc,
            self.draw_ellipse,
            self.draw_curve,
        ]
    }
}

impl IntoIterator for Operations {
    type Item = hir::OperationKindTag;
    type IntoIter = OperationsIterator;

    fn into_iter(self) -> Self::IntoIter {
        OperationsIterator::new(self)
    }
}

pub struct OperationsIterator {
    ops: Operations,
    total: usize,
    passed: usize,
    next_i: usize,
}

impl OperationsIterator {
    pub fn new(ops: Operations) -> Self {
        Self {
            ops,
            total: ops.count_enabled(),
            passed: 0,
            next_i: 0,
        }
    }

    fn inc_next(&mut self) -> usize {
        let cur = self.next_i;
        self.next_i += 1;
        cur
    }
}

impl Iterator for OperationsIterator {
    type Item = hir::OperationKindTag;

    fn next(&mut self) -> Option<Self::Item> {
        let mut i = self.inc_next();

        while self.passed != self.total
            && let Some(is_set) = self.ops.as_bools().get(i).copied()
        {
            if is_set {
                self.passed += 1;

                return Some(match i {
                    0 => hir::OperationKindTag::SetWidth,
                    1 => hir::OperationKindTag::SetColour,
                    2 => hir::OperationKindTag::DrawLine,
                    3 => hir::OperationKindTag::ContinueLine,
                    4 => hir::OperationKindTag::DrawDisk,
                    5 => hir::OperationKindTag::DrawRectangle,
                    6 => hir::OperationKindTag::DrawTriangle,
                    7 => hir::OperationKindTag::MoveCentre,
                    8 => hir::OperationKindTag::ResetCentre,
                    9 => hir::OperationKindTag::DrawArc,
                    10 => hir::OperationKindTag::DrawEllipse,
                    11 => hir::OperationKindTag::DrawCurve,
                    _ => unreachable!("there are in total 12 op kinds"),
                });
            }

            i = self.inc_next();
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let left = self.total - self.passed;
        (left, Some(left))
    }
}

impl ExactSizeIterator for OperationsIterator {}
