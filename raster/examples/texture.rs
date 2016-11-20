extern crate cgl;

use std::fs::File;

use cgl::{Color, Image, Renderer, read_bmp};
use cgl::{Shader, Vert, Mat4, Vec3, Vec4};

mod demo;

fn main() {
    let model = demo::african_head();
    let mut renderer = Renderer::with_dimensions(512, 512);
    let matrix = demo::african_head_matrix();

    let texture = {
        let mut file = File::open("assets/african_head/diffuse.bmp")
            .expect("Should open assets/african_head/diffuse.bmp");
        read_bmp(&mut file).expect("Should read image")
    };

    let shader = TexturedDiffuse;

    renderer.model(&shader, &(matrix, texture), &model);

    demo::save(renderer.image(), 9);
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

    fn fragment(&self, input: Vert, &(_, ref texture): &(Mat4<f32>, Image<Color>))
                -> Color
    {
        let light = input.norm.normalized()
            .dot(Vec3(0.2f32, 1.0, 0.4).normalized()) + 0.2;
        let albedo = texture.sample_clamp(input.tex.0, input.tex.1);
        albedo * light
    }
}
