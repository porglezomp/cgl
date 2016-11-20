extern crate cgl;

use cgl::{Color, Mat4, Renderer, Shader, Vec3, Vec4, Vert};

mod demo;

fn main() {
    let model = demo::african_head();
    let mut renderer = Renderer::with_dimensions(512, 512);
    let matrix = demo::african_head_matrix();
    let shader = DiffuseShader;

    renderer.model(&shader, &matrix, &model);

    demo::save(renderer.image(), 7);
}

struct DiffuseShader;

impl Shader<Vert, Mat4<f32>> for DiffuseShader {
    type VOut = Vert;

    fn vertex(&self, vert: Vert, mat: &Mat4<f32>, pos: &mut Vec4<f32>) -> Vert {
        *pos = *mat * vert.pos.augment();
        vert
    }

    fn fragment(&self, input: Vert, _uniform: &Mat4<f32>) -> Color {
        let c = input.norm.normalized()
            .dot(Vec3(0.0f32, 1.0, 0.5).normalized());
        Color::float_rgb(c * 1.2, c, c * 0.8)
    }
}
