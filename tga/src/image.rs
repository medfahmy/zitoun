use crate::color::Color;
use crate::point::Point;
use std::fs::File;
use std::io::{Read, Result, Write};

#[derive(Default)]
pub struct Header {
   idlength: u8,
   colourmaptype: u8,
   datatypecode: u8,
   colourmaporigin: u16,
   colourmaplength: u16,
   colourmapdepth: u8,
   x_origin: u16,
   y_origin: u16,
   width: u16,
   height: u16,
   bitsperpixel: u8,
   imagedescriptor: u8,
}

impl From<Vec<u8>> for Header {
    fn from(value: Vec<u8>) -> Self {
        todo!()
    }
}

pub struct Image<T: Color> where {
    width: u16,
    height: u16,
    buffer: Vec<T>,
}

impl<T: Color> Image<T> {
    pub fn new(width: u16, height: u16) -> Self {
        Image {
            buffer: vec![T::default(); (width * height * T::BPP as u16).into()],
            width,
            height,
        }
    }

    pub fn load_rle_data(&mut self, reader: impl Read) {
        let pixel_count = self.width * self.height;
        let curr_pixel = 0;
        let curr_byte = 0;

        while (curr_pixel < pixel_count) {
            let chunk = 0;
            todo!()
        }
    }

    pub fn unload_rle_data(&mut self, writer: impl Write) {
        todo!()
    }

    pub fn read_tga_file(&mut self, filename: &'static str) {
        let mut file = File::open(filename).unwrap();
        let mut header_buf = Vec::new();
        file.read(&mut header_buf).unwrap();

        let header: Header = header_buf.into();

        let buf_size = (header.width * header.height * ((header.bitsperpixel >> 3) as u16)) as usize;
        let mut image_buf = vec![T::default(); buf_size];

        match header.datatypecode {
            2 | 3 => todo!(),
            10 | 11 => todo!(),
            _ => panic!("unknown file format: {}", header.datatypecode),
        }

        if (header.imagedescriptor & 0x20) == 0 {
            self.flip_vertically();
        }

        if (header.imagedescriptor & 0x10) != 0 {
            self.flip_horizontally();
        }
    }

    pub fn write_tga_file(filename: String, rle: bool) {}

    pub fn flip_horizontally(&mut self) -> bool {
        false
    }

    pub fn flip_vertically(&mut self) -> bool {
        false
    }

    pub fn scale(&mut self, w: u64, h: u64) {}

    pub fn get(&self, p: Point) -> T {
        todo!()
    }

    pub fn set(&mut self, p: Point, color: T) {}

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn bpp(&self) -> u8 {
        T::BPP
    }

    pub fn buffer(&self) -> &[T] {
        &self.buffer
    }

    pub fn clear(&mut self) {}

    pub fn line(&mut self, p1: Point, p2: Point) {}
}
