#![allow(unused)]

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    // Associated function (static method): constructor
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    // Instance method: needs &mut self to modify
    fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    // Instance method: read-only access using &self
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    // Create using associated function
    let mut p = Point::new(0.0, 0.0);
    println!("initial: {:?}, dist={}", p, p.distance_from_origin());

    // Call instance method
    p.move_to(3.0, 4.0);
    println!("moved:   {:?}, dist={}", p, p.distance_from_origin());
}
