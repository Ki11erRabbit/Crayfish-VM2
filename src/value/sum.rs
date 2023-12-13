use std::collections::HashMap;
use std::fmt::{Debug, Display};
use crate::value::Value;

#[derive(Clone)]
pub struct SumType {
    name: Box<str>,
    tag: u8,
    fields: HashMap<Box<str>, Value>
}


impl Display for SumType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fields = String::new();
        for (name, value) in self.fields.iter() {
            fields.push_str(&format!("{}: {}, ", name, value));
        }
        write!(f, "{}({})", self.name, fields)
    }
}

impl Debug for SumType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fields = String::new();
        for (name, value) in self.fields.iter() {
            fields.push_str(&format!("{}: {:?}, ", name, value));
        }
        write!(f, "{}({})", self.name, fields)
    }
}