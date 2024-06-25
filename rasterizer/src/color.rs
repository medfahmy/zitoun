pub struct TGAColor{
    rgba: (u8, u8, u8, u8),
    bytespp: i32,
}


impl TGAColor {
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        TGAColor {
            rgba: (r, g, b, a),
            bytespp: 0,
        }
    }
}
