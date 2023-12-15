use crate::stack::StackChunk;
use crate::value::Value;



impl StackChunk for bool {

    fn get_value(self) -> Value {
        Value::Boolean(self)
    }

    fn get_boxed_value(self: Box<Self>) -> Value {
        Value::Boolean(*self)
    }

    fn into_chunk(self) -> Box<dyn StackChunk> {
        Box::new(self)
    }
}