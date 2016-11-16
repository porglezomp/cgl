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
    let normals = {
        let mut file = File::open("assets/african_head/global_normal.bmp")
            .expect("Should open assets/african_head/global_normal.bmp");
        read_bmp(&mut file).expect("Should read image")
    };

    let shader = TexturedGlobalNormal;

    renderer.model(&shader, &(matrix, texture, normals), &model);

    demo::save(renderer.image(), 10);
}

struct TexturedGlobalNormal;

impl Shader<Vert, (Mat4<f32>, Image<Color>, Image<Color>)> for TexturedGlobalNormal {
    type VOut = Vert;

    fn vertex(&self, vertex: Vert,
              &(matrix, _, _): &(Mat4<f32>, Image<Color>, Image<Color>),
              pos: &mut Vec4<f32>)
              -> Vert
    {
        *pos = matrix * vertex.pos.augment();
        vertex
    }

    fn fragment(&self, input: Vert,
                &(_, ref texture, ref nmap): &(Mat4<f32>, Image<Color>, Image<Color>))
                -> Color
    {
        let norm = nmap.sample_clamp(input.tex.0, input.tex.1);
        let normal = Vec3(norm.r as f32, norm.g as f32, norm.b as f32);
        let normal = (normal - Vec3(128.0, 128.0, 128.0)) / 128.0;
        let light = normal.dot(Vec3(0.2f32, 1.0, 0.4).normalized()) + 0.2;
        let albedo = texture.sample_clamp(input.tex.0, input.tex.1);
        albedo * light
    }
}
