extern crate cgl;

use cgl::{Image, Color};
use cgl::obj::Model;
use cgl::math::{Vec2, Vec3};
use cgl::bmp::write_bmp;

use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("african_head.obj").expect("should open african_head.obj");
    let model = Model::from_reader(BufReader::new(file)).expect("should load model");
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

        fn map(col: f32) -> u8 {
            if col < 0.0 {
                0
            } else if col > 1.0 {
                255
            } else {
                (col * 255.0) as u8
            }
        }
        image.triangle(Vec2(t0.0 as isize, -t0.1 as isize),
                       Vec2(t1.0 as isize, -t1.1 as isize),
                       Vec2(t2.0 as isize, -t2.1 as isize),
                       Color(map(shade1 * 0.8), map(shade1), map(shade1 * 1.2)));
    }

    let mut img_out = File::create("demo2.bmp").expect("should create demo.bmp");
    write_bmp(&image, &mut img_out).expect("should save image");
}
