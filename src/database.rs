use std::collections::HashMap;
use std::slice::Iter;
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
        let vec = self.fields.as_ref().expect("Database::num failed: vector not instantiated.");
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

    /// Adds a new &str entry to the Database.
    ///
    /// # Examples
    /// See get_str.
    pub fn add_str(&mut self, value: &'static str) {
        match self.fields {
            None => self.fields = Some(vec![Value::Str(value)]),
            Some(ref mut fields) => fields.push(Value::Str(value)),
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

    // TODO
    pub fn get_char(&self) -> char {
        'c'
    }

    /// Returns an iterator over the values associated with this Database subtree.
    ///
    /// # Panics
    /// If no values exist for this Database.
    pub fn iter(&self) -> Iter<Value> {
        self.fields.as_ref().expect("Database::iter failed: no values found.")
            .iter()
    }
}

// // Implementing IntoIterator allows us to use Rust's for-loop syntax for iterating
// // over the fields of a database.
// impl IntoIterator for Database {
//     type Item = Value;
//     type IntoIter = ::std::vec::IntoIter<Value>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.fields.expect("Database::into_iter failed: no fields initialized.").into_iter()
//     }
// }

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Int(int),
    Uint(uint),
    Str(&'static str),
}

// impl PartialEq for Value {
//     fn eq(&self, other: &Self) {
//         match

#[cfg(test)]
mod tests {
    use database::*;

    #[test]
    fn test_iter() {
        let mut db = Database::new();
        db.sub("Test").sub("Fields").add_uint(0);
        db.sub("Test").sub("Fields").add_uint(1);

        for (i, n) in db.sub("Test").sub("Fields").iter().enumerate() {
            if let &Value::Uint(u) = n {
                assert_eq!(u, i as uint);
            }
        }
    }
}
