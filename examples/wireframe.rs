extern crate cgl;

use cgl::{Image, Color, Obj, Vec3, write_bmp};

use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("assets/african_head/african_head.obj")
        .expect("should open assets/african_head/african_head.obj");
    let model = Obj::from_reader(BufReader::new(file))
        .expect("should load model");
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

    let mut img_out = File::create("demo/demo001.bmp")
        .expect("should create demo001.bmp");
    write_bmp(&image, &mut img_out)
        .expect("should save image");
}
