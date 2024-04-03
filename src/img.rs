use std::fs::File;
use std::io::Write;
use std::process::exit;
use crate::shapes::*;

#[derive(Clone, Copy)]
pub struct Color(pub u32);

pub struct Img {
    pixels: Vec<Color>,
    width: usize,
    height: usize,
}

impl Img {
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width <= 1000 && height <= 1000);

        Self {
            width,
            height,
            pixels: vec![Color(0); width * height],
        }
    }

    pub fn save_to_ppm(&self, path: &str) {
        let mut file = File::create(path).map_err(|e| {
            eprintln!("error: could not create or open file, {}", e);
            exit(1);
        }).unwrap();

        writeln!(file, "P6\n{} {} 255\n", self.width, self.height)
            .map_err(|e| {
                eprintln!("error: could not write ppm header to file, {}", e);
                exit(1);
            })
            .unwrap();

        for i in 0..self.width * self.height {
            let pixel = self.pixels[i].0;

            let bytes: [u8; 3] = [
                ((pixel >> (8 * 0)) & 0xFF) as u8,
                ((pixel >> (8 * 1)) & 0xFF) as u8,
                ((pixel >> (8 * 2)) & 0xFF) as u8,
            ];

            file.write(&bytes).map_err(|e| {
                eprintln!("error: could not write bytes to file, {}", e);
                exit(1);
            }).unwrap();
        }

        println!("generated '{}'", path);
    }

    pub fn fill(&mut self, color: Color) {
        for i in 0..self.height * self.width {
            self.pixels[i] = color;
        }
    }

    pub fn rect(&mut self, rect: Rect, color: Color) {
        for dy in 0..self.height {
            let y = rect.edge.y + dy;

            if y < self.height {
                for dx in 0..self.width {
                    let x = rect.edge.x + dx;

                    if x < self.width {
                        self.pixels[y * self.width + x] = color;
                    }
                }
            }
        }
    }

    pub fn line(&mut self, line: Line, color: Color) {
        let dx = (line.b.x as f32 - line.a.x as f32) as f32;
        let dy = (line.b.y as f32 - line.a.y as f32) as f32;

        if dx != 0.0 {
            let a = dy / dx;
            let b = line.a.y as f32 - a * line.a.x as f32;

            for y in line.a.y..line.b.y {
                for x in line.a.x..line.b.x {
                    if ((x as f32 * a + b) - y as f32) == 0.0 {
                        let idx = y * self.width + x;
                        if idx < self.width * self.height {
                            self.pixels[y * self.width + x] = color;
                        }
                    }
                }
            }
        } else {
            if line.a.x < self.width {
                let y1 = line.a.y.min(line.b.y);
                let y2 = line.a.y.max(line.b.y);

                for y in y1..y2 {
                    if y < self.height {
                        self.pixels[y * self.width + line.a.x] = color;
                    }
                }
            }
        }
    }

    pub fn triangle(&mut self, tri: Triangle, color: Color) {
        let p0x = tri.a.x as f32;
        let p0y = tri.a.y as f32;
        let p1x = tri.b.x as f32;
        let p1y = tri.b.y as f32;
        let p2x = tri.c.x as f32;
        let p2y = tri.c.y as f32;

        let area = 0.5 * (-p1y * p2x + p0y * (-p1x + p2x) + p0x * (p1y - p2y) + p1x * p2y);

        for y in 0..self.height {
            for x in 0..self.width {
                let s = 1.0 / (2.0 * area)
                    * (p0y * p2x - p0x * p2y + (p2y - p0y) * x as f32 + (p0x - p2x) * y as f32);
                let t = 1.0 / (2.0 * area)
                    * (p0x * p1y - p0y * p1x + (p0y - p1y) * x as f32 + (p1x - p0x) * y as f32);

                if s >= 0.0 && t >= 0.0 && s + t <= 1.0 {
                    self.pixels[y * self.width + x] = color;
                }
            }
        }
    }

    pub fn circle(&mut self, circle: Circle, color: Color) {
        let x1 = if circle.center.x > circle.radius {
            circle.center.x - circle.radius
        } else {
            0
        };

        // let x1: i64 = circle.center.

        let x2 = if circle.center.x + circle.radius < self.width {
            circle.center.x + circle.radius
        } else {
            self.width
        };

        let y1 = if circle.center.y > circle.radius {
            circle.center.y - circle.radius
        } else {
            0
        };

        let y2 = if circle.center.y + circle.radius < self.height {
            circle.center.y + circle.radius
        } else {
            self.height
        };

        for y in y1..y2 {
            for x in x1..x2 {
                let dx = if x > circle.center.x {
                    x - circle.center.x
                } else {
                    circle.center.x - x
                };

                let dy = if y > circle.center.y {
                    y - circle.center.y
                } else {
                    circle.center.y - y
                };

                if (dx * dx + dy * dy) <= circle.radius.pow(2) {
                    self.pixels[y * self.width + x] = color;
                }
            }
        }
    }
}
