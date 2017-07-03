use entry::Entry;
use vec::Vec3;
use sprite_type::SpriteType;

pub struct Sprite {
    class: SpriteType,
    entry: Entry,
    x_dim: usize,
    y_dim: usize,
    image: Vec<Vec3<u8>>,
}
