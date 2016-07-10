extern crate cgl;

use cgl::{Image, Color};
use cgl::obj::{Model, Vertex};
use cgl::bmp::write_bmp;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("african_head.obj").expect("should open african_head.obj");
    let model = Model::from_reader(BufReader::new(file)).expect("should load model");
    let mut image = Image::with_dimensions(512, 512);

    for tri in model.triangles {
        let Vertex(x0, y0, _) = model.vertices[tri[0] as usize];
        let Vertex(x1, y1, _) = model.vertices[tri[1] as usize];
        let Vertex(x2, y2, _) = model.vertices[tri[2] as usize];

        let x0 = x0 * 256.0 + 256.0;
        let y0 = -y0 * 256.0 + 256.0;
        let x1 = x1 * 256.0 + 256.0;
        let y1 = -y1 * 256.0 + 256.0;
        let x2 = x2 * 256.0 + 256.0;
        let y2 = -y2 * 256.0 + 256.0;

        image.line(x0 as isize, y0 as isize, x1 as isize, y1 as isize, Color::white());
        image.line(x1 as isize, y1 as isize, x2 as isize, y2 as isize, Color::white());
        image.line(x2 as isize, y2 as isize, x0 as isize, y0 as isize, Color::white());
    }

    let mut img_out = File::create("demo.bmp").expect("should create demo.bmp");
    write_bmp(&image, &mut img_out).expect("should save image");
}
