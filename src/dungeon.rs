//! Taurus - dungeon.rs
//! Copyright (C) 2017  Marcin Swieczkowski <scatman@bu.edu>

use std::collections::HashMap;

use taurus::coord::Coord;
use actor::Actor;

pub struct Dungeon {
    tile_grid: Vec<Vec<Tile>>,

    actor_list: Vec<Actor>,
    actor_map: HashMap<&Coord, &Actor>,
    actor_queue: BinaryHeap<&Actor>,

    object_list: Vec<Object>,
    object_map: HashMap<&Coord, &Object>,

    stack_list: Vec<ItemStack>,
    stack_map: HashMap<&Coord, &ItemStack>,
}

impl Dungeon {
    pub fn new() {
        Dungeon { actor_map: HashMap::new() }
    }
}
