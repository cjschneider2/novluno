use crate::entity::entry::Entry;

#[derive(Debug, Clone)]
pub struct ListItem {
    pub name: String,
    pub id: u32,
    pub entry: Entry, // Entry { File number, File Index }
}
