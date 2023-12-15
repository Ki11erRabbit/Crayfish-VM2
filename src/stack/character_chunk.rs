use crate::stack::StackChunk;
use crate::value::Value;




impl StackChunk for char {
    fn get_value(self) -> Value {
        Value::Character(self)
    }

    fn get_boxed_value(self: Box<Self>) -> Value {
        Value::Character(*self)
    }

    fn into_chunk(self) -> Box<dyn StackChunk> {
        Box::new(self)
    }
}