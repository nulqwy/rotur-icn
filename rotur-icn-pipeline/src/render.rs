use rotur_icn_renderer::{cpu::Renderer, fitter};
use rotur_icn_resolver::lir;
use rotur_icn_units::{Colour, Vector};

pub fn render(
    icon: &lir::IconLir,
    canvas: Vector,
    scale: f32,
    zoom: f32,
    camera: Vector,
    background: Colour,
) -> (Vec<u8>, (usize, usize)) {
    let mut renderer = Renderer::new(canvas / zoom, scale * zoom, camera, background);
    renderer.load(icon);

    let (mut buf, buf_size) = renderer.new_buf();

    renderer.render(&mut buf);

    (buf, buf_size)
}

/// Choose canvas and camera settings based on other settings.
///
/// ## Canvas
///
/// If fit is set, but no explicit is given, then fit is chosen.
/// Same if only explicit is given.
/// If both are set, then largest of two per-axis is selected.
/// If no present, then default 20×20 is given.
///
/// ## Camera
///
/// Either default (0; 0) or fitted is chosen first.
/// Then it's offset by-axis.
pub fn choose_canvas_camera(
    icon: &lir::IconLir,
    fit: bool,
    pad: f32,
    canvas: Option<Vector>,
    camera: Vector,
) -> (Vector, Vector) {
    let (canvas_f, camera_f) = fit
        .then(|| fitter::fit(icon))
        .map_or((None, None), |fc| (Some(fc.size), Some(fc.camera)));

    let final_canvas = match (canvas_f, canvas) {
        (Some(fit), Some(set)) => fit.max(set),
        (Some(fit), None) => fit,
        (None, Some(set)) => set,
        (None, None) => Vector::new(20.),
    } + pad * 2.;

    let final_camera = camera_f.unwrap_or(Vector::ZERO) + camera;

    (final_canvas, final_camera)
}
