use std::fmt::{Debug, Display};
use fxhash::FxHashMap;
use crate::value::Value;

pub struct Environment {
    hashmap: FxHashMap<Box<str>,Value>
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            hashmap: FxHashMap::default()
        }
    }

    pub fn insert(&mut self, key: Box<str>, value: Value) {
        self.hashmap.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.hashmap.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.hashmap.get_mut(key)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.hashmap.contains_key(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<Value> {
        self.hashmap.remove(key)
    }

    pub fn len(&self) -> usize {
        self.hashmap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hashmap.is_empty()
    }
}

impl Default for Environment {
    fn default() -> Self {
        Environment::new()
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut env = String::new();
        for (key, value) in self.hashmap.iter() {
            env.push_str(&format!("{}: {}, \n", key, value));
        }
        write!(f, "{}", env)
    }
}

impl Debug for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut env = String::new();
        for (key, value) in self.hashmap.iter() {
            env.push_str(&format!("{}: {:?}, \n", key, value));
        }
        write!(f, "{}", env)
    }
}