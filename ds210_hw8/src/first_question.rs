use std::ops::Neg;
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> 
where
    T: Copy + Neg<Output = T>,
{
    fn clockwise(&self) -> Point<T> {
        let x_90 = self.y.neg();
        let y_90 = self.x;
        Point { x: x_90, y: y_90 }
    }
    fn counterclockwise(&self) -> Point<T> {
        let x_270 = self.y;
        let y_270 = self.x.neg();
        Point { x: x_270, y: y_270 }
    }
}
macro_rules! assert_eq {
    ($left:expr, $right:expr $(,)?) => {
        if $left.x as f64 != $right.x as f64 || $left.y as f64 != $right.y as f64 {
            panic!("Assertion failed");
        }
    };
}
fn main() {
    let mut point_0 = Point {x: 3, y: 5}; //i32
    let mut point_1 = Point {x: 3.00, y: 5.00}; //f64
    println!("{:?}", point_0.clockwise());
    println!("{:?}", point_1.counterclockwise());
    assert_eq!(point_0, point_1); // Test to see if 2 points are same even if they are of different types
}

