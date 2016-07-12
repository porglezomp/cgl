extern crate cgl;

use cgl::obj::Obj;

fn main() {
    Obj::from_file("african_head.obj").expect("Should parse african_head.obj");
}
