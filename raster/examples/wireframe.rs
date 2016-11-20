extern crate cgl;

use cgl::{Image, Color, Vec3};

mod demo;

fn main() {
    let model = demo::african_head_obj();
    let mut image = Image::with_dimensions(512, 512);

    let scale = Vec3(256.0, -256.0, 1.0);
    let offset = Vec3(256.0, 256.0, 0.0);
    for tri in model.triangles {
        let t0 = model.vertices[tri[0] as usize] * scale + offset;
        let t1 = model.vertices[tri[1] as usize] * scale + offset;
        let t2 = model.vertices[tri[2] as usize] * scale + offset;

        image.line(t0.0 as isize, t0.1 as isize, t1.0 as isize, t1.1 as isize, Color::white());
        image.line(t1.0 as isize, t1.1 as isize, t2.0 as isize, t2.1 as isize, Color::white());
        image.line(t2.0 as isize, t2.1 as isize, t0.0 as isize, t0.1 as isize, Color::white());
    }

    demo::save(&image, 1);
}
