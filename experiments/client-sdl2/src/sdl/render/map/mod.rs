mod tiles;
mod objects;

/// Helper function to move to the next map tile in order.
/// TODO: Probably fold into `map` entity iterator?
fn next_tile(x: &mut i32, y: &mut i32, stride: i32) {
    *x += 1;
    if *x >= stride {
        *x = 0;
        *y += 1;
    }
}

pub use self::tiles::tiles;
pub use self::objects::objects;
