use utility::pixel::Pixel;
use entity::entry::Entry;
use entity::sprite_type::SpriteType;

#[derive(Debug)]
pub struct Sprite {
    pub class: SpriteType,
    pub entry: Entry,
    pub x_dim: usize,
    pub y_dim: usize,
    pub x_off: usize,
    pub y_off: usize,
    pub image: Vec<Pixel>,
}
