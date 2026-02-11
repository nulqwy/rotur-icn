use rotur_icn_lexer::token;
use rotur_icn_lowerer::hir;
use rotur_icn_parser::ast;

pub fn convert(hir: &hir::IconHir) -> ast::Icon<'static> {
    ast::Icon {
        commands: hir
            .operations
            .iter()
            .map(|op| match &op.kind {
                hir::OperationKind::SetWidth(set_width) => {
                    make_command(op, hir::SetWidth::NAME, [n(set_width.value)].into_iter())
                }
                hir::OperationKind::SetColour(set_colour) => make_command(
                    op,
                    hir::SetColour::NAME,
                    [token::Literal::Colour(set_colour.value)].into_iter(),
                ),
                hir::OperationKind::DrawLine(draw_line) => make_command(
                    op,
                    hir::DrawLine::NAME,
                    [
                        n(draw_line.start.x),
                        n(draw_line.start.y),
                        n(draw_line.end.x),
                        n(draw_line.end.y),
                    ]
                    .into_iter(),
                ),
                hir::OperationKind::ContinueLine(continue_line) => make_command(
                    op,
                    hir::ContinueLine::NAME,
                    [n(continue_line.next.x), n(continue_line.next.y)].into_iter(),
                ),
                hir::OperationKind::DrawDisk(draw_disk) => make_command(
                    op,
                    hir::DrawDisk::NAME,
                    [n(draw_disk.centre.x), n(draw_disk.centre.y)].into_iter(),
                ),
                hir::OperationKind::DrawRectangle(draw_rectangle) => make_command(
                    op,
                    if draw_rectangle.filled {
                        hir::DrawRectangle::NAME_FILLED
                    } else {
                        hir::DrawRectangle::NAME_HOLLOW
                    },
                    [
                        n(draw_rectangle.centre.x),
                        n(draw_rectangle.centre.y),
                        n(draw_rectangle.sizes.x),
                        n(draw_rectangle.sizes.y),
                    ]
                    .into_iter(),
                ),
                hir::OperationKind::DrawTriangle(draw_triangle) => make_command(
                    op,
                    hir::DrawTriangle::NAME,
                    [
                        n(draw_triangle.a.x),
                        n(draw_triangle.a.y),
                        n(draw_triangle.b.x),
                        n(draw_triangle.b.y),
                        n(draw_triangle.c.x),
                        n(draw_triangle.c.y),
                    ]
                    .into_iter(),
                ),
                hir::OperationKind::MoveCentre(move_centre) => make_command(
                    op,
                    hir::MoveCentre::NAME,
                    [n(move_centre.change.x), n(move_centre.change.y)].into_iter(),
                ),
                hir::OperationKind::ResetCentre(_reset_centre) => {
                    make_command(op, hir::ResetCentre::NAME, [].into_iter())
                }
                hir::OperationKind::DrawArc(draw_arc) => make_command(
                    op,
                    hir::DrawArc::NAME,
                    [
                        n(draw_arc.centre.x),
                        n(draw_arc.centre.y),
                        n(draw_arc.radius),
                        n(draw_arc.direction),
                        n(draw_arc.arm_angle),
                    ]
                    .into_iter(),
                ),
                hir::OperationKind::DrawEllipse(draw_ellipse) => make_command(
                    op,
                    hir::DrawEllipse::NAME,
                    [
                        n(draw_ellipse.centre.x),
                        n(draw_ellipse.centre.y),
                        n(draw_ellipse.major),
                        n(draw_ellipse.ratio),
                        n(draw_ellipse.direction),
                    ]
                    .into_iter(),
                ),
                hir::OperationKind::DrawCurve(draw_curve) => make_command(
                    op,
                    hir::DrawCurve::NAME,
                    [
                        n(draw_curve.start.x),
                        n(draw_curve.start.y),
                        n(draw_curve.end.x),
                        n(draw_curve.end.y),
                        n(draw_curve.control.x),
                        n(draw_curve.control.y),
                    ]
                    .into_iter(),
                ),
            })
            .collect(),
    }
}

fn make_command<'s>(
    op: &hir::Operation,
    name: &'s str,
    args: impl Iterator<Item = token::Literal>,
) -> ast::Command<'s> {
    ast::Command {
        name,
        name_pos: op.cmd_pos,
        args: args
            .map(|lit| ast::Argument {
                lit,
                pos: op.cmd_pos,
            })
            .collect(),
    }
}

fn n(v: f32) -> token::Literal {
    token::Literal::Number(v)
}
