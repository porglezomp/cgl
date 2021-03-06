extern crate cgl;

use cgl::{Color, Renderer, Mat4, Vec3};

mod demo;

fn main() {
    let model = demo::african_head_obj();

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

    demo::save(renderer.image(), 6);
}
