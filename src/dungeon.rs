//! Taurus - dungeon.rs
//! Copyright (C) 2017  Marcin Swieczkowski <scatman@bu.edu>

use std::collections::HashMap;
use std::collections::BinaryHeap;

use taurus::coord::Coord;
use tile::Tile;
use actor::Actor;
use object::Object;
use item::ItemStack;

pub struct Dungeon<'a> {
    tile_grid: Vec<Vec<Tile>>,

    actor_list: Vec<Actor>,
    actor_map: HashMap<&'a Coord, &'a Actor>,
    actor_queue: BinaryHeap<&'a Actor>,

    object_list: Vec<Object>,
    object_map: HashMap<&'a Coord, &'a Object>,

    stack_list: Vec<ItemStack>,
    stack_map: HashMap<&'a Coord, &'a ItemStack>,
}

impl<'a> Dungeon<'a> {
    pub fn new() {
        Dungeon { actor_map: HashMap::new() }
    }
}
