use raster::{Color, Image};

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color {
        let r: u8 = rand::random_range(0..=255);
        let g: u8 = rand::random_range(0..=255);
        let b: u8 = rand::random_range(0..=255);
        Color { r, g, b, a:255 }
    }
}

#[derive(Clone)]
pub struct Point(i32, i32);

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }
    pub fn random(width: i32, height: i32) -> Self {
        let x: i32 = rand::random_range(1..=width);
        let y: i32 = rand::random_range(1..=height);
        Self(x, y)
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        let x = self.0;
        let y = self.1;
        image.display(x, y, self.color());
    }
}

pub struct Line(Point, Point);

impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Self(p1.clone(), p2.clone())
    }
    // Generate a random line within the bounds of the given width and height
    pub fn random(width: i32, height: i32) -> Self {
        let x = rand::random_range(1..=width);
        let y = rand::random_range(1..=height);
        let x2 = rand::random_range(x..=width);
        let y2 = rand::random_range(y..=height);
        Line(Point::new(x, y), Point::new(x2, y2))
    }
    fn draw_color(&self, image: &mut Image, color: &Color) {
        let x1 = self.0 .0;
        let y1 = self.0 .1;
        let x2 = self.1 .0;
        let y2 = self.1 .1;

        let dx = x2 - x1;
        let dy = y2 - y1;
        let step = dx.abs().max(dy.abs());

        let x_incr: f64 = dx as f64 / step as f64;
        let y_incr: f64 = dy as f64 / step as f64;

        let mut x = x1 as f64;
        let mut y = y1 as f64;

        for _ in 0..step {
            image.display(x.round() as i32, y.round() as i32, color.clone());
            x += x_incr;
            y += y_incr;
        }
    }
}

impl Drawable for Line {
    // Bresenham's Line Algorithm to draw the line
    fn draw(&self, image: &mut Image) {
        let color = self.color();
        self.draw_color(image, &color);
    }
}

pub struct Rectangle(Point, Point);
impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Self(p1.clone(), p2.clone())
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let x1 = self.0 .0;
        let y1 = self.0 .1;
        let x2 = self.1 .0;
        let y2 = self.1 .1;
        let color = self.color();

        Line::new(&Point::new(x1, y1), &Point::new(x2, y1)).draw_color(image, &color);
        Line::new(&Point::new(x1, y2), &Point::new(x2, y2)).draw_color(image, &color);
        Line::new(&Point::new(x1, y1), &Point::new(x1, y2)).draw_color(image, &color);
        Line::new(&Point::new(x2, y1), &Point::new(x2, y2)).draw_color(image, &color);
    }
}

pub struct Triangle(Point, Point, Point);
impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
        Self(p1.clone(), p2.clone(), p3.clone())
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let color = self.color();
        Line::new(&self.0, &self.1).draw_color(image, &color);
        Line::new(&self.1, &self.2).draw_color(image, &color);
        Line::new(&self.2, &self.0).draw_color(image, &color);
    }
}

pub struct Circle {
    pub center: Point,
    pub radius: i32,
}

impl Circle {
    pub fn new(center: Point, radius: i32) -> Self {
        Self {
            center: center,
            radius,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let center = Point::random(width, height);
        let radius = rand::random_range(1..width);
        Self::new(center, radius)
    }
}

fn draw_circle(image: &mut Image, cx: i32, cy: i32, radius: i32, color: &Color) {
    let mut x = radius;
    let mut y = 0;
    let mut err = 0;

    while x >= y {
        image.display(cx + x, cy + y, color.clone());
        image.display(cx + y, cy + x, color.clone());
        image.display(cx - y, cy + x, color.clone());
        image.display(cx - x, cy + y, color.clone());
        image.display(cx - x, cy - y, color.clone());
        image.display(cx - y, cy - x, color.clone());
        image.display(cx + y, cy - x, color.clone());
        image.display(cx + x, cy - y, color.clone());

        y += 1;
        if err <= 0 {
            err += 2 * y + 1;
        } else {
            x -= 1;
            err += 2 * (y - x) + 1;
        }
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let color = self.color();
        draw_circle(image, self.center.0, self.center.1, self.radius, &color);
    }
}