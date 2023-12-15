use crate::stack::StackChunk;
use crate::value::tuple::Tuple;
use crate::value::Value;



impl StackChunk for Tuple {
    fn get_value(self) -> Value {
        Value::Tuple(self)
    }

    fn get_boxed_value(self: Box<Self>) -> Value {
        Value::Tuple(*self)
    }

    fn into_chunk(self) -> Box<dyn StackChunk> {
        Box::new(self)
    }
}