use rand::Rng;
use raster::{Image, Color};

// Displayable Trait
pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

// Point struct
pub struct Point(i32, i32);

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }
}

// Line struct
pub struct Line(Point, Point);

impl Line {
    // Generate a random line within the bounds of the given width and height
    pub fn random(width: i32, height: i32) -> Self {
        let x = rand::thread_rng().gen_range(1..=width);
        let y = rand::thread_rng().gen_range(1..=height);
        let x2 = rand::thread_rng().gen_range(x..=width);
        let y2 = rand::thread_rng().gen_range(y..=height);
        
        Line(Point::new(x, y), Point::new(x2, y2))
    }

    // Bresenham's Line Algorithm to draw the line
    pub fn draw(self, image: &mut Image) {
        let x1 = self.0 .0;
        let y1 = self.0 .1;
        let x2 = self.1 .0;
        let y2 = self.1 .1;

        // Bresenham's Line Algorithm: 
        let dx = (x2 - x1).abs(); // Difference in x coordinates
        let dy = (y2 - y1).abs(); // Difference in y coordinates
        let mut sx = if x1 < x2 { 1 } else { -1 }; // Direction of x-axis movement
        let sy = if y1 < y2 { 1 } else { -1 }; // Direction of y-axis movement
        let mut err = dx - dy; // Error term

        let mut x = x1;
        let mut y = y1;

        loop {
            // Display the current point
            image.display(x, y, Color { r: 200, g: 0, b: 150, a: 150 });

            // Check if we've reached the end point
            if x == x2 && y == y2 {
                break;
            }

            // Calculate the error term
            let e2 = 2 * err;

            // Update x-coordinate
            if e2 > -dy {
                err -= dy;
                x += sx;
            }

            // Update y-coordinate
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }
}
