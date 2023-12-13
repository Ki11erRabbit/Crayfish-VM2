use crate::stack::StackChunk;
use crate::value::sum::SumType;
use crate::value::Value;

pub struct SumChunk {
    pub data: SumType,
}


impl StackChunk for SumChunk {
    fn get_value(self) -> Value {
        Value::Sum(self.data)
    }
}