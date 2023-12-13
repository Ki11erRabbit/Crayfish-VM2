use crate::stack::StackChunk;
use crate::value::function::Function;
use crate::value::Value;

pub struct FunctionChunk {
    pub data: Function,
}

impl StackChunk for FunctionChunk {
    fn get_value(self) -> Value {
        Value::Function(self.data)
    }
}