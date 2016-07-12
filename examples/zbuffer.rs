extern crate cgl;

use cgl::{Image, Color, Obj, Vec3, write_bmp};

use std::io::BufReader;
use std::fs::File;

fn main() {
    let model_file = File::open("assets/african_head/african_head.obj")
        .expect("Should open assets/african_head/african_head.obj");
    let model = Obj::from_reader(BufReader::new(model_file))
        .expect("Should parse the model");

    let mut image = Image::with_dimensions(512, 512);
    let mut zbuf = Image::filled(512, 512, std::f32::MIN);

    for tri in model.triangles {
        let t0 = model.vertices[tri[0] as usize];
        let t1 = model.vertices[tri[1] as usize];
        let t2 = model.vertices[tri[2] as usize];

        let normal = (t1-t0).cross(t2-t0).normalized();
        if normal.2 < 0.0 { continue; }
        let shade = normal.dot(Vec3(0.0f32, 1.0, 0.5).normalized());

        image.triangle3(t0, t1, t2, &mut zbuf,
                        Color::float_rgb(shade*1.2, shade, shade*0.8));
    }

    let mut out_file = File::create("demo/demo4.bmp")
        .expect("Should create demo/demo4.bmp");
    write_bmp(&image, &mut out_file)
        .expect("Should write the output image");
}
