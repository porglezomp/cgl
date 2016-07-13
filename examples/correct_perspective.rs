extern crate cgl;

use std::fs::File;

use cgl::{Color, Renderer, write_bmp};
use cgl::{Shader, Vert, Model, Mat4, Vec2, Vec3, Vec4};

fn main() {
    let model = Model {
        vertices: vec![
            Vert {
                pos: Vec3(-1.0, 0.0, -1.0),
                tex: Vec2(0.0, 0.0),
                norm: Vec3(0.0, 1.0, 0.0),
            },
            Vert {
                pos: Vec3(-1.0, 0.0, 1.0),
                tex: Vec2(0.0, 1.0),
                norm: Vec3(0.0, 1.0, 0.0),
            },
            Vert {
                pos: Vec3(1.0, 0.0, 1.0),
                tex: Vec2(1.0, 1.0),
                norm: Vec3(0.0, 1.0, 0.0),
            },
            Vert {
                pos: Vec3(1.0, 0.0, -1.0),
                tex: Vec2(1.0, 0.0),
                norm: Vec3(0.0, 1.0, 0.0),
            },
        ],
        triangles: vec![
            [0, 1, 2],
            [0, 2, 3],
        ],
    };

    let mut renderer = Renderer::with_dimensions(512, 512);

    let matrix = {
        let viewport = Mat4::viewport(512, 512);
        let perspective = Mat4::perspective(5.0);
        let model = Mat4::identity();
        let view = Mat4::lookat(Vec3(0.0, 0.0, 0.0),
                                Vec3(0.3, 0.5, 0.5),
                                Vec3(0.0, 1.0, 0.0));
        viewport * perspective * view * model
    };

    let shader = Checkerboard;

    renderer.model(&shader, &matrix, &model);

    let mut out_file = File::create("demo/demo10.bmp")
        .expect("Should create file demo/demo10.bmp");
    write_bmp(&renderer.image(), &mut out_file)
        .expect("Should save file");
}

struct Checkerboard;

impl Shader<Vert, Mat4<f32>> for Checkerboard {
    type VOut = Vert;

    fn vertex(&self, vert: Vert, mvp: &Mat4<f32>, pos: &mut Vec4<f32>) -> Vert {
        *pos = *mvp * vert.pos.augment();
        vert
    }

    fn fragment(&self, input: Vert, _: &Mat4<f32>) -> Color {
        let c = (input.tex.0 * 10.0).floor() + (input.tex.1 * 10.0).floor();
        if c as u32 % 2 == 0 { Color::red() } else { Color::blue() }
    }
}
