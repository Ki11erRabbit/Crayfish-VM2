use crate::stack::StackChunk;
use crate::value::tuple::Tuple;
use crate::value::Value;

pub struct TupleChunk {
    pub data: Tuple,
}


impl StackChunk for TupleChunk {
    fn get_value(self) -> Value {
        Value::Tuple(self.data)
    }
}