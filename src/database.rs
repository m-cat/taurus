use std::collections::HashMap;
use util::{int, uint};

/// Database of game information.
pub struct Database {
    subtrees: Option<HashMap<String, Database>>,
    field: Option<Value>,
}

impl Database {
    /// Returns a new Database.
    pub fn new() -> Database {
        Database {
            subtrees: None,
            field: None,
        }
    }

    /// Returns a subtree of the Database.
    pub fn sub(&mut self, name: &str) -> &mut Database {
        match self.subtrees {
            None => self.subtrees = Some(HashMap::new()),
            _ => {}
        }

        self.subtrees
            .as_mut()
            .unwrap()
            .entry(String::from(name))
            .or_insert(Database::new())
    }

    /// Adds a new uint entry to the Database.
    ///
    /// # Examples
    /// See get_uint.
    pub fn set_uint(&mut self, value: uint) {
        self.field = Some(Value::Uint(value));
    }

    /// Gets the value associated with the Database subtree as a uint.
    ///
    /// # Panics
    /// If the associated value doesn't exist or is not a uint.
    ///
    /// # Examples
    /// ```
    /// use taurus::database::Database;
    ///
    /// let mut db = Database::new();
    /// db.sub("Actor").sub("Player").sub("hp").set_uint(15);
    /// assert_eq!(db.sub("Actor").sub("Player").sub("hp").get_uint(), 15);
    /// ```
    pub fn get_uint(&self) -> uint {
        match self.field.clone() {
            Some(some) => {
                match some {
                    Value::Uint(value) => value,
                    _ => panic!("Database::get_uint failed: value is not a uint."),
                }
            }
            None => panic!("Database::get_uint failed: no value found."),
        }
    }

    pub fn set_str(&mut self, value: &str) {}
    pub fn get_str(&self) {}
}

#[derive(Clone)]
pub enum Value {
    Int(int),
    Uint(uint),
    Str(String),
}

pub fn init_game(database: &mut Database) {
    init_actors(database);
}

fn init_actors(database: &mut Database) {}
