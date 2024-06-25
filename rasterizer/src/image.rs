use std::fs::File;
use std::io::{Read, Write};
use crate::point::Point;
use crate::color::TGAColor;

#[derive(Default)]
pub struct TGAHeader {
    id_length: char,
    colormap_type: char,
    data_typecode: char,
    color_map_depth: char,
    bits_per_pixel: char,
    descriptor: char,
    colormap_origin: u8,
    colormap_length: u8,
    x_origin: u8,
    y_origin: u8,
    width: u8,
    height: u8,
}

pub struct TGAImage {
    buffer: Vec<u8>,
    width: u64,
    height: u64,
    bytes_per_pixel: u64,
}

impl TGAImage {
    pub fn new(width: u64, height: u64, bytes_per_pixel: u64) -> Self {
        TGAImage {
            buffer: vec![0; (width * height * bytes_per_pixel).try_into().unwrap()],
            width,
            height,
            bytes_per_pixel,
        }
    }

    pub fn load_rle_data(&mut self, reader: impl Read) {}
    
    pub fn unload_rle_data(&mut self, writer: impl Write) {}

    pub fn read_tga_file(filename: &'static str) {
        let mut file = File::open(filename).unwrap();
        let mut header = TGAHeader::default();
        file.read(&mut header).unwrap();
        let mut data = vec![0; header.width * header.length * (header.bites_per_pixel >> 3)];

        match header.datatype_code {
            2 | 3 => file.read(&mut data).unwrap(),
            10 | 11 => self.load_rle_data(),
            _ => panic!("unknown file format: {}", header.datatype_code),
        }

        if !(header.descriptor & 0x20) {
            self.flip_vertically();
        }

        if header.descriptor & 0x10 {
            self.flip_horizontally();
        }
    }

    pub fn write_tga_file(filename: String, rle: bool) {}

    pub fn flip_horizontally(&mut self) -> bool { false }

    pub fn flip_vertically(&mut self) -> bool { false }

    pub fn scale(&mut self, w: u64, h: u64) {}

    pub fn get(&self, p: Point) -> TGAColor { todo!() }

    pub fn set(&mut self, p: Point, color: TGAColor) {}

    pub fn get_width(&self) -> u64 { self.width }

    pub fn get_height(&self) -> u64 { self.height }

    pub fn get_bytespp(&self) -> u64 { self.bytespp }

    pub fn buffer(&self) -> &[u8] { &self.data }

    pub fn clear(&mut self) {}

    pub fn line(&mut self, p1: Point, p2: Point) {}
}
