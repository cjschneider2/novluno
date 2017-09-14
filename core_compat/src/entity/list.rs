
use entity::list_item::ListItem;

pub struct List {
    pub items: Vec<ListItem>,
}

impl List {
    pub fn new() -> List {
        List {
            items: Vec::new()
        }
    }
}
