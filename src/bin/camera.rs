extern crate cgl;

use cgl::{Color, Obj, Mat4, Renderer, Vec3, write_bmp};

use std::fs::File;
use std::io::BufReader;

fn main() {
    let model_file = File::open("african_head.obj")
        .expect("Should open african_head.obj");
    let model = Obj::from_reader(BufReader::new(model_file))
        .expect("Should parse model");

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

    let (transformed, tris) = model.transform(matrix);
    for tri in tris {
        let t0 = transformed[tri[0] as usize];
        let t1 = transformed[tri[1] as usize];
        let t2 = transformed[tri[2] as usize];

        let v0 = model.vertices[tri[0] as usize];
        let v1 = model.vertices[tri[1] as usize];
        let v2 = model.vertices[tri[2] as usize];
        let normal = (v1-v0).cross(v2-v0).normalized();
        let c = normal.dot(Vec3(0.0f32, 1.0, 0.5).normalized());

        renderer.triangle(t0, t1, t2, Color::float_rgb(c * 1.2, c, c * 0.8));
    }

    let mut out_file = File::create("demo6.bmp")
        .expect("Should create demo6.bmp");
    write_bmp(renderer.image(), &mut out_file)
        .expect("Should save image");
}
