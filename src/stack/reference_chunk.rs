use crate::stack::StackChunk;
use crate::value::{Reference, Value};





impl StackChunk for Reference {
    fn get_value(self) -> Value {
        Value::Reference(self)
    }

    fn get_boxed_value(self: Box<Self>) -> Value {
        Value::Reference(*self)
    }

    fn into_chunk(self) -> Box<dyn StackChunk> {
        Box::new(self)
    }
}