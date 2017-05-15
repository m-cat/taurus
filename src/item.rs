#![allow(dead_code)]

pub struct Item {}

#[derive(Default)]
pub struct ItemStack {
    items: Vec<Item>,
}

impl ItemStack {
    pub fn new() -> ItemStack {
        ItemStack { items: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, i: Item) {
        self.items.push(i);
    }

    pub fn remove(&mut self, index: usize) -> Item {
        self.items.remove(index)
    }
}
