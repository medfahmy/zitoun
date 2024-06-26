pub trait Color: Clone + Default {
    const BPP: u8;
}

#[derive(Clone, Default)]
pub struct Grayscale(u8);

impl Color for Grayscale {
    const BPP: u8 = 1;
}

#[derive(Clone, Default)]
pub struct RGB(u8, u8, u8);

impl Color for RGB {
    const BPP: u8 = 3;
}

#[derive(Clone, Default)]
pub struct RGBA(u8, u8, u8, u8);

impl Color for RGBA {
    const BPP: u8 = 4;
}
