use crate::stack::StackChunk;
use crate::value::product::ProductType;
use crate::value::Value;

pub struct ProductChunk {
    pub data: ProductType,
}


impl StackChunk for ProductChunk {
    fn get_value(self) -> Value {
        Value::Product(self.data)
    }
}