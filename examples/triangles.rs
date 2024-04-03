use zitoun::Img;
use zitoun::shapes::{Point, Triangle};
use zitoun::consts::*;

fn main() {
    let mut img = Img::new(WIDTH, HEIGHT);

    let unit = WIDTH / 8;

    let tri1 = Triangle {
        a: Point { x: unit * 4, y: unit * 1 },
        b: Point { x: unit * 1, y: unit * 4 },
        c: Point { x: unit * 4, y: unit * 4 },
    };

    let tri2 = Triangle {
        a: Point { x: unit * 4, y: unit * 1 },
        b: Point { x: unit * 7, y: unit * 4 },
        c: Point { x: unit * 4, y: unit * 4 },
    };

    let tri3 = Triangle {
        a: Point { x: unit * 4, y: unit * 7 },
        b: Point { x: unit * 7, y: unit * 4 },
        c: Point { x: unit * 4, y: unit * 4 },
    };

    let tri4 = Triangle {
        a: Point { x: unit * 1, y: unit * 4 },
        b: Point { x: unit * 4, y: unit * 7 },
        c: Point { x: unit * 4, y: unit * 4 },
    };

    img.triangle(tri1, RED);
    img.triangle(tri2, GREEN);
    img.triangle(tri3, BLUE);
    img.triangle(tri4, YELLOW);

    img.save_to_ppm("triangles.ppm");
}