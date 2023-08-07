#![deny(missing_docs)]
//! Implementation of an in-memory key-value store
//!
//! For now, this only supports storing keys and values as `String`

use std::collections::HashMap;

/// An in-memory key-value store
///
/// Stores `String`-`String` pairs
pub struct KvStore {
    store: HashMap<String, String>,
}

impl Default for KvStore {
    fn default() -> Self {
        KvStore::new()
    }
}

impl KvStore {
    /// generate an empty `KvStore` object
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kvs::KvStore;
    ///
    /// let kvs = KvStore::new();
    /// let empty = kvs.get("hello".to_owned());
    ///
    /// assert!(empty.is_none());
    /// ```
    pub fn new() -> Self {
        KvStore {
            store: HashMap::new(),
        }
    }

    /// set a key-value pair
    /// Takes two `String` values as arguments
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kvs::KvStore;
    ///
    /// let mut kvs = KvStore::new();
    /// kvs.set("hello".to_owned(), "world".to_owned());
    ///
    /// assert_eq!(kvs.get("hello".to_owned()).unwrap(), "world".to_owned());
    /// ```
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    /// get a key-value pair
    /// Takes a `String` and returns `Option<String>`
    /// Returns `Some(value)` if it exists, or `None` otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kvs::KvStore;
    ///
    /// let mut kvs = KvStore::new();
    /// kvs.set("hello".to_owned(), "world".to_owned());
    ///
    /// let world = kvs.get("hello".to_owned()).unwrap();
    /// assert_eq!(world, "world".to_owned());
    /// ```
    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).cloned()
    }

    /// removes a key-value pair
    /// if the key does not exist, does nothing
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kvs::KvStore;
    ///
    /// let mut kvs = KvStore::new();
    /// kvs.set("hello".to_owned(), "world".to_owned());
    ///
    /// let world = kvs.get("hello".to_owned()).unwrap();
    /// assert_eq!(world, "world".to_owned());
    ///
    /// kvs.remove("hello".to_owned());
    /// let none = kvs.get("hello".to_owned());
    /// assert!(none.is_none());
    /// ```
    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}
