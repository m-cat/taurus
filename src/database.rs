#![allow(unused_imports)] // will complain about num_traits::Zero otherwise

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;
use std::slice::Iter;
use fraction::Fraction;
use num_traits::Zero;

use util::{int, uint};

/// Database of game information.
#[derive(Default)]
pub struct Database {
    subtrees: Option<HashMap<String, Database>>,
    fields: Option<Vec<Value>>,
}

impl Database {
    /// Returns a new Database.
    pub fn new() -> Database {
        Database {
            subtrees: None,
            fields: None,
        }
    }

    /// Returns a mutable reference to a subtree of the Database,
    /// creating it if it doesn't exist.
    pub fn sub(&mut self, name: &str) -> &mut Database {
        if self.subtrees.is_none() {
            self.subtrees = Some(HashMap::new());
        }

        self.subtrees
            .as_mut()
            .unwrap() // guaranteed not to be None here
            .entry(String::from(name))
            .or_insert_with(Database::new)
    }

    /// Returns a reference to a subtree of the Database, asserting that it exists.
    ///
    /// # Panics
    /// If the subtree doesn't exist.
    pub fn get(&self, name: &str) -> &Database {
        self.subtrees
            .as_ref()
            .expect("Database::get failed: map not initialized.")
            .get(&String::from(name))
            .expect("Database::get failed: subtree not found.")
    }

    /// Returns the number of fields for this Database (children not included).
    ///
    /// # Panics
    /// If no fields have been set.
    pub fn num(&self) -> usize {
        let vec = self.fields.as_ref().expect(
            "Database::num failed: vector not instantiated.",
        );
        vec.len()
    }

    /// Adds a new uint entry to the Database.
    ///
    /// # Examples
    /// See get_uint.
    pub fn add_uint(&mut self, value: uint) {
        match self.fields {
            None => self.fields = Some(vec![Value::Uint(value)]),
            Some(ref mut fields) => fields.push(Value::Uint(value)),
        }
    }

    // TODO
    pub fn add_char(&mut self, value: char) {
        match self.fields {
            None => self.fields = Some(vec![Value::Char(value)]),
            Some(ref mut fields) => fields.push(Value::Char(value)),
        }
    }

    /// Adds a new &str entry to the Database.
    ///
    /// # Examples
    /// See get_uint.
    pub fn add_str(&mut self, value: &'static str) {
        match self.fields {
            None => self.fields = Some(vec![Value::Str(value)]),
            Some(ref mut fields) => fields.push(Value::Str(value)),
        }
    }

    /// Adds a new Fraction entry to the Database.
    ///
    /// # Examples
    /// See get_uint.
    pub fn add_fraction(&mut self, value: Fraction) {
        match self.fields {
            None => self.fields = Some(vec![Value::Fraction(value)]),
            Some(ref mut fields) => fields.push(Value::Fraction(value)),
        }
    }

    /// Gets the value associated with the Database subtree as a uint.
    /// Returns the first value stored in the subtree.
    /// For more than one, use an iterator.
    ///
    /// # Panics
    /// If the associated value doesn't exist or is not a uint.
    ///
    /// # Examples
    /// ```
    /// use taurus::database::Database;
    ///
    /// let mut db = Database::new();
    /// db.sub("Actor").sub("Player").sub("hp").add_uint(15);
    /// assert_eq!(db.sub("Actor").sub("Player").sub("hp").get_uint(), 15);
    /// ```
    pub fn get_uint(&self) -> uint {
        match self.fields.as_ref() {
            Some(ref mut vec) => {
                // Return the first field in the vector.
                match vec[0] {
                    Value::Uint(value) => value,
                    _ => panic!("Database::get_uint failed: value is not a uint."),
                }
            }
            None => panic!("Database::get_uint failed: no value found."),
        }
    }

    /// Gets the value associated with the Database subtree as a &str.
    /// Returns the first value stored in the subtree.
    /// For more than one, use an iterator.
    ///
    /// # Panics
    /// If the associated value doesn't exist or is not a &str.
    ///
    /// # Examples
    /// See get_uint.
    pub fn get_str(&self) -> &'static str {
        match self.fields.as_ref() {
            Some(ref mut vec) => {
                match vec[0] {
                    Value::Str(value) => value,
                    _ => panic!("Database::get_str failed: value is not a str."),
                }
            }
            None => panic!("Database::get_str failed: no value found."),
        }
    }

    /// Gets the value associated with the Database subtree as a char.
    /// Returns the first value stored in the subtree.
    /// For more than one, use an iterator.
    ///
    /// # Panics
    /// If the associated value doesn't exist or is not a char.
    ///
    /// # Examples
    /// See get_uint.
    pub fn get_char(&self) -> char {
        match self.fields.as_ref() {
            Some(ref mut vec) => {
                match vec[0] {
                    Value::Char(value) => value,
                    _ => panic!("Database::get_char failed: value is not a char."),
                }
            }
            None => panic!("Database::get_char failed: no value found."),
        }
    }

    /// Gets the value associated with the Database subtree as a Fraction.
    /// Returns the first value stored in the subtree.
    /// For more than one, use an iterator.
    ///
    /// # Panics
    /// If the associated value doesn't exist or is not a Fraction.
    ///
    /// # Examples
    /// See get_uint.
    pub fn get_fraction(&self) -> Fraction {
        match self.fields.as_ref() {
            Some(ref mut vec) => {
                match vec[0] {
                    Value::Fraction(value) => value,
                    _ => panic!("Database::get_fraction failed: value is not a Fraction."),
                }
            }
            None => panic!("Database::get_fraction failed: no value found."),
        }
    }

    /// Returns an iterator over the values associated with this Database subtree.
    ///
    /// # Panics
    /// If no values exist for this Database.
    pub fn iter(&self) -> Iter<Value> {
        self.fields
            .as_ref()
            .expect("Database::iter failed: no values found.")
            .iter()
    }

    // TODO
    pub fn load_database(&mut self, file: File) {
        let reader = BufReader::new(file);

        for line in reader.lines() {}
    }
}

/// Enum of possible types that can be stored in the database.
/// Supported types include int, uint, char, String, Fraction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Int(int),
    Uint(uint),
    Char(char),
    Str(&'static str),
    Fraction(Fraction),
}

#[cfg(test)]
mod tests {
    use database::*;

    #[test]
    fn test_get_uint() {
        let mut db = Database::new();
        db.sub("Test").add_uint(4);
        assert_eq!(db.get("Test").get_uint(), 4);
    }

    #[test]
    fn test_get_char() {
        let mut db = Database::new();
        db.sub("Test").add_char('c');
        assert_eq!(db.get("Test").get_char(), 'c');
    }

    #[test]
    fn test_get_str() {
        let mut db = Database::new();
        db.sub("Test").add_str("testing");
        assert_eq!(db.get("Test").get_str(), "testing");
    }

    #[test]
    fn test_get_fraction() {
        let mut db = Database::new();
        db.sub("Test").add_fraction(Fraction::zero());
        assert_eq!(db.get("Test").get_fraction(), Fraction::zero());
    }

    #[test]
    fn test_iter() {
        let mut db = Database::new();
        db.sub("Test").sub("Fields").add_uint(0);
        db.sub("Test").sub("Fields").add_uint(1);

        for (i, n) in db.get("Test").get("Fields").iter().enumerate() {
            if let &Value::Uint(u) = n {
                assert_eq!(u, i as uint);
            }
        }
    }
}
