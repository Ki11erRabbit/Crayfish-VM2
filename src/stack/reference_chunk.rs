use crate::stack::StackChunk;
use crate::value::{Reference, Value};

pub struct ReferenceChunk {
    pub data: Reference,
}


impl StackChunk for ReferenceChunk {
    fn get_value(self) -> Value {
        Value::Reference(self.data)
    }
}