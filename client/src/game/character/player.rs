
pub struct Player {
    pub position: (u32, u32),
    pub kind: u32,
    pub list_id: u32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: (100, 100),
            kind: 0,
            list_id: 0,
        }
    }
}
