use crate::entity::rmd_type::RmdType;
use crate::entity::rmd_animation::RmdAnimation;
use crate::entity::rmd_entry::RmdEntry;

#[derive(Debug)]
pub struct Rmd {
    pub kind: RmdType,
    pub string: String,
    pub animation_parts: i32,
    // for object animations
    pub animation_entry_count: i32,
    pub entry_count: i32,
    pub entries: Vec<RmdEntry>,
    pub animation_count: i32,
    pub animations: Vec<RmdAnimation>,
}

impl Rmd {
    pub fn new(kind: RmdType) -> Rmd {
        Rmd {
            kind,
            string: String::new(),
            animation_parts: 0,
            animation_entry_count: 0,
            entry_count: 0,
            entries: Vec::new(),
            animation_count: 0,
            animations: Vec::new(),
        }
    }

    pub fn add_animation(&mut self, ani: RmdAnimation) {
        self.animations.push(ani);
    }

    pub fn add_entry(&mut self, entry: RmdEntry) {
        self.entries.push(entry);
    }

    pub fn get_entry(&self, index: usize) -> Option<&RmdEntry> {
        self.entries.get(index)
    }
}
