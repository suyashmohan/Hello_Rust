use std::error::Error;
use std::ops::Mul;

trait Area<T> {
    fn area(self) -> T
    where
        T: Mul<Output = T>;
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

impl<T> Area<T> for Point<T> {
    fn area(self) -> T
    where
        T: Mul<Output = T>,
    {
        return self.x * self.y;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let point = Point::new(10.0, 20.0);
    println!("Area is {}", point.area());
    Ok(())
}
