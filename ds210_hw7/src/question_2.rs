use std::f64::consts::PI; // Import PI for Area, Radius, and Apothem Calculation
extern crate rand;
use rand::{thread_rng, Rng}; // Useful for Random f64 Values 

trait PolygonProperties {
    fn perimeter(&self) -> f64;
    fn area(&self) -> f64;
    fn radius(&self) -> f64;
    fn apothem(&self) -> f64;
}

struct Polygon { // Initial struct for the Polygon
    side_length: f64, // Since These Are Normal Polygons We Only Need 1 Input and Duplicate for All Other Sides
    number_of_sides: f64,
}

impl Polygon {
    pub fn new(side_length: f64, number_of_sides: f64) -> Self { // Creating the Initial Polygon 
        Self { side_length, number_of_sides }
    }
}

impl PolygonProperties for Polygon { // Functions Responsible for Geometery
    fn perimeter(&self) -> f64 { // Perimeter Function
        self.side_length * self.number_of_sides
    }

    fn area(&self) -> f64 { // Area Function
        let n = self.number_of_sides;
        let s = self.side_length;
        (n * s * s) / (4.0 * (PI / n).tan()) 
    }

    fn radius(&self) -> f64 {
        let n = self.number_of_sides;
        let s = self.side_length;
        s / (2.0 * (PI / n).sin())
    }

    fn apothem(&self) -> f64 {
        let n = self.number_of_sides;
        let s = self.side_length;
        s / (2.0 * (PI / n).tan())
    }
}

fn main() {
    let shapes: [f64; 9] = [6.0, 12.0, 24.0, 128.0, 256.0, 512.0, 1024.0, 2048.0, 65536.0];
    let mut rng = thread_rng();
    for &sl in &shapes {
        let random_value: f64 = rng.gen_range(1.0, 100.0); // Create polygon with equal side lengths of random size
        let test_polygon = Polygon::new(random_value, sl);
        let circle_radius_area: f64 = PI * test_polygon.radius().powf(2.0);
        let circle_apoth_area: f64 = PI * test_polygon.apothem().powf(2.0);
        if circle_radius_area > test_polygon.area() {
            println!("The polygon with {} sides has an area less than the area of a circle with the same radius", test_polygon.number_of_sides);
        } else {
            println!("The polygon with {} sides has an area greater than the area of a circle with the same radius", test_polygon.number_of_sides);
        }
        
        // Second comparison
        if circle_apoth_area > test_polygon.area() {
            println!("The polygon with {} sides has an area less than the area of a circle with a radius equal to the polygon's apothem", test_polygon.number_of_sides);
        } else {
            println!("The polygon with {} sides has an area greater than the area of a circle with a radius equal to the polygon's apothem", test_polygon.number_of_sides);
        }
    }
}
