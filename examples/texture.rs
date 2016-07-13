extern crate cgl;

use std::fs::File;

use cgl::{Color, Image, Renderer, read_bmp, write_bmp};
use cgl::{Shader, Vert, Obj, Mat4, Vec3, Vec4};

fn main() {
    let model = Obj::from_file("assets/african_head/african_head.obj")
        .expect("Should load model from assets/african_head/african_head.obj")
        .model()
        .expect("Should convert to a single-indexed model");

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

    let texture = {
        let mut file = File::open("assets/african_head/diffuse.bmp")
            .expect("Should open assets/african_head/diffuse.bmp");
        read_bmp(&mut file).expect("Should read image")
    };

    let shader = TexturedDiffuse;

    renderer.model(&shader, &(matrix, texture), &model);

    let mut out_file = File::create("demo/demo9.bmp")
        .expect("Should create output file");
    write_bmp(&renderer.image(), &mut out_file)
        .expect("Should save file");
}

struct TexturedDiffuse;

impl Shader<Vert, (Mat4<f32>, Image<Color>)> for TexturedDiffuse {
    type VOut = Vert;

    fn vertex(&self, vertex: Vert, &(matrix, _): &(Mat4<f32>, Image<Color>),
              pos: &mut Vec4<f32>)
              -> Vert
    {
        *pos = matrix * vertex.pos.augment();
        vertex
    }

    fn fragment(&self, input: Vert, &(_, _): &(Mat4<f32>, Image<Color>))
                -> Color
    {
        Color::float_rgb(input.tex.0, input.tex.1, 0.0)
    }
}
