extern crate cgl;

use cgl::{Color, Mat4, Obj, Renderer, Shader, Vec3, Vert, write_bmp};
use std::fs::File;

fn main() {
    let model = Obj::from_file("assets/african_head/african_head.obj")
        .expect("Should parse assets/african_head/african_head.obj")
        .model()
        .expect("Should convert correctly");

    let mut renderer = Renderer::with_dimensions(512, 512);

    let matrix = {
        let viewport = Mat4::viewport(512, 512);
        let perspective = Mat4::perspective(1.0);
        let model = Mat4::identity();
        let view = Mat4::lookat(Vec3(0.0, 0.0, 0.0),
                                Vec3(0.3, 0.2, 0.5),
                                Vec3(0.0, 1.0, 0.0));
        viewport * perspective * view * model
    };

    let shader = DiffuseShader::default();
    renderer.model(&shader, &matrix, &model);

    let mut out_file = File::create("demo/demo7.bmp")
        .expect("Should create file demo/demo7.bmp");
    write_bmp(renderer.image(), &mut out_file)
        .expect("Should save image");
}

#[derive(Default)]
struct DiffuseShader;

impl Shader<Vert, Mat4<f32>> for DiffuseShader {
    type VOut = Vert;

    fn vertex(&self, vertex: Vert, uniform: &Mat4<f32>) -> Vert {
        Vert {
            pos: (*uniform * vertex.pos.augment()).retro_project(),
            tex: vertex.tex,
            norm: vertex.norm,
        }
    }

    fn fragment(&self, input: Vert, _uniform: &Mat4<f32>) -> Color {
        let c = input.norm.normalized()
            .dot(Vec3(0.0f32, 1.0, 0.5).normalized());
        Color::float_rgb(c, c, c)
    }
}
