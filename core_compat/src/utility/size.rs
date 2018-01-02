#[derive(Debug)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

impl <T> Size<T> {
    pub fn new(width: T, height: T) -> Size<T> {
        Size { width, height }
    }
}