#[derive(Debug)] // Provides default ways to implement Shape
enum Shape {
    Triangle(f64, f64, f64), // Lengths of sides
    Rectangle(f64, f64),     // Length of sides
    Circle(f64),             // Radius
}
impl Shape {
    // Calculate Area
    fn is_valid(&self) -> bool {
        match self {
            Shape::Triangle(a, b, c) => {
                // Check if all sides are positive and satisfy the triangle inequality
                *a > 0.0 && *b > 0.0 && *c > 0.0 && *a + *b > *c && *a + *c > *b && *b + *c > *a
            }
            Shape::Rectangle(a, b) => *a > 0.0 && *b > 0.0, // Check if both sides are positive
            Shape::Circle(r) => *r > 0.0, // Check if radius is positive
        }
    }
    fn area(&self) -> f64 { 
        match self { 
            Shape::Triangle(a, b, c) => {
                let s = (*a + *b + *c) / 2.0; // Solving for S - The First Step of Triangle Area
                (s * (s - *a) * (s - *b) * (s - *c)).sqrt() // Remaining Equation for the Area of a Triangle Knowing 3 Sides
            }
            Shape::Rectangle(a, b) => a * b, // Area of a Rectangle
            Shape::Circle(r) => std::f64::consts::PI * r * r, // Area of a Circle
        }
    }
    fn perimeter(&self) -> f64 {
        match self {
            Shape::Triangle(a, b, c) => a + b + c, // Calculates Perimeter of Triangle
            Shape::Rectangle(a, b) => 2.0 * (a + b), // Calculates Perimeter of Rectangle
            Shape::Circle(r) => 2.0 * std::f64::consts::PI * *r, // Calculates Perimeter of Circle Using π
        }
    }
    fn double(&self) -> Shape {
        match self { // Returns a New Shape With Double the Perimeter
            Shape::Triangle(a, b, c) => Shape::Triangle(2.0 * a, 2.0 * b, 2.0 * c), 
            Shape::Rectangle(a, b) => Shape::Rectangle(2.0 * a, 2.0 * b),
            Shape::Circle(r) => Shape::Circle(2.0 * r),
        }
    }

}
fn main() {
    let triangle = Shape::Triangle(3.0, 3.0, 4.0);
    let rectangle = Shape::Rectangle(4.0, 5.0);
    let circle = Shape::Circle(8.0);
    println!("The area of a triangle is {}", triangle.area());
    println!("The perimeter of a triangle is {}", triangle.perimeter());
    println!("The shape with double the perimeter of the original triangle is a {:?}", triangle.double());
    println!("The validity of the parameters of the triangle is {:?}", triangle.is_valid());

    println!("The area of a rectangle is {}", rectangle.area());
    println!("The perimeter of a rectangle is {}", rectangle.perimeter());
    println!("The shape with double the perimeter of the original rectangle is a {:?}", rectangle.double());
    println!("The validity of the parameters of the rectangle is {:?}", rectangle.is_valid());

    println!("The area of a circle is {}", circle.area());
    println!("The perimeter of a circle is {}", circle.perimeter());
    println!("The shape with double the perimeter of the original circle is a {:?}", circle.double());
    println!("The validity of the parameters of the circle is {:?}", circle.is_valid());

}
