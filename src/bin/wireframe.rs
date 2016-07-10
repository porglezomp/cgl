extern crate cgl;

use cgl::{Image, Color, Model, Vec3, write_bmp};

use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("african_head.obj").expect("should open african_head.obj");
    let model = Model::from_reader(BufReader::new(file)).expect("should load model");
    let mut image = Image::with_dimensions(512, 512);

    for tri in model.triangles {
        let t0 = model.vertices[tri[0] as usize] * 256.0 + Vec3(256.0, -256.0, 0.0);
        let t1 = model.vertices[tri[1] as usize] * 256.0 + Vec3(256.0, -256.0, 0.0);
        let t2 = model.vertices[tri[2] as usize] * 256.0 + Vec3(256.0, -256.0, 0.0);

        image.line(t0.0 as isize, t0.1 as isize, t1.0 as isize, t1.1 as isize, Color::white());
        image.line(t1.0 as isize, t1.1 as isize, t2.0 as isize, t2.1 as isize, Color::white());
        image.line(t2.0 as isize, t2.1 as isize, t0.0 as isize, t0.1 as isize, Color::white());
    }

    let mut img_out = File::create("demo.bmp").expect("should create demo.bmp");
    write_bmp(&image, &mut img_out).expect("should save image");
}
