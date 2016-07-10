use super::Image;

use std::io::{self, Write};

pub fn write_bmp<W: Write>(image: &Image, writer: &mut W) -> io::Result<usize> {
    let row_size = (image.width * 3 + 3) & !3;
    let row_padding = row_size - image.width * 3;
    let image_size = 54 + row_size * image.height;
    try!(writer.write(b"BM"));
    try!(writer.write(&le32(image_size as i32)));
    try!(writer.write(&[0, 0, 0, 0])); // Reserved
    try!(writer.write(&le32(0))); // Offset to image data
    try!(writer.write(&le32(40))); // Size of header
    try!(writer.write(&le32(image.width as i32)));
    try!(writer.write(&le32(image.height as i32)));
    try!(writer.write(&le16(1)));
    try!(writer.write(&le16(24)));
    try!(writer.write(&le32(0)));
    try!(writer.write(&le32((row_size * image.height) as i32)));
    try!(writer.write(&le32(2835))); // Pixels per meter
    try!(writer.write(&le32(2835))); // Pixels per meter
    try!(writer.write(&le32(0))); // Indexed colors in image
    try!(writer.write(&le32(0))); // Important colors in image

    for chunk in image.bytes().chunks(image.width * 3).rev() {
        try!(writer.write(chunk));
        try!(writer.write(&[0, 0, 0, 0][..row_padding]));
    }
    Ok(image_size)
}

fn le16(value: i16) -> [u8; 2] {
    [value as u8, (value >> 8) as u8]
}

fn le32(value: i32) -> [u8; 4] {
    [value as u8, (value >> 8) as u8, (value >> 16) as u8, (value >> 24) as u8]
}
