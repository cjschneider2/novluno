use crate::utility::pixel::Pixel;
use crate::entity::entry::Entry;
use crate::entity::sprite_type::SpriteType;

#[derive(Debug)]
pub struct Sprite {
    pub class: SpriteType,
    pub rle_entry: Entry,
    pub x_dim: i32,
    pub y_dim: i32,
    pub x_off: i32,
    pub y_off: i32,
    // pub image_raw: Vec<Pixel>,
    pub image_raw: Vec<u8>, // defined to be in RGB_565_norm format
}

