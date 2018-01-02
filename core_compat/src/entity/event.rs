#[derive(Debug)]
pub struct Event {
    pub number: u16,
    pub left: u32,   // c1_x
    pub top: u32,    // c1_x
    pub right: u32,  // c2_x
    pub bottom: u32, // c2_y
}
