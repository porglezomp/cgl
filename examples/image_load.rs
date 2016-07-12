extern crate cgl;

use cgl::{read_bmp, write_bmp};
use std::fs::File;

fn main() {
    let image = {
        let mut in_file = File::open("demo/demo7.bmp")
            .expect("Should open demo/demo7.bmp");
        read_bmp(&mut in_file).expect("Should read bmp")
    };

    let mut out_file = File::create("demo/demo8.bmp")
        .expect("Should create demo/demo8.bmp");
    write_bmp(&image, &mut out_file)
        .expect("Should write image");
}
