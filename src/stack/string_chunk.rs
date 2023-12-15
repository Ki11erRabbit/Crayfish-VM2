use crate::stack::StackChunk;
use crate::value::Value;

impl StackChunk for String {
    fn get_value(self) -> Value {
        Value::String(self)
    }

    fn get_boxed_value(self: Box<Self>) -> Value {
        Value::String(*self)
    }

    fn into_chunk(self) -> Box<dyn StackChunk> {
        Box::new(self)
    }
}