use crate::stack::StackChunk;
use crate::value::Value;

impl StackChunk for String {
    fn get_value(self) -> Value {
        Value::String(self)
    }

    fn get_boxed_value(self: Box<Self>) -> Value {
        Value::String(*self)
    }
}