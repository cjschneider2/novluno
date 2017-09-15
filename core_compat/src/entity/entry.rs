#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Entry {
    file: u32,
    index: u32,
}

impl Entry {
    pub fn new(file: u32, index: u32) -> Entry {
        Entry { file, index }
    }

    pub fn file(&self) -> u32 {
        self.file
    }

    pub fn index(&self) -> u32 {
        self.index
    }
}
