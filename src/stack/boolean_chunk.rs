use crate::stack::StackChunk;
use crate::value::Value;

pub struct BooleanChunk {
    pub data: bool,
}

impl StackChunk for BooleanChunk {
    fn get_value(self) -> Value {
        Value::Boolean(self.data)
    }
}