use entity::rmd_type::RmdType;
use entity::rmd_animation::RmdAnimation;
use entity::rmd_entry::RmdEntry;

#[derive(Debug)]
pub struct Rmd {
    kind: RmdType,
    string: String,
    animation_parts: i32,
    // for object animations
    animation_entry_count: i32,
    entry_count: i32,
    entries: Vec<RmdEntry>,
    animation_count: i32,
    animations: Vec<RmdAnimation>,
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

    pub fn set_animation_count(&mut self, count: i32) {
        self.animation_count = count;
    }

    pub fn animation_count(&self) -> i32 {
        self.animation_count
    }

    pub fn set_animation_parts(&mut self, parts: i32) {
        self.animation_parts = parts;
    }

    pub fn set_animation_entry_count(&mut self, value: i32) {
        self.animation_entry_count = value;
    }

    pub fn add_entry(&mut self, entry: RmdEntry) {
        self.entries.push(entry);
    }

    pub fn get_entry(&self, index: usize) -> Option<&RmdEntry> {
        self.entries.get(index)
    }

    pub fn set_entry_count(&mut self, value: i32) {
        self.entry_count = value;
    }

    pub fn entry_count(&self) -> i32 {
        self.entry_count
    }
}
