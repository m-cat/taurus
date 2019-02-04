//! Module containing Item structs.

use crate::console::Color;
use crate::ui::Draw;

#[derive(Clone, Debug)]
pub struct Item {
    c: char,
    color: Color,
}

impl Draw for Item {
    fn draw_c(&self) -> char {
        self.c
    }

    fn draw_color(&self) -> Color {
        self.color
    }
}

#[derive(Clone, Debug)]
pub struct ItemStack {
    item: Item,
    amount: usize,
}

impl Draw for ItemStack {
    fn draw_c(&self) -> char {
        self.item.draw_c()
    }

    fn draw_color(&self) -> Color {
        self.item.draw_color()
    }
}

#[derive(Clone, Debug, Default)]
pub struct ItemStash {
    items: Vec<ItemStack>,
}

impl ItemStash {
    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, i: ItemStack) {
        self.items.push(i);
    }

    pub fn remove(&mut self, index: usize) -> ItemStack {
        self.items.remove(index)
    }

    pub fn top(&self) -> &ItemStack {
        &self.items[self.items.len() - 1]
    }
}

impl Draw for ItemStash {
    fn draw_c(&self) -> char {
        self.top().draw_c()
    }

    fn draw_color(&self) -> Color {
        self.top().draw_color()
    }
}
