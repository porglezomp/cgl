extern crate cgl;

use std::fs::File;

use cgl::{Color, Image, Renderer, read_bmp};
use cgl::{Shader, TanVert, Mat4, Vec3, Vec4};
use cgl::model::compute_tangent_space;

mod demo;

fn load_image(fname: &str) -> Image<Color> {
    let mut file = File::open(fname)
        .expect(&format!("Should open {}", fname));
    read_bmp(&mut file).expect("Should read image")
}

fn main() {
    let model = compute_tangent_space(demo::diablo_model());
    let mut renderer = Renderer::with_dimensions(512, 512);
    let matrix = demo::african_head_matrix();

    let diffuse = load_image("assets/diablo/diffuse.bmp");
    let normals = load_image("assets/diablo/normal.bmp");
    let glow = load_image("assets/diablo/glow.bmp");
    let spec = load_image("assets/diablo/spec.bmp");

    let shader = Diablo;

    renderer.model(&shader, &Uniform {
        view: matrix,
        diff: diffuse,
        norm: normals,
        glow: glow,
        spec: spec,
    }, &model);

    demo::save(renderer.image(), 13);
}

struct Uniform {
    view: Mat4<f32>,
    diff: Image<Color>,
    norm: Image<Color>,
    glow: Image<Color>,
    spec: Image<Color>,
}

struct Diablo;

impl Shader<TanVert, Uniform> for Diablo {
    type VOut = TanVert;

    fn vertex(&self, vertex: TanVert, uniform: &Uniform, pos: &mut Vec4<f32>) -> TanVert {
        *pos = uniform.view * vertex.pos.augment();
        vertex
    }

    fn fragment(&self, input: TanVert, uniform: &Uniform) -> Color {
        let norm = uniform.norm.sample_clamp(input.tex.0, input.tex.1);
        let tspace = Vec3(norm.r as f32, norm.g as f32, norm.b as f32);
        let tspace = (tspace - Vec3(128.0, 128.0, 128.0)) / 128.0;
        let normal = input.tan * tspace.0 + input.bitan * tspace.1 + input.norm * tspace.2;
        let light = normal.dot(Vec3(0.2f32, 1.0, 0.4).normalized());
        let glow = uniform.glow.sample_clamp(input.tex.0, input.tex.1);
        let albedo = uniform.diff.sample_clamp(input.tex.0, input.tex.1);
        (albedo * light) + glow
    }
}
