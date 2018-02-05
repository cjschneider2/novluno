use std::ops::Add;

use point::Point;
use size::Size;

#[derive(Debug)]
pub struct Rectangle<T> {
    pub location: Point<T>,
    pub size: Size<T>,
}

impl <T> Rectangle<T> where
    T: PartialOrd + Add<Output = T> + Copy
{
    pub fn new(location: Point<T>, size: Size<T>) -> Rectangle<T> {
        Rectangle { location, size }
    }

    pub fn new_from_points(location: (T, T), size: (T, T)) -> Rectangle<T> {
        Rectangle {
            location: Point { x: location.0, y: location.1 },
            size: Size { width: size.0, height: size.1 }
        }
    }

    pub fn contains_point(&self, point: &Point<T>) -> bool {
        if point.x > self.location.x && point.y > self.location.y {
            let f: T = self.location.x + self.size.width;
            if point.x < f {
                return true;
            }
        }
        false
    }
}