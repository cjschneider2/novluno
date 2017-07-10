//! RMD files are sort of data pointers for the Objects and 
//! Tiles and include things like which file they are in and
//! if they are a part of an animation or so.

pub struct RmdItem {
    number: u32,
    type: u32,
    x_start: u32,
    y_start: u32,
    x_end: u32,
    y_end: u32,
    entry_number: u32,
}

pub struct RmdFile {
    number: u32,
    count: u32,
    items: Vec<RmdItem>,
}