#![allow(dead_code)]

use cgl::{Color, Obj, Image, Model, Vert, Mat4, Vec3, read_bmp, write_bmp};

use std::fs::File;
use std::io::BufReader;

pub fn african_head_obj() -> Obj {
    let model_file = File::open("assets/african_head/african_head.obj")
        .expect("Should open assets/african_head/african_head.obj");
    Obj::from_reader(BufReader::new(model_file))
        .expect("Should parse obj")
}

pub fn african_head() -> Model<Vert> {
    african_head_obj().model()
        .expect("Should convert obj to model")
}

pub fn diablo_model() -> Model<Vert> {
    let model_file = File::open("assets/diablo/devilman.obj")
        .expect("Should open assets/diablo/devilman.obj");
    Obj::from_reader(BufReader::new(model_file))
        .expect("Should parse obj")
        .model()
        .expect("Should convert obj to model")
}

pub fn african_head_matrix() -> Mat4<f32> {
    let viewport = Mat4::viewport(512, 512);
    let perspective = Mat4::perspective(1.0);
    let model = Mat4::identity();
    let view = Mat4::lookat(Vec3(0.0, 0.0, 0.0),
                            Vec3(0.3, 0.2, 0.5),
                            Vec3(0.0, 1.0, 0.0));
    viewport * perspective * view * model
}

pub fn image_filename(number: u32) -> String {
    format!("output/demo{:03}.bmp", number)
}

pub fn save(image: &Image<Color>, number: u32) {
    let fname = image_filename(number);
    let mut out_file = File::create(&fname)
        .expect(&format!("Should create {}", fname));
    write_bmp(image, &mut out_file)
        .expect("Should save image");
}

pub fn load(number: u32) -> Image<Color> {
    let fname = image_filename(number);
    let mut in_file = File::open(&fname)
        .expect(&format!("Should open {}", fname));
    read_bmp(&mut in_file).expect("Should read bmp")
}
