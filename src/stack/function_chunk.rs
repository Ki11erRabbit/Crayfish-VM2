use crate::stack::StackChunk;
use crate::value::function::Function;
use crate::value::Value;



impl StackChunk for Function {
    fn get_value(self) -> Value {
        Value::Function(self)
    }

    fn get_boxed_value(self: Box<Self>) -> Value {
        Value::Function(*self)
    }

    fn into_chunk(self) -> Box<dyn StackChunk> {
        Box::new(self)
    }
}