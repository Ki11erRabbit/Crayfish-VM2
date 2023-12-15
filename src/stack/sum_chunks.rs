use crate::stack::StackChunk;
use crate::value::sum::SumType;
use crate::value::Value;



impl StackChunk for SumType {
    fn get_value(self) -> Value {
        Value::Sum(self)
    }

    fn get_boxed_value(self: Box<Self>) -> Value {
        Value::Sum(*self)
    }

    fn into_chunk(self) -> Box<dyn StackChunk> {
        Box::new(self)
    }
}