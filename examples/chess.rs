use zitoun::consts::*;
use zitoun::shapes::{Point, Rect};
use zitoun::Img;

fn main() {
    let mut img = Img::new(WIDTH, HEIGHT);

    let cols = 8;
    let rows = 8;
    let cell_height = HEIGHT / cols;
    let cell_width = WIDTH / rows;

    for y in 0..cols {
        for x in 0..rows {
            img.rect(
                Rect {
                    edge: Point {
                        x: x * cell_width,
                        y: y * cell_height,
                    },
                    width: cell_width,
                    height: cell_height,
                },
                if (x + y) % 2 == 0 { BLACK } else { WHITE },
            )
        }
    }

    img.save_to_ppm("chess.ppm");
}
