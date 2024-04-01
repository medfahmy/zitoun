use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::process::exit;

const WIDTH:  usize = 800;
const HEIGHT: usize = 800;

// 0xAARRBBGG
const GRAY:  u32 = 0xff202020;
const RED:   u32 = 0xffff2020;
const GREEN: u32 = 0xff2020ff;
const BLUE:  u32 = 0xff20ff20;

fn main() {
    let mut pixels = [0; WIDTH * HEIGHT];

    // chess(&mut pixels);
    // checker(&mut pixels);
    // circle(&mut pixels);
    draw_line(&mut pixels, Point(0, 0), Point(100, 100));
    draw_line(&mut pixels, Point(1, 0), Point(101, 100));
    save_to_ppm(&pixels, "line.ppm");
}

struct Point(usize, usize);

fn draw_line(pixels: &mut [u32], p: Point, r: Point) {

    // y = a * x + b

    // p.1 = a * p.0 + b
    // r.1 = a * r.0 + b
    // (p.1 - r.1) = a * (p.0 - r.0)

    let a = (p.1 as i64 - r.1 as i64)  / (p.0 as i64 - r.0 as i64);
    let b = p.1 as i64 - a * p.0 as i64; 

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if x as i64 * a + b == y as i64 {
                if (p.0 < x && x < r.0 && p.1 < y && y < r.1) || (p.0 > x && x > r.0 && p.1 > y && y > r.1) {
                    pixels[y * WIDTH + x] = RED;
                }
            }
        }
    }
}

fn circle(pixels: &mut [u32]) {
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
    pixels: &mut [u32],
    width: usize,
    height: usize,
    cx: usize,
    cy: usize,
    r: usize,
    color: u32,
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

fn chess(pixels: &mut [u32]) {
    let cols = 8;
    let rows = 8;
    let cell_height = HEIGHT / cols;
    let cell_width = WIDTH / rows;

    for y in 0..cols {
        for x in 0..rows {
            fill_rect(pixels, WIDTH, HEIGHT, x * cell_width, y * cell_height, cell_width, cell_height, 
                if (x + y) % 2 == 0 { 0xff202020 } else { 0xffefefef }
            )
        }
    }

    save_to_ppm(pixels, "chess.ppm");
}

fn checker(pixels: &mut [u32]) {
    fill(pixels, 0xff202020);

    let cols = 8;
    let rows = 6;
    let cell_width = WIDTH / cols;
    let cell_height = HEIGHT / rows;

    for y in 0..rows {
        for x in 0..cols {
            let color = if (x + y) % 2 == 0 {
                0xffff2020
            } else {
                0xff202020
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

fn fill(pixels: &mut [u32], color: u32) {
    for i in 0..pixels.len() {
        pixels[i] = color;
    }
}

fn fill_rect(
    pixels: &mut [u32],
    width: usize,
    height: usize,
    x0: usize,
    y0: usize,
    w: usize,
    h: usize,
    color: u32,
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

fn save_to_ppm(pixels: &[u32], path: &str) {
    let mut file = File::create(path).map_err(error).unwrap();

    writeln!(file, "P6\n{} {} 255\n", WIDTH, HEIGHT)
        .map_err(error)
        .unwrap();

    for i in 0..WIDTH * HEIGHT {
        let pixel = pixels[i];

        let bytes: [u8; 3] = [
            ((pixel >> (8 * 0)) & 0xFF) as u8,
            ((pixel >> (8 * 1)) & 0xFF) as u8,
            ((pixel >> (8 * 2)) & 0xFF) as u8,
        ];


        file.write(&bytes).map_err(error).unwrap();
    }

    println!("generated '{}'", path);
}
