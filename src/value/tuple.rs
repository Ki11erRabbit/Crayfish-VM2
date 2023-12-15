use std::fmt::{Debug, Display};
use crate::stack::StackChunk;
use crate::value::Value;

#[derive(Clone)]
pub struct Tuple {
    pub data: Box<[Value]>
}

impl Tuple {

    pub fn new(data: Box<[Value]>) -> Tuple {
        Tuple {
            data,
        }
    }

    pub fn empty() -> Tuple {
        Tuple {
            data: Box::new([]),
        }
    }

    pub fn get(&self, index: usize) -> &Value {
        &self.data[index]
    }

    pub fn into_chunk(self) -> Box<dyn StackChunk> {
        Box::new(self)
    }
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