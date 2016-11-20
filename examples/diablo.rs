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

const WIDTH: usize = 2048;
const HEIGHT: usize = 2048;

fn main() {
    let model = compute_tangent_space(demo::diablo_model());
    let mut renderer = Renderer::with_dimensions(WIDTH, HEIGHT);

    let view_matrix = {
        let viewport = Mat4::viewport(WIDTH as i32, HEIGHT as i32);
        let perspective = Mat4::perspective(1.0);
        viewport * perspective
    };
    let model_matrix = {
        let model = Mat4::identity();
        let view = Mat4::lookat(Vec3(0.0, 0.0, 0.0),
                                Vec3(0.0, 0.1, 0.6),
                                Vec3(0.0, 1.0, 0.0));
        view * model
    };

    let diffuse = load_image("assets/diablo/diffuse.bmp");
    let normals = load_image("assets/diablo/normal.bmp");
    let glow = load_image("assets/diablo/glow.bmp");
    let spec = load_image("assets/diablo/spec.bmp");

    let shader = Diablo;

    renderer.model(&shader, &Uniform {
        view: view_matrix,
        model: model_matrix,
        diff: diffuse,
        norm: normals,
        glow: glow,
        spec: spec,
    }, &model);

    demo::save(renderer.image(), 13);
}

struct Uniform {
    view: Mat4<f32>,
    model: Mat4<f32>,
    diff: Image<Color>,
    norm: Image<Color>,
    glow: Image<Color>,
    spec: Image<Color>,
}

struct Diablo;

impl Shader<TanVert, Uniform> for Diablo {
    type VOut = TanVert;

    fn vertex(&self, vertex: TanVert, uniform: &Uniform, pos: &mut Vec4<f32>) -> TanVert {
        *pos = uniform.view * uniform.model * vertex.pos.augment();
        vertex
    }

    fn fragment(&self, input: TanVert, uniform: &Uniform) -> Color {
        let normal = {
            let norm = uniform.norm.sample_clamp(input.tex.0, input.tex.1);
            let tspace = Vec3(norm.r as f32, norm.g as f32, norm.b as f32);
            let tspace = (tspace - Vec3(128.0, 128.0, 128.0)) / 128.0;
            input.tan * tspace.0 + input.bitan * tspace.1 + input.norm * tspace.2
        };

        let light_vector = Vec3(0.2f32, 1.0, 1.4).normalized();
        let light_vector2 = Vec3(0.2f32, -1.5, -0.4).normalized();
        let mut light = normal.dot(light_vector).max(0.0);
        light += normal.dot(light_vector2).max(0.0) * 0.7;
        let glow = uniform.glow.sample_clamp(input.tex.0, input.tex.1);
        let albedo = uniform.diff.sample_clamp(input.tex.0, input.tex.1);

        let pos = uniform.model * input.pos.augment();
        let pos = (Vec3(pos.0, pos.1, pos.2) + Vec3(0.0, 0.0, -1.0)) *
            Vec3(1.0, 1.0, -1.0);
        let mut spec =
            (-light_vector + pos.normalized()).normalized().dot(normal).powf(18.0);
        spec += (-light_vector2 + pos.normalized()).normalized().dot(normal).powf(18.0);
        spec *= 0.3;
        // let spec_color = uniform.spec.sample_clamp(input.tex.0, input.tex.1);
        let spec_color = Color::float_rgb(1.0, 1.0, 1.0);

        (albedo * light) + (glow * 5.0) + (spec_color * spec)
    }
}
