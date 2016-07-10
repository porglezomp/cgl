extern crate cgl;
extern crate rand;

use cgl::{Image, Color};
use cgl::obj::{Model, Vertex};
use cgl::math::Vec2;
use cgl::bmp::write_bmp;

use std::fs::File;
use std::io::BufReader;
use rand::distributions::{IndependentSample, Range};

fn main() {
    let file = File::open("african_head.obj").expect("should open african_head.obj");
    let model = Model::from_reader(BufReader::new(file)).expect("should load model");
    let mut image = Image::with_dimensions(512, 512);
    let between = Range::new(0, 255);
    let mut rng = rand::thread_rng();

    for tri in model.triangles {
        let Vertex(x0, y0, _) = model.vertices[tri[0] as usize];
        let Vertex(x1, y1, _) = model.vertices[tri[1] as usize];
        let Vertex(x2, y2, _) = model.vertices[tri[2] as usize];

        let x0 = x0 * 256.0 + 256.0;
        let y0 = -y0 * 256.0 + 256.0;
        let t0 = Vec2(x0 as isize, y0 as isize);
        let x1 = x1 * 256.0 + 256.0;
        let y1 = -y1 * 256.0 + 256.0;
        let t1 = Vec2(x1 as isize, y1 as isize);
        let x2 = x2 * 256.0 + 256.0;
        let y2 = -y2 * 256.0 + 256.0;
        let t2 = Vec2(x2 as isize, y2 as isize);

        image.triangle(t0, t1, t2, Color(between.ind_sample(&mut rng),
                                         between.ind_sample(&mut rng),
                                         between.ind_sample(&mut rng)));
    }

    let mut img_out = File::create("demo2.bmp").expect("should create demo.bmp");
    write_bmp(&image, &mut img_out).expect("should save image");
}
