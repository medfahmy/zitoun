use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::process::exit;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

fn main() {
    let mut pixels = [0; WIDTH * HEIGHT];

    // checker(&mut pixels);
    circle(&mut pixels);
}

fn circle(pixels: &mut [u32]) {
    fill(pixels, 0xff202020);
    fill_circle(
        pixels,
        WIDTH,
        HEIGHT,
        400,
        400,
        200,
        0xff2020ff,
    );
    save_to_ppm(&pixels, "circle.ppm");
}

fn fill_circle(
    pixels: &mut [u32],
    width: usize,
    height: usize,
    cx: i64,
    cy: i64,
    r: i64,
    color: u32,
) {
    let x1 = cx - r;
    let x2 = cx + r;
    let y1 = cy - r;
    let y2 = cy + r;

    for y in y1..y2 {
        if y < height as i64 {
            for x in x1..x2 {
                if x < width as i64 {
                    let dx = if x > cx { x - cx } else { cx - x };
                    let dy = if y > cy { y - cy } else { cy - y };
                    if (dx * dx + dy * dy) <= r.pow(2) {
                        pixels[y as usize * width + x as usize] = color;
                    }
                }
            }
        }
    }
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
    let mut output: [u8; WIDTH * HEIGHT * 3] = [0; WIDTH * HEIGHT * 3];

    writeln!(file, "P6\n{} {} 255\n", WIDTH, HEIGHT)
        .map_err(error)
        .unwrap();

    for i in 0..WIDTH * HEIGHT {
        let pixel = pixels[i];

        // 0xAABBGGRR
        let r = ((pixel >> (8 * 0)) & 0xFF).try_into().unwrap();
        let g = ((pixel >> (8 * 1)) & 0xFF).try_into().unwrap();
        let b = ((pixel >> (8 * 2)) & 0xFF).try_into().unwrap();

        output[i * 3] = r;
        output[i * 3 + 1] = g;
        output[i * 3 + 2] = b;
    }

    file.write_all(&output).map_err(error).unwrap();
    println!("generated '{}'", path);
}
