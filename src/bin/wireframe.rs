extern crate cgl;

use cgl::obj::Model;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("african_head.obj").expect("should open african_head.obj");
    let model = Model::from_reader(BufReader::new(file));
}
