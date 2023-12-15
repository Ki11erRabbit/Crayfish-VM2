use crate::stack::StackChunk;
use crate::value::product::ProductType;
use crate::value::Value;



impl StackChunk for ProductType {
    fn get_value(self) -> Value {
        Value::Product(self)
    }

    fn get_boxed_value(self: Box<Self>) -> Value {
        Value::Product(*self)
    }

    fn into_chunk(self) -> Box<dyn StackChunk> {
        Box::new(self)
    }
}