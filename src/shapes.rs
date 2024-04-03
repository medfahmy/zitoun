pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub struct Line {
    pub a: Point,
    pub b: Point,
}

pub struct Rect {
    pub edge: Point,
    pub width: usize,
    pub height: usize,
}

pub struct Circle {
    pub center: Point,
    pub radius: usize,
}

pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}