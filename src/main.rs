#![allow(warnings, unused)]

use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::process::exit;

const WIDTH:  usize = 800;
const HEIGHT: usize = 800;

// 0xAARRBBGG

struct Point(usize, usize);

#[derive(Clone, Copy)]
struct Color(u32);

const WHITE: Color = Color(0xffffffff);
const BLACK: Color = Color(0xff000000);
const GRAY:  Color = Color(0xff202020);
const RED:   Color = Color(0xffff2020);
const GREEN: Color = Color(0xff2020ff);
const BLUE:  Color = Color(0xff20ff20);

fn main() {
    let mut pixels = [Color(0); WIDTH * HEIGHT];

    // chess(&mut pixels);
    // checker(&mut pixels);
    // circle(&mut pixels);

    draw_line(&mut pixels, Point(0, 0), Point(WIDTH, HEIGHT), RED);
    draw_line(&mut pixels, Point(0, 0), Point(WIDTH, 2 * HEIGHT), BLUE);
    draw_line(&mut pixels, Point(0, 0), Point(2 * WIDTH, HEIGHT), GREEN);

    save_to_ppm(&pixels, "line.ppm");
}


fn draw_line(pixels: &mut [Color], p: Point, r: Point, color: Color) {
    let dx = (r.0 as f32 - p.0 as f32) as f32;
    let dy = (r.1 as f32 - p.1 as f32) as f32;

    if dx != 0.0 {
        let a = dy / dx;
        let b = p.1 as f32 - a * p.0 as f32;

        for y in p.1..r.1 {
            for x in p.0..r.0 {
                if ((x as f32 * a + b) - y as f32) == 0.0 {
                    let idx = y * WIDTH + x;
                    if idx < WIDTH * HEIGHT {
                        pixels[y * WIDTH + x] = color;
                    }
                }
            }
        }
    } else {
        if p.0 < WIDTH {
            let (y1, y2) = if p.1 < r.1 { (p.1, r.1) } else { (r.1, p.1) };

            for y in y1..y2 {
                if y < HEIGHT {
                    pixels[y * WIDTH + p.0] = color;
                }
            }
        }
    }
}

fn circle(pixels: &mut [Color]) {
    fill(pixels, GRAY);

    let cols = 8;
    let rows = 8;
    let cell_width = WIDTH / rows;
    let cell_height = HEIGHT / cols;

    for y in 0..cols {
        for x in 0..rows {
            let t = (x as f32 / rows as f32 + y as f32 / cols as f32) / 2.0;
            let r = cell_height.min(cell_width) / 2;
            let r = lerp(r / 4, r, t);

            fill_circle(
                pixels,
                WIDTH,
                HEIGHT,
                x * cell_width + cell_width / 2,
                y * cell_height + cell_height / 2,
                r as usize,
                if (x + y) % 2 == 0 { RED } else { GREEN },
            );
        }
    }

    save_to_ppm(&pixels, "circle.ppm");
}

fn lerp(min: usize, max: usize, t: f32) -> f32 {
    min as f32 + (max - min) as f32 * t
}

fn fill_circle(
    pixels: &mut [Color],
    width: usize,
    height: usize,
    cx: usize,
    cy: usize,
    r: usize,
    color: Color,
) {
    let x1 = if cx > r { cx - r } else { 0 };
    let x2 = if cx + r < width { cx + r } else { width };
    let y1 = if cy > r { cy - r } else { 0 };
    let y2 = if cy + r < height { cy + r } else { height };

    for y in y1..y2 {
        for x in x1..x2 {
            let dx = if x > cx { x - cx } else { cx - x };
            let dy = if y > cy { y - cy } else { cy - y };

            if (dx * dx + dy * dy) <= r.pow(2) {
                pixels[y * width + x] = color;
            }
        }
    }
}

fn chess(pixels: &mut [Color]) {
    let cols = 8;
    let rows = 8;
    let cell_height = HEIGHT / cols;
    let cell_width = WIDTH / rows;

    for y in 0..cols {
        for x in 0..rows {
            fill_rect(pixels, WIDTH, HEIGHT, x * cell_width, y * cell_height, cell_width, cell_height, 
                if (x + y) % 2 == 0 { BLACK } else { WHITE }
            )
        }
    }

    save_to_ppm(pixels, "chess.ppm");
}

fn checker(pixels: &mut [Color]) {
    fill(pixels, GRAY);

    let cols = 8;
    let rows = 6;
    let cell_width = WIDTH / cols;
    let cell_height = HEIGHT / rows;

    for y in 0..rows {
        for x in 0..cols {
            let color = if (x + y) % 2 == 0 {
                RED
            } else {
                GRAY
            };

            fill_rect(
                pixels,
                WIDTH,
                HEIGHT,
                x * cell_width,
                y * cell_height,
                cell_width,
                cell_height,
                color,
            );
        }
    }

    save_to_ppm(&pixels, "checker.ppm");
}

fn fill(pixels: &mut [Color], color: Color) {
    for i in 0..pixels.len() {
        pixels[i] = color;
    }
}

fn fill_rect(
    pixels: &mut [Color],
    width: usize,
    height: usize,
    x0: usize,
    y0: usize,
    w: usize,
    h: usize,
    color: Color,
) {
    for dy in 0..h {
        let y = y0 + dy;

        if y < height {
            for dx in 0..w {
                let x = x0 + dx;

                if x < width {
                    pixels[y * height + x] = color;
                }
            }
        }
    }
}

fn error(e: impl Error) {
    eprintln!("error: could not create or open file: {}", e);
    exit(1);
}

fn save_to_ppm(pixels: &[Color], path: &str) {
    let mut file = File::create(path).map_err(error).unwrap();

    writeln!(file, "P6\n{} {} 255\n", WIDTH, HEIGHT)
        .map_err(error)
        .unwrap();

    for i in 0..WIDTH * HEIGHT {
        let pixel = pixels[i];

        let bytes: [u8; 3] = [
            ((pixel.0 >> (8 * 0)) & 0xFF) as u8,
            ((pixel.0 >> (8 * 1)) & 0xFF) as u8,
            ((pixel.0 >> (8 * 2)) & 0xFF) as u8,
        ];


        file.write(&bytes).map_err(error).unwrap();
    }

    println!("generated '{}'", path);
}
