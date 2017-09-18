#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl <T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}