use std::f64::consts::PI;

trait PolygonProperties {
    fn perimeter(&self) -> f64;
    fn area(&self) -> f64;
    fn radius(&self) -> f64;
    fn apothem(&self) -> f64;
    // You can add other method prototypes here if needed
}

struct Polygon {
    sides_lengths: Vec<f64>,
    number_of_sides: f64,
}

impl Polygon {
    pub fn new(sides_lengths: Vec<f64>, number_of_sides: f64) -> Self {
        Self { sides_lengths, number_of_sides }
    }
}

impl PolygonProperties for Polygon {
    fn perimeter(&self) -> f64 {
        self.sides_lengths.iter().sum()
    }

    fn area(&self) -> f64 {
        let n = self.number_of_sides;
        let s = self.sides_lengths[0]; // Assuming all sides are of equal length
        (n * s * s) / (4.0 * (PI / n).tan())
    }

    fn radius(&self) -> f64 {
        let n = self.number_of_sides;
        let s = self.sides_lengths[0]; // Assuming all sides are of equal length
        s / (2.0 * (PI / n).sin())
    }

    fn apothem(&self) -> f64 {
        let n = self.number_of_sides;
        let s = self.sides_lengths[0]; // Assuming all sides are of equal length
        s / (2.0 * (PI / n).tan())
    }
}

fn main() {
    let polygon = Polygon::new(vec![30.0, 30.0, 30.0], 3.0); // Each side is 30 units, and it's a triangle
    println!("The perimeter of the polygon is: {}", polygon.perimeter());
    println!("The area of the polygon is: {}", polygon.area());
    println!("The radius of the circumcircle of the polygon is: {}", polygon.radius());
    println!("The apothem of the polygon is: {}", polygon.apothem());
}
