use entity::resource::Resource;

pub struct ResourceFile {
    pub name: String,
    pub file_number: u32,
    pub resources: Vec<Resource>,
}

impl ResourceFile {
    pub fn new() -> ResourceFile {
        ResourceFile {
            name: String::new(),
            file_number: 0,
            resources: Vec::new(),
        }
    }
}
