use std::fmt::{Debug, Display};
use crate::value::Value;

#[derive(Clone)]
pub struct Tuple {
    pub data: Box<[Value]>
}



impl Display for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fields = String::new();
        for value in self.data.iter() {
            fields.push_str(&format!("{}, ", value));
        }
        write!(f, "({})", fields)
    }
}

impl Debug for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fields = String::new();
        for value in self.data.iter() {
            fields.push_str(&format!("{:?}, ", value));
        }
        write!(f, "({})", fields)
    }
}