use zitoun::consts::*;
use zitoun::shapes::{Circle, Point};
use zitoun::Img;

fn main() {
    let mut img = Img::new(WIDTH, HEIGHT);

    img.fill(GRAY);

    let cols = 8;
    let rows = 8;
    let cell_width = WIDTH / rows;
    let cell_height = HEIGHT / cols;

    for y in 0..cols {
        for x in 0..rows {
            let t = (x as f32 / rows as f32 + y as f32 / cols as f32) / 2.0;
            let r = cell_height.min(cell_width) / 2;
            let r = lerp(r / 4, r, t);

            img.circle(
                Circle {
                    center: Point {
                        x: x * cell_width + cell_width / 2,
                        y: y * cell_height + cell_height / 2,
                    },
                    radius: r as usize,
                },
                if (x + y) % 2 == 0 { RED } else { GREEN },
            );
        }
    }

    img.save_to_ppm("circle.ppm");
}

fn lerp(min: usize, max: usize, t: f32) -> f32 {
    min as f32 + (max - min) as f32 * t
}
