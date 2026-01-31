// https://www.shadertoy.com/view/WltSD7
// APPROX 4
pub fn cos_acos_3(x: f32) -> f32 {
    let x = (x * 0.5 + 0.5).sqrt();
    x * (x * (x * (x * -0.008972 + 0.039071) - 0.107074) + 0.576975) + 0.5
}
