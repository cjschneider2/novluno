use utility::point::Point;
use utility::size::Size;

#[derive(Debug)]
pub struct Rectangle<T> {
    pub location: Point<T>,
    pub size: Size<T>,
}

impl <T> Rectangle<T> {
    pub fn new(location: Point<T>, size: Size<T>) -> Rectangle<T> {
        Rectangle { location, size }
    }
}