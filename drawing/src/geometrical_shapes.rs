extern crate bresenham;
use bresenham::Bresenham;
use rand::Rng;
use raster::{Image, Color};

trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

// Point
// struct Point(i32, i32);
// impl Point {
//     fn random(x: i32, y: i32) -> Self {
//         Self(x, y)
//     }

//     fn draw(self, image) {
//         image.despl
//     }
// }

// Line
struct Line(Point, Point);
impl Line {
    pub fn random(width: i32, height: i32) -> self {
        let x = rand::thread_rng().gen_range(1..=width);
        let y = rand::thread_rng().gen_range(1..=height);
        let x2 = rand::thread_rng().gen_range(x..=width);
        let y2 = rand::thread_rng().gen_range(y..=height);
        
        self (
            Point::new(x, y),
            Point::new(x2, y2)
        )
    }
    //Bresenham’s Line Generation Algorithm
    pub fn draw(&self, image: &Image) {
        let x = self.0.0;
        let y = self.0.1;

        let x2 = self.1.0;
        let y2 = self.1.1;

         for (x, y) in Bresenham::new((x, y), (x2, y2)) {
            display(image, x, y, Color{r: 200, g: 0, b: 150, a: 150})
        }
    }
}