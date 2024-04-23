use std::ops::Neg;
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T:Copy + Neg<Output = T>> Point<T> 
//T Requires the Negative Trait and Copy Trait for Code to Properly Output Rotation
{
    fn clockwise(&self) -> Point<T> { // Formula is xcosΘ - 
        let x_90 = self.y.neg(); // Formula is x_90 = xcosΘ - ysinΘ where Θ == 90 (Since cos(90) and sin(90) == 1, We Just Negate Y)
        let y_90 = self.x; // Formula is y_90 = ycosΘ + xsinΘ Θ == 90 (See Explanation above)
        Point { x: x_90, y: y_90 } // Returns New Point
    }
    fn counterclockwise(&self) -> Point<T> {
        let x_270 = self.y; // Formula is x_270 = xcosΘ - ysinΘ where Θ == 270 or -90 (Cos(270) == 0 and Sin(270) == -1, Simplified to Y)
        let y_270 = self.x.neg(); // Formula is y_90 = ycosΘ + xsinΘ where Θ == 270 or -90 (Simplifies to -X)
        Point { x: x_270, y: y_270 } // Returns New Point
    }
}
macro_rules! assert_eq {
    ($left:expr, $right:expr $(,)?) => {
        if $left.x as f64 != $right.x as f64 || $left.y as f64 != $right.y as f64 {
            panic!("Assertion failed"); // Converts Both Point {x as f64, y as f64} and Compares the Two Points
        }
    };
}
fn main() {
    let point_0 = Point {x: 3, y: 5}; //i32
    let point_1 = Point {x: 3.00, y: 5.00}; //f64
    println!("{:?}", point_0.clockwise());
    println!("{:?}", point_1.counterclockwise());
    assert_eq!(point_0, point_1); // Test to see if 2 points are same even if they are of different types
}

