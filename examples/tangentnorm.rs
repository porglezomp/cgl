extern crate cgl;

use std::fs::File;

use cgl::{Color, Image, Renderer, read_bmp};
use cgl::{Shader, TanVert, Mat4, Vec3, Vec4};
use cgl::model::compute_tangent_space;

mod demo;

fn main() {
    let model = compute_tangent_space(demo::african_head());
    let mut renderer = Renderer::with_dimensions(512, 512);
    let matrix = demo::african_head_matrix();

    let texture = {
        let mut file = File::open("assets/african_head/diffuse.bmp")
            .expect("Should open assets/african_head/diffuse.bmp");
        read_bmp(&mut file).expect("Should read image")
    };

    let normals = {
        let mut file = File::open("assets/african_head/normal.bmp")
            .expect("Should open assets/african_head/normal.bmp");
        read_bmp(&mut file).expect("Should read image")
    };

    let shader = TexturedNormal;

    renderer.model(&shader, &(matrix, texture, normals), &model);

    demo::save(renderer.image(), 12);
}

struct TexturedNormal;

impl Shader<TanVert, (Mat4<f32>, Image<Color>, Image<Color>)> for TexturedNormal {
    type VOut = TanVert;

    fn vertex(&self, vertex: TanVert,
              &(matrix, _, _): &(Mat4<f32>, Image<Color>, Image<Color>),
              pos: &mut Vec4<f32>)
              -> TanVert
    {
        *pos = matrix * vertex.pos.augment();
        vertex
    }

    fn fragment(&self, input: TanVert,
                &(_, ref texture, ref nmap): &(Mat4<f32>, Image<Color>, Image<Color>))
                -> Color
    {
        let norm = nmap.sample_clamp(input.tex.0, input.tex.1);
        let tspace = Vec3(norm.r as f32, norm.g as f32, norm.b as f32);
        let tspace = (tspace - Vec3(128.0, 128.0, 128.0)) / 128.0;
        let normal = input.tan * tspace.0 + input.bitan * tspace.1 + input.norm * tspace.2;
        let light = normal.dot(Vec3(0.2f32, 1.0, 0.4).normalized()) + 0.2;
        let albedo = texture.sample_clamp(input.tex.0, input.tex.1);
        albedo * light
    }
}
