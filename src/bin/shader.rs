extern crate cgl;

use cgl::obj::Obj;

fn main() {
    let model = Obj::from_file("african_head.obj")
        .expect("Should parse african_head.obj")
        .model()
        .expect("Should convert correctly");
    println!("{:?}", model);
}
