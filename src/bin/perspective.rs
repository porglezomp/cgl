extern crate cgl;

use std::fs::File;
use std::io::BufReader;

use cgl::{Color, Image, Model, Mat4, Vec3, write_bmp};

fn main() {
    let model_file = File::open("african_head.obj")
        .expect("Should open african_head.obj");
    let model = Model::from_reader(BufReader::new(model_file))
        .expect("Should parse model");

    let mut image = Image::with_dimensions(512, 512);
    let mut zbuf = Image::filled(512, 512, std::f32::MIN);

    let perspective = {
        let mut p = Mat4::identity();
        p[(2, 3)] = 5.0;
        p[(3, 2)] = -1.0 / 3.0;
        p
    };

    for tri in model.triangles {
        let t0 = model.vertices[tri[0] as usize];
        let t1 = model.vertices[tri[1] as usize];
        let t2 = model.vertices[tri[2] as usize];

        let normal = (t1-t0).cross(t2-t0).normalized();
        let shade = normal.dot(Vec3(0.0f32, 1.0, 0.5).normalized());

        image.triangle3((perspective * t0.augment()).retro_project(),
                        (perspective * t1.augment()).retro_project(),
                        (perspective * t2.augment()).retro_project(),
                        &mut zbuf,
                        Color::float_rgb(shade * 1.2, shade, shade * 0.8));
    }

    let mut out_file = File::create("demo5.bmp")
        .expect("Should create demo5.bmp");
    write_bmp(&image, &mut out_file)
        .expect("Should save image");
}
