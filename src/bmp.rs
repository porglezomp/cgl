use super::{Image, Color};

use std::io::{self, Read, Write};

const FULL_HEADER_SIZE: usize = 54;
const DATA_HEADER_SIZE: usize = 40;
const BPP: i16 = 24;
const PIXELS_PER_METER: usize = 2835;

pub fn write_bmp<W: Write>(image: &Image<Color>, writer: &mut W)
                           -> io::Result<usize>
{
    let row_size = (image.width * 3 + 3) & !3;
    let row_padding = row_size - image.width * 3;
    let image_size = FULL_HEADER_SIZE + row_size * image.height;
    try!(writer.write(b"BM"));
    try!(writer.write(&le32(image_size as i32)));
    try!(writer.write(&[0, 0, 0, 0])); // Reserved
    try!(writer.write(&le32(FULL_HEADER_SIZE as i32)));
    try!(writer.write(&le32(DATA_HEADER_SIZE as i32)));
    try!(writer.write(&le32(image.width as i32)));
    try!(writer.write(&le32(image.height as i32)));
    try!(writer.write(&le16(1)));
    try!(writer.write(&le16(BPP)));
    try!(writer.write(&le32(0)));
    try!(writer.write(&le32((row_size * image.height) as i32)));
    try!(writer.write(&le32(PIXELS_PER_METER as i32)));
    try!(writer.write(&le32(PIXELS_PER_METER as i32)));
    try!(writer.write(&le32(0))); // Indexed colors in image
    try!(writer.write(&le32(0))); // Important colors in image

    for chunk in image.bytes().chunks(image.width * 3).rev() {
        try!(writer.write(chunk));
        try!(writer.write(&[0, 0, 0, 0][..row_padding]));
    }
    Ok(image_size)
}

pub fn read_bmp<R: Read>(reader: &mut R) -> io::Result<Image<Color>> {
    let mut header = [0; FULL_HEADER_SIZE];
    try!(reader.read_exact(&mut header));
    if &header[..2] != b"BM" {
        return Err(io::Error::new(io::ErrorKind::Other,
                                  "The magic number should be BM"));
    }
    let width = read_le32(&header[18..22]) as usize;
    let height = read_le32(&header[22..26]) as usize;
    let bpp = read_le16(&header[28..30]);
    if bpp != BPP {
        return Err(io::Error::new(io::ErrorKind::Other,
                                  "Only 24 BPP is supported"));
    }
    let row_width = (width * 3 + 3) & !3;
    let mut image_buf = vec![0; row_width * height];
    try!(reader.read_exact(&mut image_buf));
    let mut image = Image::with_dimensions(width, height);
    for (source, mut dest) in image_buf.chunks(row_width).rev()
            .zip(image.bytes_mut().chunks_mut(width * 3)) {
        dest.copy_from_slice(&source[..width * 3]);
    }
    Ok(image)
}

fn le16(value: i16) -> [u8; 2] {
    [(value >> 0) as u8,
     (value >> 8) as u8]
}

fn le32(value: i32) -> [u8; 4] {
    [(value >>  0) as u8,
     (value >>  8) as u8,
     (value >> 16) as u8,
     (value >> 24) as u8]
}

fn read_le16(value: &[u8]) -> i16 {
    (value[0] as i16) << 0 |
    (value[1] as i16) << 8
}

fn read_le32(value: &[u8]) -> i32 {
    (value[0] as i32) <<  0 |
    (value[1] as i32) <<  8 |
    (value[2] as i32) << 16 |
    (value[3] as i32) << 24
}
