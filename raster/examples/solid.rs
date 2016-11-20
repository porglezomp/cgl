extern crate cgl;

use cgl::{Image, Color, Vec2, Vec3};

mod demo;

fn main() {
    let model = demo::african_head_obj();
    let mut image = Image::with_dimensions(512, 512);

    for tri in model.triangles {
        let t0 = model.vertices[tri[0] as usize] * 256.0 + Vec3(256.0, -256.0, 256.0);
        let t1 = model.vertices[tri[1] as usize] * 256.0 + Vec3(256.0, -256.0, 256.0);
        let t2 = model.vertices[tri[2] as usize] * 256.0 + Vec3(256.0, -256.0, 256.0);

        let normal = (t1-t0).cross(t2-t0).normalized();
        let shade1 = normal.dot(Vec3(0.0f32, 1.0, 0.5).normalized());
        let shade2 = normal.dot(Vec3(0.0, 0.0, 1.0));
        if shade2 < 0.0 {
            continue;
        }

        image.triangle(Vec2(t0.0 as isize, -t0.1 as isize),
                       Vec2(t1.0 as isize, -t1.1 as isize),
                       Vec2(t2.0 as isize, -t2.1 as isize),
                       Color::float_rgb(shade1 * 1.2, shade1, shade1 * 0.8));
    }

    demo::save(&image, 2);
}
