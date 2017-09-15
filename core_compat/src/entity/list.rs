
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

    pub fn get_item(&self, index: usize) -> Option<&ListItem> {
        self.items.get(index)
    }
}
